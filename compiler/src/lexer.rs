//! Talk++ DSL Lexer
//! 
//! Tokenizes Talk++ natural language input into structured tokens

use crate::error::CompilerError;
use logos::Logos;
use serde::{Deserialize, Serialize};

#[derive(Logos, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Keywords
    #[token("if")]
    If,
    
    #[token("then")]
    Then,
    
    #[token("else")]
    Else,
    
    #[token("when")]
    When,
    
    #[token("and")]
    And,
    
    #[token("or")]
    Or,
    
    #[token("using")]
    Using,
    
    #[token("with")]
    With,
    
    #[token("to")]
    To,
    
    #[token("in")]
    In,
    
    #[token("from")]
    From,

    // Action verbs
    #[token("send")]
    #[token("sends")]
    Send,
    
    #[token("store")]
    #[token("stores")]
    Store,
    
    #[token("validate")]
    #[token("validates")]
    Validate,
    
    #[token("process")]
    #[token("processes")]
    Process,
    
    #[token("trigger")]
    #[token("triggers")]
    Trigger,
    
    #[token("call")]
    #[token("calls")]
    Call,

    // Services and resources
    #[regex(r"[A-Z][a-zA-Z0-9]*", |lex| lex.slice().to_owned())]
    Service(String),
    
    // Variables and identifiers
    #[regex(r"[a-z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),
    
    // String literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_owned() // Remove quotes
    })]
    #[regex(r#"`([^`\\]|\\.)*`"#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_owned() // Remove backticks
    })]
    String(String),
    
    // Numbers
    #[regex(r"\d+", |lex| lex.slice().parse::<i64>().unwrap())]
    Integer(i64),
    
    #[regex(r"\d+\.\d+", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    // Punctuation
    #[token(",")]
    Comma,
    
    #[token(".")]
    Dot,
    
    #[token(":")]
    Colon,
    
    #[token(";")]
    Semicolon,

    // Skip whitespace and comments
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    #[error]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: std::ops::Range<usize>,
    pub line: usize,
    pub column: usize,
}

pub fn tokenize(input: &str) -> Result<Vec<TokenWithSpan>, CompilerError> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(input);
    let mut line = 1;
    let mut column = 1;
    let mut last_pos = 0;

    while let Some(token) = lexer.next() {
        let span = lexer.span();
        
        // Update line and column tracking
        let slice = &input[last_pos..span.start];
        for c in slice.chars() {
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        last_pos = span.start;

        match token {
            Token::Error => {
                return Err(CompilerError::lexical(
                    span.start,
                    format!("Invalid token: '{}'", &input[span.clone()]),
                ));
            }
            _ => {
                tokens.push(TokenWithSpan {
                    token,
                    span,
                    line,
                    column,
                });
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenization() {
        let input = r#"if new user registers then validate email using SendGrid"#;
        let tokens = tokenize(input).unwrap();
        
        assert_eq!(tokens[0].token, Token::If);
        assert_eq!(tokens[1].token, Token::Identifier("new".to_string()));
        assert_eq!(tokens[2].token, Token::Identifier("user".to_string()));
        assert_eq!(tokens[3].token, Token::Identifier("registers".to_string()));
        assert_eq!(tokens[4].token, Token::Then);
        assert_eq!(tokens[5].token, Token::Validate);
        assert_eq!(tokens[6].token, Token::Identifier("email".to_string()));
        assert_eq!(tokens[7].token, Token::Using);
        assert_eq!(tokens[8].token, Token::Service("SendGrid".to_string()));
    }

    #[test]
    fn test_string_literals() {
        let input = r#"store user data in table "users""#;
        let tokens = tokenize(input).unwrap();
        
        assert!(matches!(tokens.last().unwrap().token, Token::String(ref s) if s == "users"));
    }

    #[test]
    fn test_error_handling() {
        let input = "if @ invalid";
        let result = tokenize(input);
        assert!(result.is_err());
    }
} 