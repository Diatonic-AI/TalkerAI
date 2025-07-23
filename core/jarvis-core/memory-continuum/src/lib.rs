use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, Semaphore};
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

pub mod short_term;
pub mod long_term;
pub mod procedural;
pub mod episodic;
pub mod spatial;
pub mod consolidation;
pub mod retrieval;
pub mod graph;

pub use short_term::ShortTermMemory;
pub use long_term::LongTermMemory;
pub use procedural::ProceduralMemory;
pub use episodic::EpisodicMemory;
pub use spatial::SpatialMemory;
pub use consolidation::MemoryConsolidation;
pub use retrieval::MemoryRetrieval;

/// Multi-layer memory continuum that orchestrates all memory types
#[derive(Debug)]
pub struct MemoryContinuum {
    pub stm: Arc<ShortTermMemory>,
    pub ltm: Arc<LongTermMemory>,
    pub procedural: Arc<ProceduralMemory>,
    pub episodic: Arc<EpisodicMemory>,
    pub spatial: Arc<SpatialMemory>,
    pub consolidation: Arc<MemoryConsolidation>,
    pub retrieval: Arc<MemoryRetrieval>,
    
    // Memory management
    active_memories: Arc<DashMap<Uuid, ActiveMemory>>,
    memory_graph: Arc<RwLock<graph::MemoryGraph>>,
    consolidation_scheduler: Arc<tokio::sync::Mutex<ConsolidationScheduler>>,
    
    // Configuration
    config: MemoryConfig,
}

/// Active memory tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveMemory {
    pub id: Uuid,
    pub memory_type: MemoryType,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u64,
    pub importance_score: f64,
    pub associations: Vec<Uuid>,
}

/// Memory types in the continuum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryType {
    ShortTerm,
    LongTerm,
    Procedural,
    Episodic,
    Spatial,
}

/// Memory item with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub id: Uuid,
    pub content: serde_json::Value,
    pub memory_type: MemoryType,
    pub encoding: MemoryEncoding,
    pub metadata: MemoryMetadata,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
}

/// Memory encoding formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryEncoding {
    Text(String),
    Vector(Vec<f32>),
    Graph(graph::GraphNode),
    Procedural(procedural::Procedure),
    Episode(episodic::Episode),
    Spatial(spatial::SpatialData),
}

/// Memory metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadata {
    pub importance: f64,
    pub confidence: f64,
    pub source: String,
    pub tags: Vec<String>,
    pub associations: Vec<Uuid>,
    pub consolidation_level: u8,
    pub access_pattern: AccessPattern,
}

/// Memory access patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    pub frequency: f64,
    pub recency: f64,
    pub context_relevance: f64,
    pub emotional_valence: f64,
}

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub stm_capacity: usize,
    pub ltm_capacity: Option<usize>, // None = unlimited
    pub consolidation_threshold: f64,
    pub forgetting_curve_factor: f64,
    pub importance_decay_rate: f64,
    pub max_associations: usize,
    pub spatial_resolution: f64,
    pub episodic_compression_ratio: f64,
}

/// Consolidation scheduler for memory management
#[derive(Debug)]
struct ConsolidationScheduler {
    pending_consolidations: Vec<ConsolidationTask>,
    last_consolidation: Instant,
    consolidation_interval: Duration,
}

#[derive(Debug, Clone)]
struct ConsolidationTask {
    memory_id: Uuid,
    scheduled_at: Instant,
    priority: f64,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            stm_capacity: 1000,
            ltm_capacity: None,
            consolidation_threshold: 0.7,
            forgetting_curve_factor: 0.1,
            importance_decay_rate: 0.05,
            max_associations: 50,
            spatial_resolution: 1.0,
            episodic_compression_ratio: 0.3,
        }
    }
}

impl MemoryContinuum {
    /// Create a new memory continuum
    pub async fn new(config: MemoryConfig) -> Result<Self> {
        info!("🧠 Initializing JARVIS Memory Continuum");
        
        let stm = Arc::new(ShortTermMemory::new(config.stm_capacity).await?);
        let ltm = Arc::new(LongTermMemory::new().await?);
        let procedural = Arc::new(ProceduralMemory::new().await?);
        let episodic = Arc::new(EpisodicMemory::new().await?);
        let spatial = Arc::new(SpatialMemory::new(config.spatial_resolution).await?);
        
        let memory_graph = Arc::new(RwLock::new(graph::MemoryGraph::new()));
        
        let consolidation = Arc::new(MemoryConsolidation::new(
            Arc::clone(&stm),
            Arc::clone(&ltm),
            Arc::clone(&memory_graph),
            config.consolidation_threshold,
        ));
        
        let retrieval = Arc::new(MemoryRetrieval::new(
            Arc::clone(&stm),
            Arc::clone(&ltm),
            Arc::clone(&procedural),
            Arc::clone(&episodic),
            Arc::clone(&spatial),
            Arc::clone(&memory_graph),
        ));
        
        let consolidation_scheduler = Arc::new(tokio::sync::Mutex::new(ConsolidationScheduler {
            pending_consolidations: Vec::new(),
            last_consolidation: Instant::now(),
            consolidation_interval: Duration::from_secs(300), // 5 minutes
        }));

        Ok(Self {
            stm,
            ltm,
            procedural,
            episodic,
            spatial,
            consolidation,
            retrieval,
            active_memories: Arc::new(DashMap::new()),
            memory_graph,
            consolidation_scheduler,
            config,
        })
    }

