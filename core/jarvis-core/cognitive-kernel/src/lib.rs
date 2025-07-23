use std::sync::Arc;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use dashmap::DashMap;
use anyhow::{Result, anyhow};

/// Core cognitive kernel that orchestrates all JARVIS thinking processes
#[derive(Debug)]
pub struct CognitiveKernel {
    pub active_contexts: Arc<DashMap<Uuid, ExecutionContext>>,
    pub global_state: Arc<DashMap<String, serde_json::Value>>,
}

impl CognitiveKernel {
    pub fn new() -> Self {
        Self {
            active_contexts: Arc::new(DashMap::new()),
            global_state: Arc::new(DashMap::new()),
        }
    }

    /// Primary entry point: converts user intent into executable plan
    pub async fn process_intent(&self, raw_intent: &str, _context: Option<ExecutionContext>) -> Result<IntentExecutionPlan> {
        tracing::info!("Processing intent: {}", raw_intent);
        
        // Parse and classify the intent
        let intent = self.parse_intent(raw_intent).await?;
        
        // Create execution context
        let ctx_id = Uuid::new_v4();
        let ctx = ExecutionContext::new(intent.id);
        self.active_contexts.insert(ctx_id, ctx);
        
        // Generate execution plan
        let plan = self.create_execution_plan(&intent).await?;
        
        tracing::info!("Generated execution plan with {} tasks", plan.tasks.len());
        Ok(plan)
    }

    async fn parse_intent(&self, raw_text: &str) -> Result<Intent> {
        let domain = self.classify_domain(raw_text);
        let risk_level = self.assess_risk(raw_text);
        
        Ok(Intent {
            id: Uuid::new_v4(),
            raw_text: raw_text.to_string(),
            structured_goal: format!("[{}] {}", domain, raw_text),
            domain,
            complexity: 0.5,
            confidence: 0.8,
            constraints: Vec::new(),
            success_criteria: vec!["task_completed".to_string()],
            risk_level,
            created_at: Utc::now(),
        })
    }

    pub fn classify_domain(&self, text: &str) -> String {
        let text_lower = text.to_lowercase();
        
        if text_lower.contains("deploy") || text_lower.contains("kubernetes") || text_lower.contains("docker") {
            "infra_deployment".to_string()
        } else if text_lower.contains("database") || text_lower.contains("postgres") || text_lower.contains("sql") {
            "database_admin".to_string()
        } else if text_lower.contains("marketing") || text_lower.contains("content") || text_lower.contains("blog") {
            "marketing_content".to_string()
        } else {
            "general".to_string()
        }
    }

    pub fn assess_risk(&self, text: &str) -> RiskLevel {
        let text_lower = text.to_lowercase();
        
        if text_lower.contains("production") || text_lower.contains("delete") || text_lower.contains("drop") {
            RiskLevel::Critical
        } else if text_lower.contains("modify") || text_lower.contains("update") || text_lower.contains("migrate") {
            RiskLevel::High
        } else if text_lower.contains("create") || text_lower.contains("deploy") {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }

    async fn create_execution_plan(&self, intent: &Intent) -> Result<IntentExecutionPlan> {
        let tasks = self.generate_tasks_for_domain(&intent.domain, intent)?;
        
        Ok(IntentExecutionPlan {
            id: Uuid::new_v4(),
            intent_id: intent.id,
            tasks,
            dependencies: Vec::new(),
            estimated_duration: Duration::minutes(15),
            autonomy_tier: self.determine_autonomy_tier(intent),
            checkpoints: Vec::new(),
            rollback_plan: None,
            created_at: Utc::now(),
        })
    }

    fn generate_tasks_for_domain(&self, domain: &str, intent: &Intent) -> Result<Vec<ExecutionTask>> {
        let mut tasks = Vec::new();
        
        match domain {
            "infra_deployment" => {
                tasks.push(ExecutionTask {
                    id: Uuid::new_v4(),
                    name: "analyze_requirements".to_string(),
                    description: "Analyze deployment requirements and constraints".to_string(),
                    task_type: TaskType::Sense,
                    agent_type: "analyzer-agent".to_string(),
                    inputs: HashMap::new(),
                    expected_outputs: vec!["requirements.json".to_string()],
                    estimated_duration: Duration::minutes(5),
                    status: TaskStatus::Pending,
                    dry_run_first: false,
                });
                
                tasks.push(ExecutionTask {
                    id: Uuid::new_v4(),
                    name: "create_deployment_plan".to_string(),
                    description: "Create detailed deployment plan".to_string(),
                    task_type: TaskType::Plan,
                    agent_type: "planner-agent".to_string(),
                    inputs: HashMap::new(),
                    expected_outputs: vec!["deployment-plan.yaml".to_string()],
                    estimated_duration: Duration::minutes(10),
                    status: TaskStatus::Pending,
                    dry_run_first: false,
                });
            },
            _ => {
                tasks.push(ExecutionTask {
                    id: Uuid::new_v4(),
                    name: "generic_task".to_string(),
                    description: format!("Execute {} task", domain),
                    task_type: TaskType::Execute,
                    agent_type: "generic-agent".to_string(),
                    inputs: HashMap::new(),
                    expected_outputs: vec!["result.json".to_string()],
                    estimated_duration: Duration::minutes(10),
                    status: TaskStatus::Pending,
                    dry_run_first: true,
                });
            }
        }
        
        Ok(tasks)
    }

    fn determine_autonomy_tier(&self, intent: &Intent) -> u8 {
        match intent.risk_level {
            RiskLevel::Low => 3,
            RiskLevel::Medium => 2,
            RiskLevel::High => 2,
            RiskLevel::Critical => 1,
        }
    }

    /// Query global cognitive state
    pub fn get_global_state(&self, key: &str) -> Option<serde_json::Value> {
        self.global_state.get(key).map(|v| v.clone())
    }

    /// Update global cognitive state
    pub fn set_global_state(&self, key: String, value: serde_json::Value) {
        self.global_state.insert(key, value);
    }
}

/// Core intent structure with metadata and classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub id: Uuid,
    pub raw_text: String,
    pub structured_goal: String,
    pub domain: String,
    pub complexity: f64,
    pub confidence: f64,
    pub constraints: Vec<String>,
    pub success_criteria: Vec<String>,
    pub risk_level: RiskLevel,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium, 
    High,
    Critical,
}

