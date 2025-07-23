use cognitive_kernel::CognitiveKernel;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize simple logging
    println!("🧠 Initializing JARVIS Cognitive Kernel");
    
    // Create the cognitive kernel
    let kernel = CognitiveKernel::new();
    
    // Example 1: Simple infrastructure deployment
    println!("\n📋 Example 1: Infrastructure Deployment");
    let deployment_intent = "Deploy the marketing website to staging with blue-green deployment";
    
    match kernel.process_intent(deployment_intent, None).await {
        Ok(plan) => {
            println!("✅ Generated execution plan with {} tasks", plan.tasks.len());
            println!("   Estimated duration: {}min", plan.estimated_duration.num_minutes());
            println!("   Autonomy tier: {}", plan.autonomy_tier);
            
            // Print task breakdown
            for (i, task) in plan.tasks.iter().enumerate() {
                println!("   Task {}: {} ({:?})", i+1, task.name, task.task_type);
            }
        },
        Err(e) => {
            println!("❌ Failed to process intent: {}", e);
        }
    }
    
    // Example 2: Database operation
    println!("\n📋 Example 2: Database Migration");
    let db_intent = "Backup the postgres database and migrate to new schema version 2.1";
    
    match kernel.process_intent(db_intent, None).await {
        Ok(plan) => {
            println!("✅ Generated database plan with {} tasks", plan.tasks.len());
            println!("   Risk level: {:?} (database operations)", plan.tasks[0].dry_run_first);
        },
        Err(e) => {
            println!("❌ Failed to process db intent: {}", e);
        }
    }
    
    // Example 3: Marketing content creation
    println!("\n📋 Example 3: Marketing Content");
    let marketing_intent = "Create blog post about our new AI features with SEO optimization";
    
    match kernel.process_intent(marketing_intent, None).await {
        Ok(plan) => {
            println!("✅ Generated marketing plan with {} tasks", plan.tasks.len());
            println!("   Domain: Marketing (higher autonomy)");
        },
        Err(e) => {
            println!("❌ Failed to process marketing intent: {}", e);
        }
    }
    
    // Example 4: Show cognitive state
    println!("\n🧠 Cognitive Kernel State:");
    println!("   Active contexts: {}", kernel.active_contexts.len());
    println!("   Global state entries: {}", kernel.global_state.len());
    
    // Set some global context
    kernel.set_global_state(
        "current_environment".to_string(),
        serde_json::Value::String("staging".to_string())
    );
    
    kernel.set_global_state(
        "deployment_strategy".to_string(),
        serde_json::Value::String("blue_green".to_string())
    );
    
    println!("   Updated global state with deployment preferences");
    
    // Example 5: Domain classification demonstration
    println!("\n🔍 Domain Classification Examples:");
    let test_intents = vec![
        "Deploy kubernetes application to production",
        "Backup postgresql database with compression",
        "Write marketing email for product launch",
        "Clean up old log files and temporary data",
        "Rotate API keys for security compliance",
    ];
    
    for intent in test_intents {
        let domain = kernel.classify_domain(intent);
        let risk = kernel.assess_risk(intent);
        println!("   '{}' → Domain: {}, Risk: {:?}", intent, domain, risk);
    }
    
    println!("\n🎯 JARVIS Cognitive Kernel demonstration complete!");
    println!("   Ready to process real intents and execute plans");
    
    Ok(())
} 