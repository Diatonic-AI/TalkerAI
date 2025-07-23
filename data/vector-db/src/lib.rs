use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, error};
use uuid::Uuid;

/// Vector Database Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDbConfig {
    pub qdrant_url: String,
    pub qdrant_api_key: Option<String>,
    pub collection_name: String,
    pub vector_size: u64,
    pub distance_metric: DistanceMetric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    Dot,
}

/// Document for vector storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: Uuid,
    pub content: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub vector: Option<Vec<f32>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Search result with similarity score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document: VectorDocument,
    pub score: f32,
    pub rank: usize,
}

/// Vector database interface
#[async_trait]
pub trait VectorDatabase {
    async fn initialize(&mut self) -> Result<()>;
    async fn create_collection(&self, name: &str, vector_size: u64) -> Result<()>;
    async fn upsert_document(&self, document: VectorDocument) -> Result<()>;
    async fn upsert_documents(&self, documents: Vec<VectorDocument>) -> Result<()>;
    async fn search(&self, query_vector: Vec<f32>, limit: usize, filter: Option<HashMap<String, serde_json::Value>>) -> Result<Vec<SearchResult>>;
    async fn search_by_text(&self, query: &str, limit: usize, filter: Option<HashMap<String, serde_json::Value>>) -> Result<Vec<SearchResult>>;
    async fn delete_document(&self, id: Uuid) -> Result<()>;
    async fn get_document(&self, id: Uuid) -> Result<Option<VectorDocument>>;
    async fn get_collection_info(&self) -> Result<CollectionInfo>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub name: String,
    pub vector_size: u64,
    pub points_count: u64,
    pub indexed: bool,
}

/// Qdrant implementation of vector database
pub struct QdrantVectorDb {
    client: qdrant_client::client::QdrantClient,
    config: VectorDbConfig,
    embeddings: Box<dyn EmbeddingModel + Send + Sync>,
}

impl QdrantVectorDb {
    pub async fn new(config: VectorDbConfig) -> Result<Self> {
        let client = if let Some(api_key) = &config.qdrant_api_key {
            qdrant_client::client::QdrantClient::from_url(&config.qdrant_url)
                .with_api_key(api_key)
                .build()?
        } else {
            qdrant_client::client::QdrantClient::from_url(&config.qdrant_url).build()?
        };

        // Initialize embedding model
        let embeddings = Box::new(FastEmbedModel::new().await?);

        Ok(Self {
            client,
            config,
            embeddings,
        })
    }
}

#[async_trait]
impl VectorDatabase for QdrantVectorDb {
    async fn initialize(&mut self) -> Result<()> {
        info!("Initializing Qdrant vector database");
        
        // Check if collection exists, create if not
        let collections = self.client.list_collections().await?;
        let collection_exists = collections.collections
            .iter()
            .any(|c| c.name == self.config.collection_name);

        if !collection_exists {
            self.create_collection(&self.config.collection_name, self.config.vector_size).await?;
        }

        Ok(())
    }