/// Execution plan with hierarchical tasks and dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentExecutionPlan {
    pub id: Uuid,
    pub intent_id: Uuid,
    pub tasks: Vec<ExecutionTask>,
    pub dependencies: Vec<TaskDependency>,
    pub estimated_duration: Duration,
    pub autonomy_tier: u8,
    pub checkpoints: Vec<Checkpoint>,
    pub rollback_plan: Option<RollbackPlan>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTask {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub task_type: TaskType,
    pub agent_type: String,
    pub inputs: HashMap<String, serde_json::Value>,
    pub expected_outputs: Vec<String>,
    pub estimated_duration: Duration,
    pub status: TaskStatus,
    pub dry_run_first: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Sense,
    Plan,
    Execute,
    Verify,
    Reflect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    WaitingApproval,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    pub from_task: Uuid,
    pub to_task: Uuid,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Sequential,
    Conditional,
    DataFlow,
    Resource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub task_id: Uuid,
    pub description: String,
    pub requires_approval: bool,
    pub auto_rollback_on_fail: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackPlan {
    pub steps: Vec<RollbackStep>,
    pub auto_trigger_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackStep {
    pub id: Uuid,
    pub description: String,
    pub command: String,
    pub verification: String,
}

/// Execution context that tracks state and progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub id: Uuid,
    pub intent_id: Uuid,
    pub execution_state: ExecutionState,
    pub created_at: DateTime<Utc>,
}

impl ExecutionContext {
    pub fn new(intent_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            intent_id,
            execution_state: ExecutionState::Planning,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionState {
    Planning,
    Executing,
    Completed,
    Failed { error: String },
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cognitive_kernel_creation() {
        let kernel = CognitiveKernel::new();
        assert!(kernel.active_contexts.is_empty());
        assert!(kernel.global_state.is_empty());
    }

    #[tokio::test]
    async fn test_intent_processing() {
        let kernel = CognitiveKernel::new();
        
        let result = kernel.process_intent(
            "Deploy the marketing website to staging",
            None
        ).await;
        
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert!(!plan.tasks.is_empty());
        assert_eq!(plan.autonomy_tier, 2); // Medium risk = tier 2
    }

    #[test]
    fn test_domain_classification() {
        let kernel = CognitiveKernel::new();
        
        assert_eq!(kernel.classify_domain("deploy kubernetes app"), "infra_deployment");
        assert_eq!(kernel.classify_domain("backup postgres database"), "database_admin");
        assert_eq!(kernel.classify_domain("create marketing content"), "marketing_content");
        assert_eq!(kernel.classify_domain("random task"), "general");
    }

    #[test]
    fn test_risk_assessment() {
        let kernel = CognitiveKernel::new();
        
        assert!(matches!(kernel.assess_risk("delete production database"), RiskLevel::Critical));
        assert!(matches!(kernel.assess_risk("update user profile"), RiskLevel::High));
        assert!(matches!(kernel.assess_risk("create new deployment"), RiskLevel::Medium));
        assert!(matches!(kernel.assess_risk("read configuration"), RiskLevel::Low));
    }
} 