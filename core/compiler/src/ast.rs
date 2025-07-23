//! Abstract Syntax Tree for Talk++ DSL

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Conditional(ConditionalStatement),
    Action(ActionStatement),
    Assignment(AssignmentStatement),
    Comment(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalStatement {
    pub condition: Condition,
    pub then_actions: Vec<ActionStatement>,
    pub else_actions: Option<Vec<ActionStatement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    Event(EventCondition),
    Comparison(ComparisonCondition),
    Logical(LogicalCondition),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCondition {
    pub subject: String,
    pub action: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonCondition {
    pub left: Expression,
    pub operator: ComparisonOperator,
    pub right: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalCondition {
    pub left: Box<Condition>,
    pub operator: LogicalOperator,
    pub right: Box<Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionStatement {
    pub action: Action,
    pub target: Option<Expression>,
    pub service: Option<ServiceCall>,
    pub parameters: HashMap<String, Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Send,
    Store,
    Validate,
    Process,
    Trigger,
    Call,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCall {
    pub name: String,
    pub method: Option<String>,
    pub config: HashMap<String, Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentStatement {
    pub variable: String,
    pub value: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Property(PropertyAccess),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyAccess {
    pub object: Box<Expression>,
    pub property: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Action {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "send" | "sends" => Action::Send,
            "store" | "stores" => Action::Store,
            "validate" | "validates" => Action::Validate,
            "process" | "processes" => Action::Process,
            "trigger" | "triggers" => Action::Trigger,
            "call" | "calls" => Action::Call,
            _ => Action::Custom(s.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Action::Send => "send".to_string(),
            Action::Store => "store".to_string(),
            Action::Validate => "validate".to_string(),
            Action::Process => "process".to_string(),
            Action::Trigger => "trigger".to_string(),
            Action::Call => "call".to_string(),
            Action::Custom(s) => s.clone(),
        }
    }
}

impl Expression {
    pub fn identifier(name: impl Into<String>) -> Self {
        Expression::Identifier(name.into())
    }

    pub fn string(value: impl Into<String>) -> Self {
        Expression::String(value.into())
    }

    pub fn integer(value: i64) -> Self {
        Expression::Integer(value)
    }

    pub fn float(value: f64) -> Self {
        Expression::Float(value)
    }

    pub fn boolean(value: bool) -> Self {
        Expression::Boolean(value)
    }
} 