use std::collections::HashMap;
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::{Intent, RiskLevel, ExecutionTask, TaskType, TaskStatus};

/// Builds hierarchical intent graphs from natural language
#[derive(Debug)]
pub struct IntentGraphBuilder {
    domain_patterns: HashMap<String, Vec<String>>,
    risk_patterns: HashMap<RiskLevel, Vec<String>>,
}

impl IntentGraphBuilder {
    pub fn new() -> Self {
        let mut builder = Self {
            domain_patterns: HashMap::new(),
            risk_patterns: HashMap::new(),
        };
        
        builder.init_patterns();
        builder
    }

    /// Parse natural language into structured intent
    pub async fn parse_intent(&self, raw_text: &str) -> Result<Intent> {
        let domain = self.classify_domain(raw_text);
        let risk_level = self.assess_risk(raw_text);
        let (constraints, success_criteria) = self.extract_constraints(raw_text);
        
        Ok(Intent {
            id: Uuid::new_v4(),
            raw_text: raw_text.to_string(),
            structured_goal: format!("[{}] {}", domain, raw_text),
            domain,
            complexity: self.calculate_complexity(raw_text),
            confidence: 0.85,
            constraints,
            success_criteria,
            risk_level,
            created_at: Utc::now(),
        })
    }

    pub fn classify_domain(&self, text: &str) -> String {
        let text_lower = text.to_lowercase();
        let mut scores = HashMap::new();
        
        for (domain, patterns) in &self.domain_patterns {
            let score = patterns.iter()
                .filter(|pattern| text_lower.contains(*pattern))
                .count();
            if score > 0 {
                scores.insert(domain.clone(), score);
            }
        }
        
        scores.into_iter()
            .max_by_key(|(_, score)| *score)
            .map(|(domain, _)| domain)
            .unwrap_or_else(|| "general".to_string())
    }

    pub fn assess_risk(&self, text: &str) -> RiskLevel {
        let text_lower = text.to_lowercase();
        
        // Check in order of severity
        for (risk_level, patterns) in &self.risk_patterns {
            for pattern in patterns {
                if text_lower.contains(pattern) {
                    return risk_level.clone();
                }
            }
        }
        
        RiskLevel::Low
    }

    fn init_patterns(&mut self) {
        // Domain patterns
        self.domain_patterns.insert("infra_deployment".to_string(), vec![
            "deploy".to_string(), "kubernetes".to_string(), "docker".to_string(),
            "container".to_string(), "server".to_string(), "staging".to_string(),
            "production".to_string(), "infrastructure".to_string(),
        ]);
        
        self.domain_patterns.insert("database_admin".to_string(), vec![
            "database".to_string(), "postgres".to_string(), "sql".to_string(),
            "migration".to_string(), "schema".to_string(), "backup".to_string(),
        ]);
        
        self.domain_patterns.insert("marketing_content".to_string(), vec![
            "marketing".to_string(), "content".to_string(), "blog".to_string(),
            "social".to_string(), "email".to_string(), "campaign".to_string(),
        ]);

        // Risk patterns (in order of severity)
        self.risk_patterns.insert(RiskLevel::Critical, vec![
            "production".to_string(), "delete".to_string(), "drop".to_string(),
            "destroy".to_string(), "rm -rf".to_string(),
        ]);
        
        self.risk_patterns.insert(RiskLevel::High, vec![
            "modify".to_string(), "update".to_string(), "migrate".to_string(),
            "change".to_string(),
        ]);
        
        self.risk_patterns.insert(RiskLevel::Medium, vec![
            "create".to_string(), "add".to_string(), "install".to_string(),
            "deploy".to_string(),
        ]);
    }

    fn extract_constraints(&self, text: &str) -> (Vec<String>, Vec<String>) {
        let mut constraints = Vec::new();
        let mut success_criteria = Vec::new();
        
        let text_lower = text.to_lowercase();
        
        if text_lower.contains("blue-green") {
            constraints.push("blue_green_deployment".to_string());
        }
        if text_lower.contains("zero downtime") {
            constraints.push("zero_downtime".to_string());
        }
        
        success_criteria.push("task_completed_successfully".to_string());
        success_criteria.push("no_errors_reported".to_string());
        
        (constraints, success_criteria)
    }

