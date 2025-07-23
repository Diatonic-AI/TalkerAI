use anyhow::Result;
use uuid::Uuid;
use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize};

use crate::{Intent, IntentExecutionPlan, ExecutionTask, TaskDependency, Checkpoint, RollbackPlan, RollbackStep, DependencyType};

/// Adaptive planner that creates and modifies execution plans
#[derive(Debug)]
pub struct AdaptivePlanner {
    planning_strategies: std::collections::HashMap<String, PlanningStrategy>,
}

impl AdaptivePlanner {
    pub fn new() -> Self {
        let mut planner = Self {
            planning_strategies: std::collections::HashMap::new(),
        };
        
        planner.init_strategies();
        planner
    }

    /// Create execution plan from tasks and intent
    pub async fn create_execution_plan(
        &self,
        tasks: Vec<ExecutionTask>,
        intent: &Intent
    ) -> Result<IntentExecutionPlan> {
        let strategy = self.planning_strategies.get(&intent.domain)
            .unwrap_or(&self.default_strategy());

        // Build task dependencies
        let dependencies = self.build_dependencies(&tasks)?;
        
        // Calculate estimated duration
        let estimated_duration = self.calculate_duration(&tasks, &dependencies)?;
        
        // Determine autonomy tier
        let autonomy_tier = self.determine_autonomy_tier(intent)?;
        
        // Create checkpoints
        let checkpoints = self.create_checkpoints(&tasks, strategy)?;
        
        // Generate rollback plan
        let rollback_plan = self.generate_rollback_plan(&tasks)?;
        
        Ok(IntentExecutionPlan {
            id: Uuid::new_v4(),
            intent_id: intent.id,
            tasks,
            dependencies,
            estimated_duration,
            autonomy_tier,
            checkpoints,
            rollback_plan: Some(rollback_plan),
            created_at: Utc::now(),
        })
    }

    fn init_strategies(&mut self) {
        // Infrastructure deployment strategy
        self.planning_strategies.insert("infra_deployment".to_string(), PlanningStrategy {
            name: "infrastructure".to_string(),
            checkpoint_frequency: CheckpointFrequency::PerPhase,
            dry_run_mandatory: true,
            parallel_limit: 2,
        });
        
        // Database strategy  
        self.planning_strategies.insert("database_admin".to_string(), PlanningStrategy {
            name: "database".to_string(),
            checkpoint_frequency: CheckpointFrequency::PerTask,
            dry_run_mandatory: true,
            parallel_limit: 1,
        });
        
        // Marketing strategy
        self.planning_strategies.insert("marketing_content".to_string(), PlanningStrategy {
            name: "marketing".to_string(),
            checkpoint_frequency: CheckpointFrequency::Major,
            dry_run_mandatory: false,
            parallel_limit: 3,
        });
    }

    fn default_strategy(&self) -> PlanningStrategy {
        PlanningStrategy {
            name: "default".to_string(),
            checkpoint_frequency: CheckpointFrequency::PerPhase,
            dry_run_mandatory: false,
            parallel_limit: 2,
        }
    }

    fn build_dependencies(&self, tasks: &[ExecutionTask]) -> Result<Vec<TaskDependency>> {
        let mut dependencies = Vec::new();
        
        // Simple sequential dependencies for now
        for i in 0..tasks.len().saturating_sub(1) {
            dependencies.push(TaskDependency {
                from_task: tasks[i].id,
                to_task: tasks[i + 1].id,
                dependency_type: DependencyType::Sequential,
            });
        }
        
        Ok(dependencies)
    }

    fn calculate_duration(&self, tasks: &[ExecutionTask], _dependencies: &[TaskDependency]) -> Result<Duration> {
        let total_minutes: i64 = tasks.iter()
            .map(|t| t.estimated_duration.num_minutes())
            .sum();
        
        // Apply parallelization factor
        let parallel_factor = 0.7;
        let estimated_minutes = (total_minutes as f64 * parallel_factor) as i64;
        
        Ok(Duration::minutes(estimated_minutes))
    }

