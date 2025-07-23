//! Talk++ Compiler CLI (talkppc)
//! 
//! Command-line interface for compiling Talk++ DSL to target languages.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;
use talkpp_compiler::{Compiler, CompilerConfig, TargetLanguage, OptimizationLevel};

#[derive(Parser)]
#[command(name = "talkppc")]
#[command(about = "Talk++ Compiler - Compile Talk++ DSL to executable code")]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Talk++ source file
    Build {
        /// Input Talk++ source file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Target language
        #[arg(short, long, default_value = "rust")]
        target: String,
        
        /// Optimization level
        #[arg(long, default_value = "debug")]
        optimization: String,
        
        /// Enable debug mode
        #[arg(long)]
        debug: bool,
    },
    
    /// Validate Talk++ syntax
    Check {
        /// Input Talk++ source file
        #[arg(short, long)]
        input: PathBuf,
    },
    
    /// Show compiler version and supported languages
    Info,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Build { input, output, target, optimization, debug } => {
            build_command(input, output, target, optimization, debug).await
        }
        Commands::Check { input } => {
            check_command(input).await
        }
        Commands::Info => {
            info_command()
        }
    }
}

async fn build_command(
    input: PathBuf,
    output: Option<PathBuf>,
    target: String,
    optimization: String,
    debug: bool,
) -> Result<()> {
    println!("{} Compiling Talk++ source: {}", "Building".green().bold(), input.display());
    
    // Read source file
    let source = std::fs::read_to_string(&input)?;
    
    // Parse target language
    let target_language = match target.to_lowercase().as_str() {
        "rust" => TargetLanguage::Rust,
        "python" => TargetLanguage::Python,
        "javascript" | "js" => TargetLanguage::JavaScript,
        "typescript" | "ts" => TargetLanguage::TypeScript,
        "bash" => TargetLanguage::Bash,
        _ => return Err(anyhow::anyhow!("Unsupported target language: {}", target)),
    };
    
    // Parse optimization level
    let optimization_level = match optimization.to_lowercase().as_str() {
        "debug" => OptimizationLevel::Debug,
        "release" => OptimizationLevel::Release,
        "size" => OptimizationLevel::Size,
        _ => return Err(anyhow::anyhow!("Invalid optimization level: {}", optimization)),
    };
    
    // Create compiler config
    let config = CompilerConfig {
        target_language,
        optimization_level,
        debug_mode: debug,
    };
    
    // Compile the source
    let compiler = Compiler::with_config(config);
    let compiled_code = compiler.compile(&source)?;
    
    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension(match target_language {
            TargetLanguage::Rust => "rs",
            TargetLanguage::Python => "py",
            TargetLanguage::JavaScript => "js",
            TargetLanguage::TypeScript => "ts",
            TargetLanguage::Bash => "sh",
        });
        path
    });
    
    // Write compiled code
    std::fs::write(&output_path, compiled_code)?;
    
    println!("{} Compilation completed: {}", "Success".green().bold(), output_path.display());
    
    Ok(())
}

async fn check_command(input: PathBuf) -> Result<()> {
    println!("{} Checking Talk++ syntax: {}", "Checking".yellow().bold(), input.display());
    
    let source = std::fs::read_to_string(&input)?;
    let compiler = Compiler::new();
    
    match compiler.compile(&source) {
        Ok(_) => {
            println!("{} Syntax is valid", "Success".green().bold());
        }
        Err(e) => {
            println!("{} Syntax error: {}", "Error".red().bold(), e);
            return Err(e);
        }
    }
    
    Ok(())
}

fn info_command() -> Result<()> {
    println!("{}", "Talk++ Compiler Information".blue().bold());
    println!("Version: 0.2.0");
    println!("Supported target languages:");
    println!("  • Rust");
    println!("  • Python"); 
    println!("  • JavaScript");
    println!("  • TypeScript");
    println!("  • Bash");
    
    Ok(())
} 