    fn calculate_complexity(&self, text: &str) -> f64 {
        let mut complexity = 0.1;
        
        // Length-based complexity
        complexity += (text.len() as f64) * 0.001;
        
        // Keyword-based complexity
        let complex_keywords = ["migration", "production", "blue-green", "cluster"];
        for keyword in &complex_keywords {
            if text.to_lowercase().contains(keyword) {
                complexity += 0.2;
            }
        }
        
        complexity.min(1.0)
    }

    /// Generate task list for a domain
    pub fn generate_tasks_for_domain(&self, domain: &str, intent: &Intent) -> Result<Vec<ExecutionTask>> {
        let mut tasks = Vec::new();
        
        match domain {
            "infra_deployment" => {
                tasks.push(self.create_task(
                    "analyze_requirements",
                    "Analyze deployment requirements and constraints",
                    TaskType::Sense,
                    "analyzer-agent",
                    vec!["requirements.json"],
                    5,
                    false
                ));
                
                tasks.push(self.create_task(
                    "create_deployment_plan", 
                    "Create detailed deployment plan with rollback strategy",
                    TaskType::Plan,
                    "planner-agent",
                    vec!["deployment-plan.yaml", "rollback-plan.yaml"],
                    10,
                    false
                ));
                
                tasks.push(self.create_task(
                    "execute_deployment",
                    "Execute the deployment with monitoring",
                    TaskType::Execute,
                    "executor-agent", 
                    vec!["deployment-result.json"],
                    15,
                    true
                ));
                
                tasks.push(self.create_task(
                    "verify_deployment",
                    "Verify deployment success and health checks",
                    TaskType::Verify,
                    "verifier-agent",
                    vec!["health-check.json"],
                    5,
                    false
                ));
            },
            
            "database_admin" => {
                tasks.push(self.create_task(
                    "backup_database",
                    "Create full database backup before changes", 
                    TaskType::Sense,
                    "backup-agent",
                    vec!["backup.sql", "backup-manifest.json"],
                    10,
                    false
                ));
                
                tasks.push(self.create_task(
                    "execute_migration",
                    "Execute database migration with transaction safety",
                    TaskType::Execute,
                    "db-agent",
                    vec!["migration-result.json"],
                    20,
                    true
                ));
            },
            
            "marketing_content" => {
                tasks.push(self.create_task(
                    "generate_content",
                    "Generate marketing content with brand compliance",
                    TaskType::Execute,
                    "content-agent",
                    vec!["content.md", "seo-analysis.json"],
                    15,
                    false
                ));
            },
            
            _ => {
                tasks.push(self.create_task(
                    "generic_task",
                    &format!("Execute {} task", domain),
                    TaskType::Execute,
                    "generic-agent",
                    vec!["result.json"],
                    10,
                    true
                ));
            }
        }
        
        Ok(tasks)
    }

    fn create_task(
        &self,
        name: &str,
        description: &str,
        task_type: TaskType,
        agent_type: &str,
        outputs: Vec<&str>,
        duration_minutes: i64,
        dry_run: bool
    ) -> ExecutionTask {
        ExecutionTask {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            task_type,
            agent_type: agent_type.to_string(),
            inputs: HashMap::new(),
            expected_outputs: outputs.iter().map(|s| s.to_string()).collect(),
            estimated_duration: chrono::Duration::minutes(duration_minutes),
            status: TaskStatus::Pending,
            dry_run_first: dry_run,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_intent_parsing() {
        let builder = IntentGraphBuilder::new();
        
        let result = builder.parse_intent("Deploy the marketing website to staging").await;
        assert!(result.is_ok());
        
        let intent = result.unwrap();
        assert_eq!(intent.domain, "infra_deployment");
        assert!(matches!(intent.risk_level, RiskLevel::Medium));
    }

    #[test]
    fn test_domain_classification() {
        let builder = IntentGraphBuilder::new();
        
        assert_eq!(builder.classify_domain("deploy kubernetes app"), "infra_deployment");
        assert_eq!(builder.classify_domain("backup postgres database"), "database_admin");
        assert_eq!(builder.classify_domain("create marketing content"), "marketing_content");
        assert_eq!(builder.classify_domain("random task"), "general");
    }

    #[test]
    fn test_risk_assessment() {
        let builder = IntentGraphBuilder::new();
        
        assert!(matches!(builder.assess_risk("delete production database"), RiskLevel::Critical));
        assert!(matches!(builder.assess_risk("update user profile"), RiskLevel::High));
        assert!(matches!(builder.assess_risk("create new deployment"), RiskLevel::Medium));
        assert!(matches!(builder.assess_risk("read configuration"), RiskLevel::Low));
    }
} 