    fn determine_autonomy_tier(&self, intent: &Intent) -> Result<u8> {
        let mut tier = 2u8; // Default operator tier
        
        // Adjust based on risk level
        match intent.risk_level {
            crate::RiskLevel::Low => tier = tier.max(3),
            crate::RiskLevel::Medium => tier = tier.max(2),
            crate::RiskLevel::High => tier = tier.min(2),
            crate::RiskLevel::Critical => tier = tier.min(1),
        }
        
        // Adjust based on complexity
        if intent.complexity > 0.8 {
            tier = tier.min(2);
        }
        
        Ok(tier)
    }

    fn create_checkpoints(&self, tasks: &[ExecutionTask], strategy: &PlanningStrategy) -> Result<Vec<Checkpoint>> {
        let mut checkpoints = Vec::new();
        
        match strategy.checkpoint_frequency {
            CheckpointFrequency::PerTask => {
                for task in tasks {
                    if task.dry_run_first {
                        checkpoints.push(Checkpoint {
                            task_id: task.id,
                            description: format!("Review dry-run results for {}", task.name),
                            requires_approval: true,
                            auto_rollback_on_fail: true,
                        });
                    }
                }
            },
            CheckpointFrequency::PerPhase => {
                let execute_tasks: Vec<_> = tasks.iter()
                    .filter(|t| matches!(t.task_type, crate::TaskType::Execute))
                    .collect();
                
                for task in execute_tasks {
                    checkpoints.push(Checkpoint {
                        task_id: task.id,
                        description: format!("Phase checkpoint: {}", task.name),
                        requires_approval: false,
                        auto_rollback_on_fail: true,
                    });
                }
            },
            CheckpointFrequency::Major => {
                if let Some(last_task) = tasks.last() {
                    checkpoints.push(Checkpoint {
                        task_id: last_task.id,
                        description: "Final review before completion".to_string(),
                        requires_approval: true,
                        auto_rollback_on_fail: false,
                    });
                }
            },
        }
        
        Ok(checkpoints)
    }

    fn generate_rollback_plan(&self, tasks: &[ExecutionTask]) -> Result<RollbackPlan> {
        let mut rollback_steps = Vec::new();
        
        // Generate rollback steps in reverse order
        for task in tasks.iter().rev() {
            if matches!(task.task_type, crate::TaskType::Execute) {
                rollback_steps.push(RollbackStep {
                    id: Uuid::new_v4(),
                    description: format!("Rollback {}", task.name),
                    command: format!("rollback_{}", task.name.replace(' ', "_")),
                    verification: format!("verify_rollback_{}", task.name.replace(' ', "_")),
                });
            }
        }
        
        Ok(RollbackPlan {
            steps: rollback_steps,
            auto_trigger_conditions: vec![
                "health_check_failure".to_string(),
                "manual_trigger".to_string(),
            ],
        })
    }
}

/// Planning strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningStrategy {
    pub name: String,
    pub checkpoint_frequency: CheckpointFrequency,
    pub dry_run_mandatory: bool,
    pub parallel_limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckpointFrequency {
    PerTask,
    PerPhase,
    Major,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RiskLevel, TaskType, TaskStatus};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_adaptive_planner_creation() {
        let planner = AdaptivePlanner::new();
        assert!(planner.planning_strategies.contains_key("infra_deployment"));
    }

    #[test]
    fn test_autonomy_tier_determination() {
        let planner = AdaptivePlanner::new();
        
        let low_risk_intent = Intent {
            id: Uuid::new_v4(),
            raw_text: "test".to_string(),
            structured_goal: "test".to_string(),
            domain: "test".to_string(),
            complexity: 0.2,
            confidence: 0.9,
            constraints: vec![],
            success_criteria: vec![],
            risk_level: RiskLevel::Low,
            created_at: Utc::now(),
        };
        
        let tier = planner.determine_autonomy_tier(&low_risk_intent).unwrap();
        assert!(tier >= 3);
    }
} 