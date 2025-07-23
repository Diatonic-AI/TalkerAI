use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use axum::{
    extract::{Extension, Path, Query, State},
    http::{header, Method, StatusCode},
    middleware,
    response::{IntoResponse, Json},
    routing::{get, post, put, delete},
    Router,
};
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::{info, instrument, Level};
use uuid::Uuid;

use jarvis_core::{CognitiveKernel, Intent, IntentExecutionPlan, RiskLevel};

mod auth;
mod config;
mod error;
mod handlers;
mod middleware as custom_middleware;
mod models;
mod schema;
mod services;

use config::Config;
use error::{ApiError, ApiResult};
use models::*;
use schema::{MutationRoot, QueryRoot};

/// Main application state
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::Client,
    pub cognitive_kernel: Arc<CognitiveKernel>,
    pub active_sessions: Arc<DashMap<Uuid, UserSession>>,
    pub config: Arc<Config>,
}

/// User session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub permissions: Vec<String>,
}

/// Health check response
#[derive(Serialize, SimpleObject)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub cognitive_kernel_status: String,
    pub database_status: String,
    pub redis_status: String,
}

/// Intent processing request
#[derive(Debug, Deserialize)]
pub struct ProcessIntentRequest {
    pub intent: String,
    pub context: Option<serde_json::Value>,
    pub user_preferences: Option<UserPreferences>,
}

/// Intent processing response
#[derive(Debug, Serialize, SimpleObject)]
pub struct ProcessIntentResponse {
    pub plan_id: Uuid,
    pub intent_id: Uuid,
    pub estimated_duration: i64, // minutes
    pub autonomy_tier: u8,
    pub tasks: Vec<TaskSummary>,
    pub risk_level: String,
    pub requires_approval: bool,
}

/// Task summary for API responses
#[derive(Debug, Serialize, SimpleObject)]
pub struct TaskSummary {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub task_type: String,
    pub estimated_duration: i64,
    pub status: String,
    pub dry_run_first: bool,
}

/// User preferences
#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct UserPreferences {
    pub max_autonomy_tier: Option<u8>,
    pub require_approval_for_risks: Vec<String>,
    pub preferred_execution_mode: Option<String>,
    pub notification_preferences: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .json()
        .init();

    info!("🚀 Starting Talk++ API Server");

    // Load configuration
    let config = Arc::new(Config::load()?);
    info!("✅ Configuration loaded");

    // Initialize database
    let database_url = config.database_url.as_ref()
        .expect("DATABASE_URL must be set");
    let db = PgPool::connect(database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&db).await?;
    info!("✅ Database migrations completed");

    // Initialize Redis
    let redis_url = config.redis_url.as_ref()
        .expect("REDIS_URL must be set");
    let redis_client = redis::Client::open(redis_url.as_str())?;
    info!("✅ Redis connection established");

    // Initialize JARVIS Cognitive Kernel
    let cognitive_kernel = Arc::new(CognitiveKernel::new());
    info!("✅ JARVIS Cognitive Kernel initialized");

    // Initialize application state
    let app_state = AppState {
        db,
        redis: redis_client,
        cognitive_kernel,
        active_sessions: Arc::new(DashMap::new()),
        config: config.clone(),
    };

    // Create GraphQL schema
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(app_state.clone())
        .finish();

    // Build the application router  
    let app = Router::new()
        // Health checks
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        
        // API v1 routes
        .nest("/api/v1", api_v1_routes())
        
        // GraphQL endpoint
        .route("/graphql", post(graphql_handler))
        .route("/graphql/playground", get(graphql_playground))
        
        // Metrics endpoint (for Prometheus)
        .route("/metrics", get(metrics_handler))
        
        // WebSocket for real-time updates
        .route("/ws", get(websocket_handler))
        
        // State and middleware
        .layer(Extension(schema))
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                        .max_age(Duration::from_secs(3600))
                )
                .layer(middleware::from_fn(custom_middleware::request_id))
                .layer(middleware::from_fn(custom_middleware::rate_limit))
        );

    // Start the server
    let bind_addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&bind_addr).await?;
    
    info!("🌐 Server listening on http://{}", bind_addr);
    info!("📊 GraphQL Playground available at http://{}/graphql/playground", bind_addr);
    info!("📈 Metrics available at http://{}/metrics", bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}