    async fn create_collection(&self, name: &str, vector_size: u64) -> Result<()> {
        use qdrant_client::qdrant::{CreateCollection, VectorParams, VectorsConfig, Distance};
        
        let distance = match self.config.distance_metric {
            DistanceMetric::Cosine => Distance::Cosine,
            DistanceMetric::Euclidean => Distance::Euclid,
            DistanceMetric::Dot => Distance::Dot,
        };

        self.client.create_collection(&CreateCollection {
            collection_name: name.to_string(),
            vectors_config: Some(VectorsConfig {
                config: Some(qdrant_client::qdrant::vectors_config::Config::Params(VectorParams {
                    size: vector_size,
                    distance: distance.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        }).await?;

        info!("Created Qdrant collection: {}", name);
        Ok(())
    }

    async fn upsert_document(&self, mut document: VectorDocument) -> Result<()> {
        // Generate embedding if not provided
        if document.vector.is_none() {
            document.vector = Some(self.embeddings.embed(&document.content).await?);
        }

        let vector = document.vector.as_ref().unwrap();
        
        use qdrant_client::qdrant::{PointStruct, UpsertPoints};
        
        let point = PointStruct::new(
            document.id.to_string(),
            vector.clone(),
            document.metadata.clone(),
        );

        self.client.upsert_points(UpsertPoints {
            collection_name: self.config.collection_name.clone(),
            points: vec![point],
            ..Default::default()
        }).await?;

        Ok(())
    }

    async fn upsert_documents(&self, mut documents: Vec<VectorDocument>) -> Result<()> {
        // Generate embeddings for documents that don't have them
        for doc in &mut documents {
            if doc.vector.is_none() {
                doc.vector = Some(self.embeddings.embed(&doc.content).await?);
            }
        }

        use qdrant_client::qdrant::{PointStruct, UpsertPoints};
        
        let points: Vec<PointStruct> = documents
            .iter()
            .map(|doc| {
                PointStruct::new(
                    doc.id.to_string(),
                    doc.vector.as_ref().unwrap().clone(),
                    doc.metadata.clone(),
                )
            })
            .collect();

        self.client.upsert_points(UpsertPoints {
            collection_name: self.config.collection_name.clone(),
            points,
            ..Default::default()
        }).await?;

        Ok(())
    }

    async fn search(&self, query_vector: Vec<f32>, limit: usize, filter: Option<HashMap<String, serde_json::Value>>) -> Result<Vec<SearchResult>> {
        use qdrant_client::qdrant::{SearchPoints, Filter};
        
        let search_request = SearchPoints {
            collection_name: self.config.collection_name.clone(),
            vector: query_vector,
            limit: limit as u64,
            filter: filter.map(|f| self.build_filter(f)),
            with_payload: Some(true.into()),
            ..Default::default()
        };

        let response = self.client.search_points(&search_request).await?;
        
        let results = response.result
            .into_iter()
            .enumerate()
            .map(|(rank, point)| {
                let id = Uuid::parse_str(&point.id.unwrap().point_id_options.unwrap().to_string())
                    .unwrap_or_else(|_| Uuid::new_v4());
                
                let metadata: HashMap<String, serde_json::Value> = point.payload
                    .into_iter()
                    .map(|(k, v)| (k, serde_json::to_value(v).unwrap_or(serde_json::Value::Null)))
                    .collect();

                let content = metadata.get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                SearchResult {
                    document: VectorDocument {
                        id,
                        content,
                        metadata,
                        vector: None, // Don't return vectors in search results
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                    score: point.score,
                    rank,
                }
            })
            .collect();

        Ok(results)
    }

    async fn search_by_text(&self, query: &str, limit: usize, filter: Option<HashMap<String, serde_json::Value>>) -> Result<Vec<SearchResult>> {
        let query_vector = self.embeddings.embed(query).await?;
        self.search(query_vector, limit, filter).await
    }

    async fn delete_document(&self, id: Uuid) -> Result<()> {
        use qdrant_client::qdrant::{DeletePoints, PointsSelector, PointsIdsList, PointId};
        
        self.client.delete_points(&DeletePoints {
            collection_name: self.config.collection_name.clone(),
            points: Some(PointsSelector {
                points_selector_one_of: Some(
                    qdrant_client::qdrant::points_selector::PointsSelectorOneOf::Points(
                        PointsIdsList {
                            ids: vec![PointId {
                                point_id_options: Some(
                                    qdrant_client::qdrant::point_id::PointIdOptions::Uuid(id.to_string())
                                ),
                            }],
                        }
                    )
                ),
            }),
            ..Default::default()
        }).await?;

        Ok(())
    }

    async fn get_document(&self, id: Uuid) -> Result<Option<VectorDocument>> {
        use qdrant_client::qdrant::{GetPoints, PointsSelector, PointsIdsList, PointId};
        
        let response = self.client.get_points(&GetPoints {
            collection_name: self.config.collection_name.clone(),
            ids: Some(PointsSelector {
                points_selector_one_of: Some(
                    qdrant_client::qdrant::points_selector::PointsSelectorOneOf::Points(
                        PointsIdsList {
                            ids: vec![PointId {
                                point_id_options: Some(
                                    qdrant_client::qdrant::point_id::PointIdOptions::Uuid(id.to_string())
                                ),
                            }],
                        }
                    )
                ),
            }),
            with_payload: Some(true.into()),
            with_vectors: Some(true.into()),
        }).await?;

        if let Some(point) = response.result.first() {
            let metadata: HashMap<String, serde_json::Value> = point.payload
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(serde_json::Value::Null)))
                .collect();

            let content = metadata.get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let vector = point.vectors.as_ref()
                .and_then(|v| match v {
                    qdrant_client::qdrant::vectors::VectorsOptions::Vector(vec) => Some(vec.data.clone()),
                    _ => None,
                });

            Ok(Some(VectorDocument {
                id,
                content,
                metadata,
                vector,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_collection_info(&self) -> Result<CollectionInfo> {
        let info = self.client.collection_info(&self.config.collection_name).await?;
        
        Ok(CollectionInfo {
            name: self.config.collection_name.clone(),
            vector_size: info.result.unwrap().config.unwrap().params.unwrap().vectors.unwrap().size,
            points_count: info.result.unwrap().points_count.unwrap_or(0),
            indexed: true, // Simplified
        })
    }
}

impl QdrantVectorDb {
    fn build_filter(&self, filter: HashMap<String, serde_json::Value>) -> qdrant_client::qdrant::Filter {
        // Convert HashMap filter to Qdrant filter
        // This is a simplified implementation
        qdrant_client::qdrant::Filter::default()
    }
}

/// Embedding model interface
#[async_trait]
pub trait EmbeddingModel {
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    async fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>>;
    fn embedding_size(&self) -> usize;
}

/// FastEmbed implementation
pub struct FastEmbedModel {
    model: fastembed::TextEmbedding,
}

impl FastEmbedModel {
    pub async fn new() -> Result<Self> {
        use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
        
        let model = TextEmbedding::try_new(InitOptions {
            model_name: EmbeddingModel::BGESmallENV15,
            show_download_progress: true,
            ..Default::default()
        })?;

        Ok(Self { model })
    }
}

#[async_trait]
impl EmbeddingModel for FastEmbedModel {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.model.embed(vec![text], None)?;
        Ok(embeddings.into_iter().next().unwrap_or_default())
    }

    async fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        let embeddings = self.model.embed(texts, None)?;
        Ok(embeddings)
    }

    fn embedding_size(&self) -> usize {
        384 // BGE Small model embedding size
    }
}

/// RAG (Retrieval Augmented Generation) functionality
pub struct RagSystem {
    vector_db: Box<dyn VectorDatabase + Send + Sync>,
    chunk_size: usize,
    chunk_overlap: usize,
}

impl RagSystem {
    pub fn new(vector_db: Box<dyn VectorDatabase + Send + Sync>) -> Self {
        Self {
            vector_db,
            chunk_size: 1000,
            chunk_overlap: 200,
        }
    }

    /// Add document to RAG system with chunking
    pub async fn add_document(&self, content: &str, metadata: HashMap<String, serde_json::Value>) -> Result<Vec<Uuid>> {
        let chunks = self.chunk_text(content);
        let mut document_ids = Vec::new();

        for (i, chunk) in chunks.iter().enumerate() {
            let doc_id = Uuid::new_v4();
            let mut chunk_metadata = metadata.clone();
            chunk_metadata.insert("content".to_string(), serde_json::Value::String(chunk.clone()));
            chunk_metadata.insert("chunk_index".to_string(), serde_json::Value::Number(i.into()));
            chunk_metadata.insert("total_chunks".to_string(), serde_json::Value::Number(chunks.len().into()));

            let document = VectorDocument {
                id: doc_id,
                content: chunk.clone(),
                metadata: chunk_metadata,
                vector: None, // Will be generated during upsert
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            self.vector_db.upsert_document(document).await?;
            document_ids.push(doc_id);
        }

        Ok(document_ids)
    }

    /// Retrieve relevant context for a query
    pub async fn retrieve_context(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        self.vector_db.search_by_text(query, limit, None).await
    }

    /// Generate response with retrieved context
    pub async fn generate_with_context(&self, query: &str, context_limit: usize) -> Result<RagResponse> {
        let search_results = self.retrieve_context(query, context_limit).await?;
        
        let context = search_results
            .iter()
            .map(|result| result.document.content.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(RagResponse {
            query: query.to_string(),
            context,
            sources: search_results,
        })
    }

    fn chunk_text(&self, text: &str) -> Vec<String> {
        let mut chunks = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        
        let mut i = 0;
        while i < words.len() {
            let mut chunk_words = Vec::new();
            let mut current_size = 0;
            
            while i < words.len() && current_size < self.chunk_size {
                chunk_words.push(words[i]);
                current_size += words[i].len() + 1; // +1 for space
                i += 1;
            }
            
            chunks.push(chunk_words.join(" "));
            
            // Back up for overlap
            if i < words.len() {
                let overlap_words = std::cmp::min(self.chunk_overlap / 10, chunk_words.len());
                i -= overlap_words;
            }
        }
        
        chunks
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RagResponse {
    pub query: String,
    pub context: String,
    pub sources: Vec<SearchResult>,
} 