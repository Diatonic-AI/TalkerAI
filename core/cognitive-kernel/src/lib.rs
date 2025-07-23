use std::sync::Arc;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use dashmap::DashMap;
use anyhow::Result;

pub mod intent_graph;
pub mod adaptive_planner;

pub use intent_graph::IntentGraphBuilder;
pub use adaptive_planner::AdaptivePlanner;

/// Core cognitive kernel that orchestrates all JARVIS thinking processes
#[derive(Debug)]
pub struct CognitiveKernel {
    pub intent_graph: Arc<IntentGraphBuilder>,
    pub planner: Arc<AdaptivePlanner>,
    pub active_contexts: Arc<DashMap<Uuid, ExecutionContext>>,
    pub global_state: Arc<DashMap<String, serde_json::Value>>,
}

impl CognitiveKernel {
    pub fn new() -> Self {
        Self {
            intent_graph: Arc::new(IntentGraphBuilder::new()),
            planner: Arc::new(AdaptivePlanner::new()),
            active_contexts: Arc::new(DashMap::new()),
            global_state: Arc::new(DashMap::new()),
        }
    }

    /// Primary entry point: converts user intent into executable plan
    pub async fn process_intent(&self, raw_intent: &str, _context: Option<ExecutionContext>) -> Result<IntentExecutionPlan> {
        tracing::info!("Processing intent: {}", raw_intent);
        
        // 1. Parse and structure the intent
        let intent = self.intent_graph.parse_intent(raw_intent).await?;
        
        // 2. Create execution context
        let ctx_id = Uuid::new_v4();
        let ctx = ExecutionContext::new(intent.id);
        self.active_contexts.insert(ctx_id, ctx);
        
        // 3. Generate tasks for the domain
        let tasks = self.intent_graph.generate_tasks_for_domain(&intent.domain, &intent)?;
        
        // 4. Create execution plan
        let plan = self.planner.create_execution_plan(tasks, &intent).await?;
        
        tracing::info!("Generated execution plan with {} tasks", plan.tasks.len());
        Ok(plan)
    }

    /// Query global cognitive state
    pub fn get_global_state(&self, key: &str) -> Option<serde_json::Value> {
        self.global_state.get(key).map(|v| v.clone())
    }

    /// Update global cognitive state
    pub fn set_global_state(&self, key: String, value: serde_json::Value) {
        self.global_state.insert(key, value);
    }

    /// Public methods for demonstration
    pub fn classify_domain(&self, text: &str) -> String {
        self.intent_graph.classify_domain(text)
    }

    pub fn assess_risk(&self, text: &str) -> RiskLevel {
        self.intent_graph.assess_risk(text)
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
        assert!(plan.autonomy_tier > 0);
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