//! Compiler error types and handling

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Lexical error at position {position}: {message}")]
    LexicalError { position: usize, message: String },

    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Semantic error: {message}")]
    SemanticError { message: String },

    #[error("Code generation error: {message}")]
    CodeGenError { message: String },

    #[error("Unsupported feature: {feature}")]
    UnsupportedFeature { feature: String },

    #[error("Internal compiler error: {message}")]
    InternalError { message: String },

    #[error("IO error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}

impl CompilerError {
    pub fn lexical(position: usize, message: impl Into<String>) -> Self {
        Self::LexicalError {
            position,
            message: message.into(),
        }
    }

    pub fn parse(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::ParseError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn semantic(message: impl Into<String>) -> Self {
        Self::SemanticError {
            message: message.into(),
        }
    }

    pub fn codegen(message: impl Into<String>) -> Self {
        Self::CodeGenError {
            message: message.into(),
        }
    }

    pub fn unsupported(feature: impl Into<String>) -> Self {
        Self::UnsupportedFeature {
            feature: feature.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }
} 