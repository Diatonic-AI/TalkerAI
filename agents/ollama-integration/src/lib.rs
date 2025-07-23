use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use uuid::Uuid;

/// Ollama Model Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub size: u64,
    pub digest: String,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    pub details: ModelDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Vec<String>,
    pub parameter_size: String,
    pub quantization_level: String,
}

/// Chat Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

/// Ollama Task Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaTaskConfig {
    pub id: Uuid,
    pub model_name: String,
    pub task_type: OllamaTaskType,
    pub parameters: OllamaParameters,
    pub context_window: usize,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OllamaTaskType {
    ChatCompletion,
    CodeGeneration,
    DataAnalysis,
    Research,
    Summarization,
    Translation,
    Embedding,
    CustomTask { name: String, prompt_template: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaParameters {
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub repeat_penalty: f32,
    pub seed: Option<i32>,
    pub num_predict: Option<i32>,
    pub num_ctx: Option<i32>,
}

impl Default for OllamaParameters {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            seed: None,
            num_predict: None,
            num_ctx: Some(2048),
        }
    }
}

/// Automated Task Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatedTask {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub trigger: TaskTrigger,
    pub actions: Vec<TaskAction>,
    pub schedule: Option<TaskSchedule>,
    pub enabled: bool,
    pub last_run: Option<chrono::DateTime<chrono::Utc>>,
    pub next_run: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskTrigger {
    Schedule(TaskSchedule),
    DataChange { source: String, pattern: String },
    ApiCall { endpoint: String },
    FileChange { path: String },
    Custom { condition: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskSchedule {
    Cron(String),
    Interval { seconds: u64 },
    Daily { hour: u8, minute: u8 },
    Weekly { day: u8, hour: u8, minute: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskAction {
    LlmQuery {
        model: String,
        prompt: String,
        store_result: bool,
    },
    DataExtraction {
        source: String,
        format: String,
    },
    ApiCall {
        url: String,
        method: String,
        headers: HashMap<String, String>,
        body: Option<String>,
    },
    FileOperation {
        operation: String,
        path: String,
        content: Option<String>,
    },
    Notification {
        channel: String,
        message: String,
    },
}

/// Ollama Integration Manager
pub struct OllamaManager {
    client: ollama_rs::Ollama,
    models: RwLock<HashMap<String, OllamaModel>>,
    tasks: RwLock<HashMap<Uuid, AutomatedTask>>,
    chat_sessions: RwLock<HashMap<Uuid, ChatSession>>,
    base_url: String,
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub id: Uuid,
    pub model_name: String,
    pub messages: Vec<ChatMessage>,
    pub parameters: OllamaParameters,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

impl OllamaManager {
    pub fn new(base_url: Option<String>) -> Self {
        let url = base_url.unwrap_or_else(|| "http://localhost:11434".to_string());
        let client = ollama_rs::Ollama::new(url.clone());
        
        Self {
            client,
            models: RwLock::new(HashMap::new()),
            tasks: RwLock::new(HashMap::new()),
            chat_sessions: RwLock::new(HashMap::new()),
            base_url: url,
        }
    }

    /// Initialize Ollama manager and discover available models
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing Ollama manager at {}", self.base_url);
        
        // Test connection
        match self.client.list_local_models().await {
            Ok(models) => {
                info!("Successfully connected to Ollama, found {} models", models.len());
                
                let mut model_map = self.models.write().await;
                for model in models {
                    let ollama_model = OllamaModel {
                        name: model.name.clone(),
                        size: model.size,
                        digest: model.digest,
                        modified_at: chrono::DateTime::from_timestamp(model.modified_at.timestamp(), 0)
                            .unwrap_or_else(chrono::Utc::now),
                        details: ModelDetails {
                            format: model.details.format,
                            family: model.details.family,
                            families: model.details.families.unwrap_or_default(),
                            parameter_size: model.details.parameter_size,
                            quantization_level: model.details.quantization_level,
                        },
                    };
                    model_map.insert(model.name, ollama_model);
                }
            }
            Err(e) => {
                error!("Failed to connect to Ollama: {}", e);
                return Err(anyhow::anyhow!("Ollama connection failed: {}", e));
            }
        }
        
        Ok(())
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<OllamaModel>> {
        let models = self.models.read().await;
        Ok(models.values().cloned().collect())
    }

    /// Pull a model from Ollama registry
    pub async fn pull_model(&self, model_name: &str) -> Result<()> {
        info!("Pulling model: {}", model_name);
        
        let request = ollama_rs::generation::completion::request::GenerationRequest::new(
            model_name.to_string(),
            "test".to_string(), // Just to check if model exists
        );
        
        // This is a simplified version - real implementation would use proper pull API
        match self.client.generate(request).await {
            Ok(_) => {
                info!("Model {} is available", model_name);
                self.refresh_models().await?;
                Ok(())
            }
            Err(e) => {
                error!("Failed to access model {}: {}", model_name, e);
                Err(anyhow::anyhow!("Model pull failed: {}", e))
            }
        }
    }

    /// Create a new chat session
    pub async fn create_chat_session(&self, model_name: String, parameters: Option<OllamaParameters>) -> Result<Uuid> {
        let session_id = Uuid::new_v4();
        let session = ChatSession {
            id: session_id,
            model_name,
            messages: Vec::new(),
            parameters: parameters.unwrap_or_default(),
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        };

        {
            let mut sessions = self.chat_sessions.write().await;
            sessions.insert(session_id, session);
        }

        info!("Created chat session: {}", session_id);
        Ok(session_id)
    }

    /// Send message in chat session
    pub async fn send_message(&self, session_id: Uuid, message: String) -> Result<String> {
        let response = {
            let mut sessions = self.chat_sessions.write().await;
            let session = sessions.get_mut(&session_id)
                .ok_or_else(|| anyhow::anyhow!("Chat session not found: {}", session_id))?;

            // Add user message
            session.messages.push(ChatMessage {
                role: MessageRole::User,
                content: message.clone(),
                timestamp: chrono::Utc::now(),
            });
            session.last_activity = chrono::Utc::now();

            // Generate response
            let request = ollama_rs::generation::completion::request::GenerationRequest::new(
                session.model_name.clone(),
                message,
            );

            let response = self.client.generate(request).await
                .map_err(|e| anyhow::anyhow!("Ollama generation failed: {}", e))?;

            // Add assistant response
            session.messages.push(ChatMessage {
                role: MessageRole::Assistant,
                content: response.response.clone(),
                timestamp: chrono::Utc::now(),
            });

            response.response
        };

        Ok(response)
    }

    /// Create automated task
    pub async fn create_automated_task(&self, mut task: AutomatedTask) -> Result<Uuid> {
        task.id = Uuid::new_v4();
        let task_id = task.id;

        // Calculate next run time if scheduled
        if let Some(schedule) = &task.schedule {
            task.next_run = Some(self.calculate_next_run(schedule)?);
        }

        {
            let mut tasks = self.tasks.write().await;
            tasks.insert(task_id, task);
        }

        info!("Created automated task: {}", task_id);
        Ok(task_id)
    }

    /// Execute automated task
    pub async fn execute_task(&self, task_id: Uuid) -> Result<TaskExecutionResult> {
        let task = {
            let tasks = self.tasks.read().await;
            tasks.get(&task_id).cloned()
                .ok_or_else(|| anyhow::anyhow!("Task not found: {}", task_id))?
        };

        if !task.enabled {
            return Ok(TaskExecutionResult {
                task_id,
                success: false,
                message: "Task is disabled".to_string(),
                execution_time: chrono::Utc::now(),
                results: Vec::new(),
            });
        }

        info!("Executing automated task: {} ({})", task.name, task_id);
        let start_time = chrono::Utc::now();
        let mut results = Vec::new();

        for action in &task.actions {
            match self.execute_action(action).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    error!("Task action failed: {}", e);
                    return Ok(TaskExecutionResult {
                        task_id,
                        success: false,
                        message: format!("Action failed: {}", e),
                        execution_time: start_time,
                        results,
                    });
                }
            }
        }

        // Update task execution time
        {
            let mut tasks = self.tasks.write().await;
            if let Some(task) = tasks.get_mut(&task_id) {
                task.last_run = Some(start_time);
                if let Some(schedule) = &task.schedule {
                    task.next_run = Some(self.calculate_next_run(schedule)?);
                }
            }
        }

        Ok(TaskExecutionResult {
            task_id,
            success: true,
            message: "Task completed successfully".to_string(),
            execution_time: start_time,
            results,
        })
    }

    /// Research assistant functionality
    pub async fn research_assistant(&self, query: &str, model_name: &str) -> Result<ResearchResult> {
        info!("Starting research for query: {}", query);

        // Create research prompt
        let research_prompt = format!(
            "You are a research assistant. Please provide a comprehensive analysis of the following query:\n\n{}\n\nProvide:\n1. Key insights\n2. Relevant facts\n3. Potential implications\n4. Further research directions",
            query
        );

        let request = ollama_rs::generation::completion::request::GenerationRequest::new(
            model_name.to_string(),
            research_prompt,
        );

        let response = self.client.generate(request).await
            .map_err(|e| anyhow::anyhow!("Research generation failed: {}", e))?;

        Ok(ResearchResult {
            id: Uuid::new_v4(),
            query: query.to_string(),
            model_used: model_name.to_string(),
            analysis: response.response,
            confidence_score: 0.8, // Placeholder
            sources: Vec::new(), // Would be populated in full implementation
            created_at: chrono::Utc::now(),
        })
    }

    /// Code generation assistant
    pub async fn code_generation(&self, specification: &str, language: &str, model_name: &str) -> Result<CodeGenerationResult> {
        info!("Generating code for: {} in {}", specification, language);

        let code_prompt = format!(
            "Generate {} code for the following specification:\n\n{}\n\nProvide:\n1. Clean, well-commented code\n2. Usage examples\n3. Error handling\n4. Testing suggestions",
            language, specification
        );

        let request = ollama_rs::generation::completion::request::GenerationRequest::new(
            model_name.to_string(),
            code_prompt,
        );

        let response = self.client.generate(request).await
            .map_err(|e| anyhow::anyhow!("Code generation failed: {}", e))?;

        Ok(CodeGenerationResult {
            id: Uuid::new_v4(),
            specification: specification.to_string(),
            language: language.to_string(),
            model_used: model_name.to_string(),
            generated_code: response.response,
            quality_score: 0.85, // Placeholder
            created_at: chrono::Utc::now(),
        })
    }

    async fn refresh_models(&self) -> Result<()> {
        // Refresh the models list from Ollama
        self.initialize().await
    }

    async fn execute_action(&self, action: &TaskAction) -> Result<ActionResult> {
        match action {
            TaskAction::LlmQuery { model, prompt, store_result } => {
                let request = ollama_rs::generation::completion::request::GenerationRequest::new(
                    model.clone(),
                    prompt.clone(),
                );
                
                let response = self.client.generate(request).await
                    .map_err(|e| anyhow::anyhow!("LLM query failed: {}", e))?;

                Ok(ActionResult {
                    action_type: "llm_query".to_string(),
                    success: true,
                    result: serde_json::json!({
                        "model": model,
                        "prompt": prompt,
                        "response": response.response,
                        "stored": store_result
                    }),
                    error: None,
                })
            }
            TaskAction::DataExtraction { source, format } => {
                // Placeholder for data extraction
                Ok(ActionResult {
                    action_type: "data_extraction".to_string(),
                    success: true,
                    result: serde_json::json!({
                        "source": source,
                        "format": format,
                        "extracted_data": "placeholder"
                    }),
                    error: None,
                })
            }
            TaskAction::ApiCall { url, method, headers, body } => {
                // Placeholder for API call
                Ok(ActionResult {
                    action_type: "api_call".to_string(),
                    success: true,
                    result: serde_json::json!({
                        "url": url,
                        "method": method,
                        "status": "completed"
                    }),
                    error: None,
                })
            }
            TaskAction::FileOperation { operation, path, content } => {
                // Placeholder for file operations
                Ok(ActionResult {
                    action_type: "file_operation".to_string(),
                    success: true,
                    result: serde_json::json!({
                        "operation": operation,
                        "path": path,
                        "completed": true
                    }),
                    error: None,
                })
            }
            TaskAction::Notification { channel, message } => {
                info!("Notification to {}: {}", channel, message);
                Ok(ActionResult {
                    action_type: "notification".to_string(),
                    success: true,
                    result: serde_json::json!({
                        "channel": channel,
                        "message": message,
                        "sent": true
                    }),
                    error: None,
                })
            }
        }
    }

    fn calculate_next_run(&self, schedule: &TaskSchedule) -> Result<chrono::DateTime<chrono::Utc>> {
        let now = chrono::Utc::now();
        
        match schedule {
            TaskSchedule::Interval { seconds } => {
                Ok(now + chrono::Duration::seconds(*seconds as i64))
            }
            TaskSchedule::Daily { hour, minute } => {
                let next_day = now.date_naive() + chrono::Duration::days(1);
                let next_run = next_day.and_hms_opt(*hour as u32, *minute as u32, 0)
                    .ok_or_else(|| anyhow::anyhow!("Invalid time: {}:{}", hour, minute))?
                    .and_utc();
                Ok(next_run)
            }
            TaskSchedule::Weekly { day, hour, minute } => {
                // Simplified weekly calculation
                let days_ahead = (*day as i64 - now.weekday().number_from_monday() as i64 + 7) % 7;
                let next_week = now.date_naive() + chrono::Duration::days(days_ahead);
                let next_run = next_week.and_hms_opt(*hour as u32, *minute as u32, 0)
                    .ok_or_else(|| anyhow::anyhow!("Invalid time: {}:{}", hour, minute))?
                    .and_utc();
                Ok(next_run)
            }
            TaskSchedule::Cron(cron_expr) => {
                // Placeholder for cron parsing - would use a cron library
                Ok(now + chrono::Duration::hours(1))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskExecutionResult {
    pub task_id: Uuid,
    pub success: bool,
    pub message: String,
    pub execution_time: chrono::DateTime<chrono::Utc>,
    pub results: Vec<ActionResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_type: String,
    pub success: bool,
    pub result: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResearchResult {
    pub id: Uuid,
    pub query: String,
    pub model_used: String,
    pub analysis: String,
    pub confidence_score: f32,
    pub sources: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGenerationResult {
    pub id: Uuid,
    pub specification: String,
    pub language: String,
    pub model_used: String,
    pub generated_code: String,
    pub quality_score: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
} 