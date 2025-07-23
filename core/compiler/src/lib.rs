//! Talk++ DSL Compiler
//! 
//! This crate provides the core compilation functionality for Talk++,
//! converting natural language DSL into executable Rust code.

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod codegen;
pub mod error;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Main compiler interface
pub struct Compiler {
    config: CompilerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    pub target_language: TargetLanguage,
    pub optimization_level: OptimizationLevel,
    pub debug_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Bash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    Debug,
    Release,
    Size,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            target_language: TargetLanguage::Rust,
            optimization_level: OptimizationLevel::Debug,
            debug_mode: true,
        }
    }
}

impl Compiler {
    /// Create a new compiler instance with default configuration
    pub fn new() -> Self {
        Self {
            config: CompilerConfig::default(),
        }
    }

    /// Create a new compiler instance with custom configuration
    pub fn with_config(config: CompilerConfig) -> Self {
        Self { config }
    }

    /// Compile Talk++ DSL source code to target language
    pub fn compile(&self, source: &str) -> Result<String> {
        // Parse the source into tokens
        let tokens = lexer::tokenize(source)?;
        
        // Parse tokens into AST
        let ast = parser::parse(tokens)?;
        
        // Generate code from AST
        let code = codegen::generate(&ast, &self.config)?;
        
        Ok(code)
    }

    /// Compile and validate the generated code
    pub fn compile_and_validate(&self, source: &str) -> Result<String> {
        let code = self.compile(source)?;
        
        // TODO: Add validation logic
        
        Ok(code)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
} 