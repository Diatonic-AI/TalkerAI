//! Talk++ DSL Parser
//! 
//! Converts tokenized Talk++ input into an Abstract Syntax Tree

use crate::ast::*;
use crate::error::CompilerError;
use crate::lexer::{Token, TokenWithSpan};
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<TokenWithSpan>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, CompilerError> {
        let mut program = Program::new();

        while !self.is_at_end() {
            if let Some(statement) = self.parse_statement()? {
                program.add_statement(statement);
            }
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, CompilerError> {
        if self.is_at_end() {
            return Ok(None);
        }

        match &self.peek().token {
            Token::If | Token::When => {
                let conditional = self.parse_conditional()?;
                Ok(Some(Statement::Conditional(conditional)))
            }
            Token::Send | Token::Store | Token::Validate | Token::Process | Token::Trigger | Token::Call => {
                let action = self.parse_action()?;
                Ok(Some(Statement::Action(action)))
            }
            Token::Identifier(_) => {
                // Could be assignment or action
                if self.peek_ahead(1).map(|t| &t.token) == Some(&Token::Colon) {
                    let assignment = self.parse_assignment()?;
                    Ok(Some(Statement::Assignment(assignment)))
                } else {
                    let action = self.parse_action()?;
                    Ok(Some(Statement::Action(action)))
                }
            }
            _ => {
                self.advance();
                Ok(None)
            }
        }
    }

    fn parse_conditional(&mut self) -> Result<ConditionalStatement, CompilerError> {
        // Consume 'if' or 'when'
        self.advance();

        let condition = self.parse_condition()?;

        // Expect 'then'
        if !self.check(&Token::Then) {
            return Err(self.error("Expected 'then' after condition"));
        }
        self.advance();

        let mut then_actions = Vec::new();
        
        // Parse actions until we hit 'else' or end
        while !self.is_at_end() && !self.check(&Token::Else) {
            if let Some(Statement::Action(action)) = self.parse_statement()? {
                then_actions.push(action);
            } else {
                break;
            }
        }

        let else_actions = if self.check(&Token::Else) {
            self.advance(); // consume 'else'
            let mut actions = Vec::new();
            
            while !self.is_at_end() {
                if let Some(Statement::Action(action)) = self.parse_statement()? {
                    actions.push(action);
                } else {
                    break;
                }
            }
            
            Some(actions)
        } else {
            None
        };

        Ok(ConditionalStatement {
            condition,
            then_actions,
            else_actions,
        })
    }

    fn parse_condition(&mut self) -> Result<Condition, CompilerError> {
        let mut condition = self.parse_primary_condition()?;

        while self.check(&Token::And) || self.check(&Token::Or) {
            let operator = if self.check(&Token::And) {
                LogicalOperator::And
            } else {
                LogicalOperator::Or
            };
            self.advance();

            let right = self.parse_primary_condition()?;
            condition = Condition::Logical(LogicalCondition {
                left: Box::new(condition),
                operator,
                right: Box::new(right),
            });
        }

        Ok(condition)
    }

    fn parse_primary_condition(&mut self) -> Result<Condition, CompilerError> {
        // Parse event conditions like "new user registers"
        if self.check_identifier() {
            let mut parts = Vec::new();
            
            while self.check_identifier() {
                if let Token::Identifier(id) = &self.peek().token {
                    parts.push(id.clone());
                    self.advance();
                } else {
                    break;
                }
            }

            if parts.len() >= 2 {
                let subject = parts[0..parts.len()-1].join(" ");
                let action = parts.last().unwrap().clone();
                
                let context = if self.check(&Token::In) || self.check(&Token::From) || self.check(&Token::To) {
                    self.advance(); // consume preposition
                    if let Token::String(s) = &self.peek().token {
                        let ctx = s.clone();
                        self.advance();
                        Some(ctx)
                    } else if let Token::Identifier(s) = &self.peek().token {
                        let ctx = s.clone();
                        self.advance();
                        Some(ctx)
                    } else {
                        None
                    }
                } else {
                    None
                };

                return Ok(Condition::Event(EventCondition {
                    subject,
                    action,
                    context,
                }));
            }
        }

        Err(self.error("Expected condition"))
    }

    fn parse_action(&mut self) -> Result<ActionStatement, CompilerError> {
        let action = if let Token::Identifier(verb) = &self.peek().token {
            let action = Action::from_str(verb);
            self.advance();
            action
        } else {
            match &self.peek().token {
                Token::Send => { self.advance(); Action::Send }
                Token::Store => { self.advance(); Action::Store }
                Token::Validate => { self.advance(); Action::Validate }
                Token::Process => { self.advance(); Action::Process }
                Token::Trigger => { self.advance(); Action::Trigger }
                Token::Call => { self.advance(); Action::Call }
                _ => return Err(self.error("Expected action verb")),
            }
        };

        // Parse target (what to act on)
        let target = if self.check_identifier() || self.check(&Token::String("".to_string())) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Parse service call (using X, with Y)
        let service = if self.check(&Token::Using) || self.check(&Token::With) {
            self.advance(); // consume 'using' or 'with'
            
            if let Token::Service(name) = &self.peek().token {
                let name = name.clone();
                self.advance();
                Some(ServiceCall {
                    name,
                    method: None,
                    config: HashMap::new(),
                })
            } else {
                None
            }
        } else {
            None
        };

        Ok(ActionStatement {
            action,
            target,
            service,
            parameters: HashMap::new(),
        })
    }

    fn parse_assignment(&mut self) -> Result<AssignmentStatement, CompilerError> {
        let variable = if let Token::Identifier(name) = &self.peek().token {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(self.error("Expected variable name"));
        };

        // Consume ':'
        if !self.check(&Token::Colon) {
            return Err(self.error("Expected ':' after variable name"));
        }
        self.advance();

        let value = self.parse_expression()?;

        Ok(AssignmentStatement { variable, value })
    }

    fn parse_expression(&mut self) -> Result<Expression, CompilerError> {
        match &self.peek().token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::identifier(name))
            }
            Token::String(value) => {
                let value = value.clone();
                self.advance();
                Ok(Expression::string(value))
            }
            Token::Integer(value) => {
                let value = *value;
                self.advance();
                Ok(Expression::integer(value))
            }
            Token::Float(value) => {
                let value = *value;
                self.advance();
                Ok(Expression::float(value))
            }
            Token::Service(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::identifier(name))
            }
            _ => Err(self.error("Expected expression")),
        }
    }

    // Helper methods
    fn peek(&self) -> &TokenWithSpan {
        &self.tokens[self.current]
    }

    fn peek_ahead(&self, offset: usize) -> Option<&TokenWithSpan> {
        self.tokens.get(self.current + offset)
    }

    fn advance(&mut self) -> &TokenWithSpan {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &TokenWithSpan {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token) == std::mem::discriminant(token)
        }
    }

    fn check_identifier(&self) -> bool {
        matches!(self.peek().token, Token::Identifier(_))
    }

    fn error(&self, message: &str) -> CompilerError {
        let token = self.peek();
        CompilerError::parse(token.line, token.column, message)
    }
}

pub fn parse(tokens: Vec<TokenWithSpan>) -> Result<Program, CompilerError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_simple_conditional() {
        let input = "if new user registers then validate email using SendGrid";
        let tokens = tokenize(input).unwrap();
        let ast = parse(tokens).unwrap();

        assert_eq!(ast.statements.len(), 1);
        if let Statement::Conditional(cond) = &ast.statements[0] {
            assert!(matches!(cond.condition, Condition::Event(_)));
            assert_eq!(cond.then_actions.len(), 1);
        } else {
            panic!("Expected conditional statement");
        }
    }

    #[test]
    fn test_action_statement() {
        let input = "send welcome message using Twilio";
        let tokens = tokenize(input).unwrap();
        let ast = parse(tokens).unwrap();

        assert_eq!(ast.statements.len(), 1);
        if let Statement::Action(action) = &ast.statements[0] {
            assert!(matches!(action.action, Action::Send));
            assert!(action.service.is_some());
        } else {
            panic!("Expected action statement");
        }
    }
} 