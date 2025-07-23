use async_graphql::{Context, Object, Result, SimpleObject, Enum, ID};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::{AppState, ProcessIntentRequest, UserPreferences};

/// GraphQL Query Root
pub struct QueryRoot;

/// GraphQL Mutation Root  
pub struct MutationRoot;

/// Intent representation for GraphQL
#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct IntentGQL {
    pub id: ID,
    pub raw_text: String,
    pub structured_goal: String,
    pub domain: String,
    pub complexity: f64,
    pub confidence: f64,
    pub risk_level: RiskLevelGQL,
    pub created_at: DateTime<Utc>,
}

/// Execution plan for GraphQL
#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlanGQL {
    pub id: ID,
    pub intent_id: ID,
    pub estimated_duration: i32, // minutes
    pub autonomy_tier: i32,
    pub tasks: Vec<TaskGQL>,
    pub created_at: DateTime<Utc>,
    pub status: ExecutionStatusGQL,
}

/// Task representation for GraphQL
#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct TaskGQL {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub task_type: TaskTypeGQL,
    pub agent_type: String,
    pub estimated_duration: i32, // minutes
    pub status: TaskStatusGQL,
    pub dry_run_first: bool,
}

/// Risk level enum for GraphQL
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevelGQL {
    Low,
    Medium,
    High,
    Critical,
}

/// Task type enum for GraphQL
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskTypeGQL {
    Sense,
    Plan,
    Execute,
    Verify,
    Reflect,
}

/// Task status enum for GraphQL
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatusGQL {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    WaitingApproval,
}

/// Execution status enum for GraphQL
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatusGQL {
    Planning,
    Executing,
    Completed,
    Failed,
    Cancelled,
}

/// User information for GraphQL
#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct UserGQL {
    pub id: ID,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub preferences: Option<UserPreferencesGQL>,
}

/// User preferences for GraphQL
#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferencesGQL {
    pub max_autonomy_tier: Option<i32>,
    pub require_approval_for_risks: Vec<RiskLevelGQL>,
    pub preferred_execution_mode: Option<String>,
}

/// Cognitive kernel status for GraphQL
#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct KernelStatusGQL {
    pub status: String,
    pub active_contexts: i32,
    pub processed_intents_today: i32,
    pub average_processing_time_ms: f64,
    pub memory_usage_mb: f64,
}

#[Object]
impl QueryRoot {
    /// Get system health status
    async fn health(&self, ctx: &Context<'_>) -> Result<String> {
        let _state = ctx.data::<AppState>()?;
        Ok("healthy".to_string())
    }

    /// Get current user information
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<UserGQL>> {
        let _state = ctx.data::<AppState>()?;
        // TODO: Implement user retrieval from session
        Ok(None)
    }

    /// Get intent by ID
    async fn intent(&self, ctx: &Context<'_>, id: ID) -> Result<Option<IntentGQL>> {
        let _state = ctx.data::<AppState>()?;
        let _intent_id = Uuid::parse_str(&id)?;
        
        // TODO: Implement intent retrieval from database
        Ok(None)
    }

    /// List all intents for current user
    async fn intents(
        &self, 
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<String>,
    ) -> Result<Vec<IntentGQL>> {
        let _state = ctx.data::<AppState>()?;
        let _limit = first.unwrap_or(20).min(100);
        
        // TODO: Implement pagination and intent listing
        Ok(vec![])
    }

    /// Get execution plan by ID
    async fn execution_plan(&self, ctx: &Context<'_>, id: ID) -> Result<Option<ExecutionPlanGQL>> {
        let _state = ctx.data::<AppState>()?;
        let _plan_id = Uuid::parse_str(&id)?;
        
        // TODO: Implement execution plan retrieval
        Ok(None)
    }

    /// List execution plans
    async fn execution_plans(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        status: Option<ExecutionStatusGQL>,
    ) -> Result<Vec<ExecutionPlanGQL>> {
        let _state = ctx.data::<AppState>()?;
        let _limit = first.unwrap_or(20).min(100);
        
        // TODO: Implement execution plan listing with filtering
        Ok(vec![])
    }

    /// Get task by ID
    async fn task(&self, ctx: &Context<'_>, id: ID) -> Result<Option<TaskGQL>> {
        let _state = ctx.data::<AppState>()?;
        let _task_id = Uuid::parse_str(&id)?;
        
        // TODO: Implement task retrieval
        Ok(None)
    }

    /// List tasks
    async fn tasks(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        status: Option<TaskStatusGQL>,
        plan_id: Option<ID>,
    ) -> Result<Vec<TaskGQL>> {
        let _state = ctx.data::<AppState>()?;
        let _limit = first.unwrap_or(20).min(100);
        
        // TODO: Implement task listing with filtering
        Ok(vec![])
    }

    /// Get cognitive kernel status
    async fn kernel_status(&self, ctx: &Context<'_>) -> Result<KernelStatusGQL> {
        let state = ctx.data::<AppState>()?;
        
        Ok(KernelStatusGQL {
            status: "operational".to_string(),
            active_contexts: state.active_sessions.len() as i32,
            processed_intents_today: 0, // TODO: Implement counter
            average_processing_time_ms: 150.0, // TODO: Calculate from metrics
            memory_usage_mb: 0.0, // TODO: Get actual memory usage
        })
    }

