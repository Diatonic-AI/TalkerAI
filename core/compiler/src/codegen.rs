//! Talk++ DSL Code Generator
//! 
//! Converts parsed AST into executable code for various target languages

use crate::ast::*;
use crate::error::CompilerError;
use crate::{CompilerConfig, TargetLanguage};
use quote::{format_ident, quote};
use syn::Ident;

pub fn generate(program: &Program, config: &CompilerConfig) -> Result<String, CompilerError> {
    match config.target_language {
        TargetLanguage::Rust => generate_rust(program, config),
        TargetLanguage::Python => generate_python(program, config),
        TargetLanguage::JavaScript => generate_javascript(program, config),
        TargetLanguage::TypeScript => generate_typescript(program, config),
        TargetLanguage::Bash => generate_bash(program, config),
    }
}

fn generate_rust(program: &Program, config: &CompilerConfig) -> Result<String, CompilerError> {
    let mut function_bodies = Vec::new();
    
    for statement in &program.statements {
        let code = match statement {
            Statement::Conditional(cond) => generate_rust_conditional(cond)?,
            Statement::Action(action) => generate_rust_action(action)?,
            Statement::Assignment(assign) => generate_rust_assignment(assign)?,
            Statement::Comment(comment) => format!("// {}", comment),
        };
        function_bodies.push(code);
    }

    let handler_body = function_bodies.join("\n    ");
    
    let code = if config.debug_mode {
        format!(
            r#"use anyhow::Result;
use serde::{{Deserialize, Serialize}};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Event {{
    pub data: serde_json::Value,
    pub context: HashMap<String, String>,
}}

#[derive(Debug, Serialize)]
pub struct Response {{
    pub success: bool,
    pub data: serde_json::Value,
    pub message: String,
}}

impl Response {{
    pub fn success(message: impl Into<String>) -> Self {{
        Self {{
            success: true,
            data: serde_json::json!({{}}),
            message: message.into(),
        }}
    }}
    
    pub fn error(message: impl Into<String>) -> Self {{
        Self {{
            success: false,
            data: serde_json::json!({{}}),
            message: message.into(),
        }}
    }}
}}

#[tokio::main]
async fn main() -> Result<()> {{
    tracing_subscriber::init();
    
    let event = Event {{
        data: serde_json::json!({{}}),
        context: HashMap::new(),
    }};
    
    let response = handler(event).await?;
    println!("{{}}", serde_json::to_string_pretty(&response)?);
    
    Ok(())
}}

pub async fn handler(event: Event) -> Result<Response> {{
    tracing::info!("Processing event: {{:?}}", event);
    
    {}
    
    Ok(Response::success("Function executed successfully"))
}}"#,
            handler_body
        )
    } else {
        format!(
            r#"use anyhow::Result;
use serde::{{Deserialize, Serialize}};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Event {{
    pub data: serde_json::Value,
    pub context: HashMap<String, String>,
}}

#[derive(Debug, Serialize)]
pub struct Response {{
    pub success: bool,
    pub data: serde_json::Value,
    pub message: String,
}}

impl Response {{
    pub fn success(message: impl Into<String>) -> Self {{
        Self {{
            success: true,
            data: serde_json::json!({{}}),
            message: message.into(),
        }}
    }}
}}

pub async fn handler(event: Event) -> Result<Response> {{
    {}
    
    Ok(Response::success("Function executed successfully"))
}}"#,
            handler_body
        )
    };

    Ok(code)
}

fn generate_rust_conditional(cond: &ConditionalStatement) -> Result<String, CompilerError> {
    let condition_code = generate_rust_condition(&cond.condition)?;
    
    let then_code = cond
        .then_actions
        .iter()
        .map(generate_rust_action)
        .collect::<Result<Vec<_>, _>>()?
        .join("\n        ");
    
    let else_code = if let Some(else_actions) = &cond.else_actions {
        let else_body = else_actions
            .iter()
            .map(generate_rust_action)
            .collect::<Result<Vec<_>, _>>()?
            .join("\n        ");
        format!(" else {{\n        {}\n    }}", else_body)
    } else {
        String::new()
    };

    Ok(format!(
        "if {} {{\n        {}\n    }}{}",
        condition_code, then_code, else_code
    ))
}