    /// Store a memory item in the appropriate memory system
    #[instrument(skip(self, content))]
    pub async fn store_memory(
        &self,
        content: serde_json::Value,
        memory_type: MemoryType,
        metadata: MemoryMetadata,
    ) -> Result<Uuid> {
        let memory_id = Uuid::new_v4();
        let now = Utc::now();
        
        debug!("Storing memory {} in {:?}", memory_id, memory_type);

        // Create memory item
        let memory_item = MemoryItem {
            id: memory_id,
            content: content.clone(),
            memory_type: memory_type.clone(),
            encoding: self.encode_memory(&content, &memory_type).await?,
            metadata: metadata.clone(),
            created_at: now,
            last_accessed: now,
        };

        // Store in appropriate memory system
        match memory_type {
            MemoryType::ShortTerm => {
                self.stm.store(memory_item).await?;
            },
            MemoryType::LongTerm => {
                self.ltm.store(memory_item).await?;
            },
            MemoryType::Procedural => {
                if let MemoryEncoding::Procedural(procedure) = &memory_item.encoding {
                    self.procedural.store_procedure(procedure.clone()).await?;
                }
            },
            MemoryType::Episodic => {
                if let MemoryEncoding::Episode(episode) = &memory_item.encoding {
                    self.episodic.store_episode(episode.clone()).await?;
                }
            },
            MemoryType::Spatial => {
                if let MemoryEncoding::Spatial(spatial_data) = &memory_item.encoding {
                    self.spatial.store_spatial_data(spatial_data.clone()).await?;
                }
            },
        }

        // Track active memory
        let active_memory = ActiveMemory {
            id: memory_id,
            memory_type,
            created_at: now,
            last_accessed: now,
            access_count: 1,
            importance_score: metadata.importance,
            associations: metadata.associations.clone(),
        };
        
        self.active_memories.insert(memory_id, active_memory);

        // Update memory graph
        {
            let mut graph = self.memory_graph.write().await;
            graph.add_memory_node(memory_id, &metadata).await?;
            
            // Create associations
            for associated_id in &metadata.associations {
                graph.add_association(memory_id, *associated_id, 1.0).await?;
            }
        }

        // Schedule consolidation if needed
        if metadata.importance > self.config.consolidation_threshold {
            self.schedule_consolidation(memory_id, metadata.importance).await;
        }

        info!("Memory {} stored successfully", memory_id);
        Ok(memory_id)
    }

    /// Retrieve memories based on query
    #[instrument(skip(self))]
    pub async fn retrieve_memories(
        &self,
        query: &str,
        memory_types: Vec<MemoryType>,
        limit: usize,
    ) -> Result<Vec<MemoryItem>> {
        debug!("Retrieving memories for query: {}", query);
        
        let memories = self.retrieval.retrieve(query, memory_types, limit).await?;
        
        // Update access patterns
        for memory in &memories {
            self.update_access_pattern(memory.id).await;
        }
        
        debug!("Retrieved {} memories", memories.len());
        Ok(memories)
    }

    /// Get memory associations
    pub async fn get_associations(&self, memory_id: Uuid) -> Result<Vec<Uuid>> {
        let graph = self.memory_graph.read().await;
        Ok(graph.get_associations(memory_id).await?)
    }

    /// Update memory importance
    pub async fn update_importance(&self, memory_id: Uuid, new_importance: f64) -> Result<()> {
        if let Some(mut active_memory) = self.active_memories.get_mut(&memory_id) {
            active_memory.importance_score = new_importance;
            
            // Schedule consolidation if importance increased significantly
            if new_importance > self.config.consolidation_threshold {
                self.schedule_consolidation(memory_id, new_importance).await;
            }
        }
        
        Ok(())
    }