    /// Search vectors
    async fn vector_search(
        &self,
        ctx: &Context<'_>,
        query: String,
        limit: Option<i32>,
        threshold: Option<f64>,
    ) -> Result<Vec<VectorSearchResult>> {
        let _state = ctx.data::<AppState>()?;
        let _search_limit = limit.unwrap_or(10).min(100);
        let _similarity_threshold = threshold.unwrap_or(0.7);
        
        // TODO: Implement vector search
        Ok(vec![])
    }
}

/// Vector search result for GraphQL
#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub id: ID,
    pub content: String,
    pub similarity: f64,
    pub metadata: Option<String>, // JSON metadata
}

#[Object]
impl MutationRoot {
    /// Process a new intent
    async fn process_intent(
        &self,
        ctx: &Context<'_>,
        intent: String,
        context: Option<String>,
    ) -> Result<ExecutionPlanGQL> {
        let state = ctx.data::<AppState>()?;
        
        // Process through cognitive kernel
        let plan = state.cognitive_kernel.process_intent(&intent, None).await
            .map_err(|e| async_graphql::Error::new(format!("Failed to process intent: {}", e)))?;

        // Convert to GraphQL format
        let tasks: Vec<TaskGQL> = plan.tasks.iter().map(|task| TaskGQL {
            id: ID::from(task.id.to_string()),
            name: task.name.clone(),
            description: task.description.clone(),
            task_type: match task.task_type {
                jarvis_core::TaskType::Sense => TaskTypeGQL::Sense,
                jarvis_core::TaskType::Plan => TaskTypeGQL::Plan,
                jarvis_core::TaskType::Execute => TaskTypeGQL::Execute,
                jarvis_core::TaskType::Verify => TaskTypeGQL::Verify,
                jarvis_core::TaskType::Reflect => TaskTypeGQL::Reflect,
            },
            agent_type: task.agent_type.clone(),
            estimated_duration: task.estimated_duration.num_minutes() as i32,
            status: match task.status {
                jarvis_core::TaskStatus::Pending => TaskStatusGQL::Pending,
                jarvis_core::TaskStatus::InProgress => TaskStatusGQL::InProgress,
                jarvis_core::TaskStatus::Completed => TaskStatusGQL::Completed,
                jarvis_core::TaskStatus::Failed => TaskStatusGQL::Failed,
                jarvis_core::TaskStatus::Cancelled => TaskStatusGQL::Cancelled,
                jarvis_core::TaskStatus::WaitingApproval => TaskStatusGQL::WaitingApproval,
            },
            dry_run_first: task.dry_run_first,
        }).collect();

        let gql_plan = ExecutionPlanGQL {
            id: ID::from(plan.id.to_string()),
            intent_id: ID::from(plan.intent_id.to_string()),
            estimated_duration: plan.estimated_duration.num_minutes() as i32,
            autonomy_tier: plan.autonomy_tier as i32,
            tasks,
            created_at: plan.created_at,
            status: ExecutionStatusGQL::Planning,
        };

        // TODO: Store in database

        Ok(gql_plan)
    }

    /// Execute a plan
    async fn execute_plan(&self, ctx: &Context<'_>, plan_id: ID) -> Result<ExecutionPlanGQL> {
        let _state = ctx.data::<AppState>()?;
        let _id = Uuid::parse_str(&plan_id)?;
        
        // TODO: Implement plan execution
        Err(async_graphql::Error::new("Plan execution not yet implemented"))
    }

    /// Cancel a plan
    async fn cancel_plan(&self, ctx: &Context<'_>, plan_id: ID) -> Result<bool> {
        let _state = ctx.data::<AppState>()?;
        let _id = Uuid::parse_str(&plan_id)?;
        
        // TODO: Implement plan cancellation
        Ok(false)
    }

    /// Approve a task
    async fn approve_task(&self, ctx: &Context<'_>, task_id: ID) -> Result<TaskGQL> {
        let _state = ctx.data::<AppState>()?;
        let _id = Uuid::parse_str(&task_id)?;
        
        // TODO: Implement task approval
        Err(async_graphql::Error::new("Task approval not yet implemented"))
    }

    /// Reject a task
    async fn reject_task(
        &self,
        ctx: &Context<'_>,
        task_id: ID,
        reason: Option<String>,
    ) -> Result<TaskGQL> {
        let _state = ctx.data::<AppState>()?;
        let _id = Uuid::parse_str(&task_id)?;
        let _rejection_reason = reason.unwrap_or_else(|| "No reason provided".to_string());
        
        // TODO: Implement task rejection
        Err(async_graphql::Error::new("Task rejection not yet implemented"))
    }

    /// Update user preferences
    async fn update_preferences(
        &self,
        ctx: &Context<'_>,
        preferences: UserPreferencesInput,
    ) -> Result<UserGQL> {
        let _state = ctx.data::<AppState>()?;
        
        // TODO: Implement preferences update
        Err(async_graphql::Error::new("Preferences update not yet implemented"))
    }
}

/// Input type for user preferences
#[derive(async_graphql::InputObject, Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferencesInput {
    pub max_autonomy_tier: Option<i32>,
    pub require_approval_for_risks: Option<Vec<RiskLevelGQL>>,
    pub preferred_execution_mode: Option<String>,
} 