fn generate_rust_condition(condition: &Condition) -> Result<String, CompilerError> {
    match condition {
        Condition::Event(event) => {
            // For event conditions, we'll check the event data
            Ok(format!(
                r#"event.data.get("type").and_then(|v| v.as_str()) == Some("{}")"#,
                format!("{}_{}", event.subject.replace(" ", "_"), event.action)
            ))
        }
        Condition::Comparison(comp) => {
            let left = generate_rust_expression(&comp.left)?;
            let right = generate_rust_expression(&comp.right)?;
            let op = match comp.operator {
                ComparisonOperator::Equal => "==",
                ComparisonOperator::NotEqual => "!=",
                ComparisonOperator::GreaterThan => ">",
                ComparisonOperator::LessThan => "<",
                ComparisonOperator::GreaterEqual => ">=",
                ComparisonOperator::LessEqual => "<=",
            };
            Ok(format!("{} {} {}", left, op, right))
        }
        Condition::Logical(logical) => {
            let left = generate_rust_condition(&logical.left)?;
            let right = generate_rust_condition(&logical.right)?;
            let op = match logical.operator {
                LogicalOperator::And => "&&",
                LogicalOperator::Or => "||",
            };
            Ok(format!("({}) {} ({})", left, op, right))
        }
    }
}