/// API v1 routes
fn api_v1_routes() -> Router<AppState> {
    Router::new()
        // Intent processing
        .route("/intents", post(process_intent))
        .route("/intents/:intent_id", get(get_intent))
        .route("/intents/:intent_id/status", get(get_intent_status))
        
        // Execution plans
        .route("/plans", get(list_execution_plans))
        .route("/plans/:plan_id", get(get_execution_plan))
        .route("/plans/:plan_id/execute", post(execute_plan))
        .route("/plans/:plan_id/cancel", post(cancel_plan))
        
        // Tasks
        .route("/tasks", get(list_tasks))
        .route("/tasks/:task_id", get(get_task))
        .route("/tasks/:task_id/approve", post(approve_task))
        .route("/tasks/:task_id/reject", post(reject_task))
        
        // User management
        .route("/users/me", get(get_current_user))
        .route("/users/me/preferences", get(get_user_preferences))
        .route("/users/me/preferences", put(update_user_preferences))
        
        // Cognitive kernel status
        .route("/kernel/status", get(get_kernel_status))
        .route("/kernel/metrics", get(get_kernel_metrics))
        
        // Vector database operations
        .route("/vectors/search", post(vector_search))
        .route("/vectors/embed", post(embed_text))
        
        // MCP operations
        .route("/mcp/servers", get(list_mcp_servers))
        .route("/mcp/servers/:server_id/tools", get(list_mcp_tools))
        .route("/mcp/tools/:tool_id/execute", post(execute_mcp_tool))
}

/// Health check endpoint
#[instrument]
async fn health_check(State(state): State<AppState>) -> ApiResult<Json<HealthResponse>> {
    let database_status = match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => "healthy".to_string(),
        Err(_) => "unhealthy".to_string(),
    };

    let redis_status = match redis::cmd("PING").query_async::<_, String>(&mut state.redis.get_connection_manager().await?).await {
        Ok(_) => "healthy".to_string(),
        Err(_) => "unhealthy".to_string(),
    };

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        cognitive_kernel_status: "operational".to_string(),
        database_status,
        redis_status,
    }))
}

/// Readiness check endpoint
#[instrument]
async fn readiness_check() -> impl IntoResponse {
    (StatusCode::OK, "ready")
}

/// Process intent endpoint
#[instrument(skip(state))]
async fn process_intent(
    State(state): State<AppState>,
    Json(request): Json<ProcessIntentRequest>,
) -> ApiResult<Json<ProcessIntentResponse>> {
    info!("Processing intent: {}", request.intent);

    // Process intent through cognitive kernel
    let plan = state.cognitive_kernel
        .process_intent(&request.intent, None)
        .await
        .map_err(|e| ApiError::InternalError(format!("Failed to process intent: {}", e)))?;

    // Convert tasks to API format
    let tasks: Vec<TaskSummary> = plan.tasks.iter().map(|task| TaskSummary {
        id: task.id,
        name: task.name.clone(),
        description: task.description.clone(),
        task_type: format!("{:?}", task.task_type),
        estimated_duration: task.estimated_duration.num_minutes(),
        status: format!("{:?}", task.status),
        dry_run_first: task.dry_run_first,
    }).collect();

    let response = ProcessIntentResponse {
        plan_id: plan.id,
        intent_id: plan.intent_id,
        estimated_duration: plan.estimated_duration.num_minutes(),
        autonomy_tier: plan.autonomy_tier,
        tasks,
        risk_level: format!("{:?}", state.cognitive_kernel.assess_risk(&request.intent)),
        requires_approval: plan.autonomy_tier <= 2,
    };

    // Store plan in database
    // TODO: Implement database storage

    Ok(Json(response))
}

/// GraphQL handler
async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// GraphQL playground
async fn graphql_playground() -> impl IntoResponse {
    use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
    
    axum::response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

/// Metrics endpoint for Prometheus
async fn metrics_handler() -> impl IntoResponse {
    // TODO: Implement metrics collection
    "# Talk++ API Server Metrics\n"
}

/// WebSocket handler for real-time updates
async fn websocket_handler() -> impl IntoResponse {
    // TODO: Implement WebSocket handler
    (StatusCode::NOT_IMPLEMENTED, "WebSocket endpoint not yet implemented")
}

// Placeholder handlers - these would be implemented in separate handler modules
async fn get_intent(Path(_intent_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn get_intent_status(Path(_intent_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn list_execution_plans() -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"plans": []})))
}

async fn get_execution_plan(Path(_plan_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn execute_plan(Path(_plan_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn cancel_plan(Path(_plan_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn list_tasks() -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"tasks": []})))
}

async fn get_task(Path(_task_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn approve_task(Path(_task_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn reject_task(Path(_task_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn get_current_user() -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn get_user_preferences() -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn update_user_preferences(Json(_prefs): Json<UserPreferences>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}

async fn get_kernel_status() -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"status": "operational"})))
}

async fn get_kernel_metrics() -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"metrics": {}})))
}

async fn vector_search(Json(_query): Json<serde_json::Value>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"results": []})))
}

async fn embed_text(Json(_text): Json<serde_json::Value>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"embedding": []})))
}

async fn list_mcp_servers() -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"servers": []})))
}

async fn list_mcp_tools(Path(_server_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"tools": []})))
}

async fn execute_mcp_tool(Path(_tool_id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"result": "not_implemented"})))
}
