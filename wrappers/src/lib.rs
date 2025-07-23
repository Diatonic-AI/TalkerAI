//! Talk++ Language Wrappers
//! 
//! This crate provides language-specific wrappers for executing code
//! in different programming languages.

pub mod python;
pub mod javascript;
pub mod bash;
pub mod rust;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Language wrapper trait
pub trait LanguageWrapper {
    /// Execute code in the target language
    async fn execute(&self, code: &str, args: &[String]) -> Result<String>;
    
    /// Validate code syntax
    fn validate(&self, code: &str) -> Result<()>;
    
    /// Get language version
    fn version(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Bash,
    Rust,
    Go,
    Java,
    CSharp,
}

/// Wrapper factory for creating language-specific wrappers
pub struct WrapperFactory;

impl WrapperFactory {
    /// Create a wrapper for the specified language
    pub fn create_wrapper(language: Language) -> Result<Box<dyn LanguageWrapper + Send + Sync>> {
        match language {
            Language::Python => Ok(Box::new(python::PythonWrapper::new()?)),
            Language::JavaScript => Ok(Box::new(javascript::JavaScriptWrapper::new()?)),
            Language::Bash => Ok(Box::new(bash::BashWrapper::new()?)),
            Language::Rust => Ok(Box::new(rust::RustWrapper::new()?)),
            _ => Err(anyhow::anyhow!("Language not supported: {:?}", language)),
        }
    }

    /// Get all supported languages
    pub fn supported_languages() -> Vec<Language> {
        vec![
            Language::Python,
            Language::JavaScript,
            Language::Bash,
            Language::Rust,
        ]
    }
} 