fn generate_rust_action(action: &ActionStatement) -> Result<String, CompilerError> {
    let service_code = if let Some(service) = &action.service {
        match service.name.to_lowercase().as_str() {
            "sendgrid" => generate_sendgrid_call(action)?,
            "twilio" => generate_twilio_call(action)?,
            "postgresql" | "postgres" => generate_postgres_call(action)?,
            _ => format!(r#"tracing::warn!("Service {} not implemented", "{}"); // TODO: Implement {}"#, service.name, service.name),
        }
    } else {
        match action.action {
            Action::Send => "// Send action".to_string(),
            Action::Store => "// Store action".to_string(),
            Action::Validate => "// Validate action".to_string(),
            Action::Process => "// Process action".to_string(),
            Action::Trigger => "// Trigger action".to_string(),
            Action::Call => "// Call action".to_string(),
            Action::Custom(ref name) => format!("// Custom action: {}", name),
        }
    };

    Ok(service_code)
}

fn generate_sendgrid_call(action: &ActionStatement) -> Result<String, CompilerError> {
    Ok(format!(
        r#"// SendGrid email service call
tracing::info!("Sending email via SendGrid");
// TODO: Implement actual SendGrid API call
let email_result = send_email_sendgrid().await;
if let Err(e) = email_result {{
    tracing::error!("Failed to send email: {{}}", e);
    return Ok(Response::error("Failed to send email"));
}}"#
    ))
}

fn generate_twilio_call(action: &ActionStatement) -> Result<String, CompilerError> {
    Ok(format!(
        r#"// Twilio SMS service call
tracing::info!("Sending SMS via Twilio");
// TODO: Implement actual Twilio API call
let sms_result = send_sms_twilio().await;
if let Err(e) = sms_result {{
    tracing::error!("Failed to send SMS: {{}}", e);
    return Ok(Response::error("Failed to send SMS"));
}}"#
    ))
}

fn generate_postgres_call(action: &ActionStatement) -> Result<String, CompilerError> {
    Ok(format!(
        r#"// PostgreSQL database operation
tracing::info!("Executing database operation");
// TODO: Implement actual PostgreSQL query
let db_result = execute_postgres_query().await;
if let Err(e) = db_result {{
    tracing::error!("Database operation failed: {{}}", e);
    return Ok(Response::error("Database operation failed"));
}}"#
    ))
}

fn generate_rust_assignment(assign: &AssignmentStatement) -> Result<String, CompilerError> {
    let value = generate_rust_expression(&assign.value)?;
    Ok(format!("let {} = {};", assign.variable, value))
}

fn generate_rust_expression(expr: &Expression) -> Result<String, CompilerError> {
    match expr {
        Expression::Identifier(name) => Ok(name.clone()),
        Expression::String(value) => Ok(format!(r#""{}""#, value)),
        Expression::Integer(value) => Ok(value.to_string()),
        Expression::Float(value) => Ok(value.to_string()),
        Expression::Boolean(value) => Ok(value.to_string()),
        Expression::Property(prop) => {
            let object = generate_rust_expression(&prop.object)?;
            Ok(format!("{}.{}", object, prop.property))
        }
        Expression::FunctionCall(call) => {
            let args = call
                .arguments
                .iter()
                .map(generate_rust_expression)
                .collect::<Result<Vec<_>, _>>()?
                .join(", ");
            Ok(format!("{}({})", call.name, args))
        }
    }
}

fn generate_python(program: &Program, _config: &CompilerConfig) -> Result<String, CompilerError> {
    let mut code_lines = vec![
        "#!/usr/bin/env python3".to_string(),
        "import json".to_string(),
        "import logging".to_string(),
        "from typing import Dict, Any".to_string(),
        "".to_string(),
        "logging.basicConfig(level=logging.INFO)".to_string(),
        "logger = logging.getLogger(__name__)".to_string(),
        "".to_string(),
        "def handler(event: Dict[str, Any]) -> Dict[str, Any]:".to_string(),
        "    logger.info(f'Processing event: {event}')".to_string(),
        "".to_string(),
    ];

    for statement in &program.statements {
        let line = match statement {
            Statement::Action(_) => "    # TODO: Implement action".to_string(),
            Statement::Conditional(_) => "    # TODO: Implement conditional".to_string(),
            Statement::Assignment(assign) => {
                format!("    {} = None  # TODO: Implement assignment", assign.variable)
            }
            Statement::Comment(comment) => format!("    # {}", comment),
        };
        code_lines.push(line);
    }

    code_lines.extend([
        "".to_string(),
        "    return {'success': True, 'message': 'Function executed successfully'}".to_string(),
        "".to_string(),
        "if __name__ == '__main__':".to_string(),
        "    result = handler({})".to_string(),
        "    print(json.dumps(result, indent=2))".to_string(),
    ]);

    Ok(code_lines.join("\n"))
}

fn generate_javascript(program: &Program, _config: &CompilerConfig) -> Result<String, CompilerError> {
    let mut code_lines = vec![
        "// Generated Talk++ JavaScript function".to_string(),
        "".to_string(),
        "async function handler(event) {".to_string(),
        "    console.log('Processing event:', event);".to_string(),
        "".to_string(),
    ];

    for statement in &program.statements {
        let line = match statement {
            Statement::Action(_) => "    // TODO: Implement action".to_string(),
            Statement::Conditional(_) => "    // TODO: Implement conditional".to_string(),
            Statement::Assignment(assign) => {
                format!("    let {} = null; // TODO: Implement assignment", assign.variable)
            }
            Statement::Comment(comment) => format!("    // {}", comment),
        };
        code_lines.push(line);
    }

    code_lines.extend([
        "".to_string(),
        "    return { success: true, message: 'Function executed successfully' };".to_string(),
        "}".to_string(),
        "".to_string(),
        "// Export for Node.js".to_string(),
        "if (typeof module !== 'undefined' && module.exports) {".to_string(),
        "    module.exports = { handler };".to_string(),
        "}".to_string(),
    ]);

    Ok(code_lines.join("\n"))
}

fn generate_typescript(program: &Program, _config: &CompilerConfig) -> Result<String, CompilerError> {
    let mut code_lines = vec![
        "// Generated Talk++ TypeScript function".to_string(),
        "".to_string(),
        "interface Event {".to_string(),
        "    data: any;".to_string(),
        "    context: Record<string, string>;".to_string(),
        "}".to_string(),
        "".to_string(),
        "interface Response {".to_string(),
        "    success: boolean;".to_string(),
        "    data?: any;".to_string(),
        "    message: string;".to_string(),
        "}".to_string(),
        "".to_string(),
        "export async function handler(event: Event): Promise<Response> {".to_string(),
        "    console.log('Processing event:', event);".to_string(),
        "".to_string(),
    ];

    for statement in &program.statements {
        let line = match statement {
            Statement::Action(_) => "    // TODO: Implement action".to_string(),
            Statement::Conditional(_) => "    // TODO: Implement conditional".to_string(),
            Statement::Assignment(assign) => {
                format!("    const {}: any = null; // TODO: Implement assignment", assign.variable)
            }
            Statement::Comment(comment) => format!("    // {}", comment),
        };
        code_lines.push(line);
    }

    code_lines.extend([
        "".to_string(),
        "    return { success: true, message: 'Function executed successfully' };".to_string(),
        "}".to_string(),
    ]);

    Ok(code_lines.join("\n"))
}

fn generate_bash(program: &Program, _config: &CompilerConfig) -> Result<String, CompilerError> {
    let mut code_lines = vec![
        "#!/bin/bash".to_string(),
        "# Generated Talk++ Bash script".to_string(),
        "".to_string(),
        "set -euo pipefail".to_string(),
        "".to_string(),
        "handler() {".to_string(),
        "    local event=\"$1\"".to_string(),
        "    echo \"Processing event: $event\" >&2".to_string(),
        "".to_string(),
    ];

    for statement in &program.statements {
        let line = match statement {
            Statement::Action(_) => "    # TODO: Implement action".to_string(),
            Statement::Conditional(_) => "    # TODO: Implement conditional".to_string(),
            Statement::Assignment(assign) => {
                format!("    {}=''  # TODO: Implement assignment", assign.variable)
            }
            Statement::Comment(comment) => format!("    # {}", comment),
        };
        code_lines.push(line);
    }

    code_lines.extend([
        "".to_string(),
        "    echo '{\"success\": true, \"message\": \"Function executed successfully\"}'".to_string(),
        "}".to_string(),
        "".to_string(),
        "# Main execution".to_string(),
        "if [[ \"${BASH_SOURCE[0]}\" == \"${0}\" ]]; then".to_string(),
        "    handler \"{}\"".to_string(),
        "fi".to_string(),
    ]);

    Ok(code_lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompilerConfig, TargetLanguage, OptimizationLevel};
    use crate::lexer::tokenize;
    use crate::parser::parse;

    #[test]
    fn test_rust_generation() {
        let input = "if new user registers then validate email using SendGrid";
        let tokens = tokenize(input).unwrap();
        let ast = parse(tokens).unwrap();
        
        let config = CompilerConfig {
            target_language: TargetLanguage::Rust,
            optimization_level: OptimizationLevel::Debug,
            debug_mode: true,
        };
        
        let code = generate(&ast, &config).unwrap();
        assert!(code.contains("async fn handler"));
        assert!(code.contains("SendGrid"));
    }

    #[test]
    fn test_python_generation() {
        let input = "send welcome message";
        let tokens = tokenize(input).unwrap();
        let ast = parse(tokens).unwrap();
        
        let config = CompilerConfig {
            target_language: TargetLanguage::Python,
            optimization_level: OptimizationLevel::Debug,
            debug_mode: false,
        };
        
        let code = generate(&ast, &config).unwrap();
        assert!(code.contains("def handler"));
        assert!(code.contains("#!/usr/bin/env python3"));
    }
} 