    /// Get memory statistics
    pub async fn get_statistics(&self) -> Result<MemoryStatistics> {
        let stm_count = self.stm.count().await?;
        let ltm_count = self.ltm.count().await?;
        let procedural_count = self.procedural.count().await?;
        let episodic_count = self.episodic.count().await?;
        let spatial_count = self.spatial.count().await?;
        
        let graph = self.memory_graph.read().await;
        let associations_count = graph.association_count().await?;
        
        Ok(MemoryStatistics {
            total_memories: stm_count + ltm_count + procedural_count + episodic_count + spatial_count,
            short_term_count: stm_count,
            long_term_count: ltm_count,
            procedural_count,
            episodic_count,
            spatial_count,
            associations_count,
            active_memories_count: self.active_memories.len(),
        })
    }

    /// Run consolidation process
    pub async fn run_consolidation(&self) -> Result<ConsolidationResult> {
        info!("🔄 Running memory consolidation");
        
        let result = self.consolidation.consolidate().await?;
        
        // Update consolidation scheduler
        {
            let mut scheduler = self.consolidation_scheduler.lock().await;
            scheduler.last_consolidation = Instant::now();
            scheduler.pending_consolidations.clear();
        }
        
        info!("Consolidation completed: {} memories processed", result.processed_count);
        Ok(result)
    }

    /// Encode memory content based on type
    async fn encode_memory(&self, content: &serde_json::Value, memory_type: &MemoryType) -> Result<MemoryEncoding> {
        match memory_type {
            MemoryType::ShortTerm | MemoryType::LongTerm => {
                if let Some(text) = content.as_str() {
                    Ok(MemoryEncoding::Text(text.to_string()))
                } else {
                    Ok(MemoryEncoding::Text(content.to_string()))
                }
            },
            MemoryType::Procedural => {
                // Convert content to procedure format
                let procedure = procedural::Procedure::from_json(content)?;
                Ok(MemoryEncoding::Procedural(procedure))
            },
            MemoryType::Episodic => {
                // Convert content to episode format
                let episode = episodic::Episode::from_json(content)?;
                Ok(MemoryEncoding::Episode(episode))
            },
            MemoryType::Spatial => {
                // Convert content to spatial data format
                let spatial_data = spatial::SpatialData::from_json(content)?;
                Ok(MemoryEncoding::Spatial(spatial_data))
            },
        }
    }

    /// Schedule memory consolidation
    async fn schedule_consolidation(&self, memory_id: Uuid, priority: f64) {
        let mut scheduler = self.consolidation_scheduler.lock().await;
        
        let task = ConsolidationTask {
            memory_id,
            scheduled_at: Instant::now() + Duration::from_secs(60), // 1 minute delay
            priority,
        };
        
        scheduler.pending_consolidations.push(task);
        scheduler.pending_consolidations.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
    }

    /// Update memory access pattern
    async fn update_access_pattern(&self, memory_id: Uuid) {
        if let Some(mut active_memory) = self.active_memories.get_mut(&memory_id) {
            let now = Utc::now();
            active_memory.last_accessed = now;
            active_memory.access_count += 1;
            
            // Apply forgetting curve
            let time_since_creation = (now - active_memory.created_at).num_seconds() as f64;
            let decay = (-self.config.forgetting_curve_factor * time_since_creation).exp();
            active_memory.importance_score *= decay;
        }
    }
}

/// Memory statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryStatistics {
    pub total_memories: usize,
    pub short_term_count: usize,
    pub long_term_count: usize,
    pub procedural_count: usize,
    pub episodic_count: usize,
    pub spatial_count: usize,
    pub associations_count: usize,
    pub active_memories_count: usize,
}

/// Consolidation result
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsolidationResult {
    pub processed_count: usize,
    pub promoted_to_ltm: usize,
    pub associations_created: usize,
    pub memories_forgotten: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_continuum_creation() {
        let config = MemoryConfig::default();
        let continuum = MemoryContinuum::new(config).await;
        assert!(continuum.is_ok());
    }

    #[tokio::test]
    async fn test_memory_storage_and_retrieval() {
        let config = MemoryConfig::default();
        let continuum = MemoryContinuum::new(config).await.unwrap();

        let content = serde_json::json!({"text": "Test memory content"});
        let metadata = MemoryMetadata {
            importance: 0.8,
            confidence: 0.9,
            source: "test".to_string(),
            tags: vec!["test".to_string()],
            associations: vec![],
            consolidation_level: 0,
            access_pattern: AccessPattern {
                frequency: 1.0,
                recency: 1.0,
                context_relevance: 0.8,
                emotional_valence: 0.0,
            },
        };

        let memory_id = continuum.store_memory(
            content,
            MemoryType::ShortTerm,
            metadata,
        ).await.unwrap();

        let memories = continuum.retrieve_memories(
            "Test memory",
            vec![MemoryType::ShortTerm],
            10,
        ).await.unwrap();

        assert!(!memories.is_empty());
        assert_eq!(memories[0].id, memory_id);
    }
} 