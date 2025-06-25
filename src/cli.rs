use crate::{discover_models, open, LlmError, config};
use clap::{Parser, Subcommand, Args};
use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "agentd")]
#[command(about = "A simple interface for local LLM interaction")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate text using a model
    Generate(GenerateArgs),
    /// List available models
    List,
    /// Download a model
    Download(DownloadArgs),
    /// Show model information
    Info(InfoArgs),
}

#[derive(Args)]
pub struct GenerateArgs {
    /// Model name to use
    pub model: String,
    /// Prompt text
    pub prompt: String,
    /// Temperature (0.0-2.0)
    #[arg(short, long)]
    pub temperature: Option<f32>,
    /// Top-p sampling (0.0-1.0)
    #[arg(long)]
    pub top_p: Option<f32>,
    /// Maximum tokens to generate
    #[arg(short, long)]
    pub max_tokens: Option<u32>,
}

#[derive(Args)]
pub struct DownloadArgs {
    /// Model name to download
    pub model: String,
}

#[derive(Args)]
pub struct InfoArgs {
    /// Model name to show info for
    pub model: String,
}

pub fn run_cli() -> Result<(), LlmError> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Generate(args) => generate_command(args),
        Commands::List => list_command(),
        Commands::Download(args) => download_command(args),
        Commands::Info(args) => info_command(args),
    }
}

fn generate_command(args: GenerateArgs) -> Result<(), LlmError> {
    let mut llm = open(&args.model)?;
    
    let mut cli_args = Vec::new();

    if let Some(temp) = args.temperature {
        cli_args.extend_from_slice(&["--temp".to_string(), temp.to_string()]);
    }
    if let Some(top_p) = args.top_p {
        cli_args.extend_from_slice(&["--top-p".to_string(), top_p.to_string()]);
    }
    if let Some(max_tokens) = args.max_tokens {
        cli_args.extend_from_slice(&["--n-predict".to_string(), max_tokens.to_string()]);
    }

    if !cli_args.is_empty() {
        llm = llm.with_args(cli_args);
    }

    let prompt = if args.prompt.is_empty() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).map_err(|e| LlmError::Io(e))?;
        buffer
    } else {
        args.prompt.clone()
    };
    
    let response = llm.generate(&prompt)?;
    
    println!("{}", response.trim());
    Ok(())
}

fn list_command() -> Result<(), LlmError> {
    let discovered = discover_models()?;
    let config = config::load_config()?;
    
    if discovered.is_empty() && config.models.is_empty() {
        println!("No models found. Download a model with: agentd download <model-name>");
        return Ok(());
    }
    
    println!("Available models:");
    println!();
    
    // Show configured models first
    for (name, entry) in &config.models {
        let status = if config::resolve_model_path(name).is_ok() { "✓" } else { "✗" };
        println!("  {} {} - {}", status, name, entry.description.as_deref().unwrap_or("No description"));
    }
    
    // Show discovered models
    for (name, entry) in &discovered {
        if !config.models.contains_key(name) {
            let status = if config::resolve_model_path(name).is_ok() { "✓" } else { "✗" };
            println!("  {} {} - {}", status, name, entry.description.as_deref().unwrap_or("Auto-discovered"));
        }
    }
    
    Ok(())
}

fn download_command(args: DownloadArgs) -> Result<(), LlmError> {
    // Predefined model download URLs
    let model_urls = get_model_urls();
    
    if let Some(url_info) = model_urls.get(&args.model) {
        println!("Downloading model: {}", args.model);
        println!("This would download from: {}", url_info.url);
        println!("File: {}", url_info.filename);
        // Placeholder for actual download logic
        // For example, using huggingface-cli
        println!("Run this command to download:");
        println!("huggingface-cli download {} {} --local-dir ~/.agentd/models/",
                 url_info.repo, url_info.filename);
    } else {
        return Err(LlmError::InvalidModelPath(format!("Unknown model: {}", args.model)));
    }
    
    Ok(())
}

fn info_command(args: InfoArgs) -> Result<(), LlmError> {
    let config = config::load_config()?;
    let discovered = discover_models()?;
    
    let model_entry = config.models.get(&args.model)
        .or_else(|| discovered.get(&args.model))
        .ok_or_else(|| LlmError::InvalidModelPath(format!("Model '{}' not found", args.model)))?;
    
    let model_path = config::resolve_model_path(&args.model)?;
    
    println!("Model: {}", args.model);
    println!("File: {}", model_entry.file);
    println!("Path: {}", model_path.display());
    println!("Description: {}", model_entry.description.as_deref().unwrap_or("N/A"));
    
    Ok(())
}

struct ModelUrl<'a> {
    repo: &'a str,
    filename: &'a str,
    url: &'a str,
}

fn get_model_urls<'a>() -> HashMap<String, ModelUrl<'a>> {
    let mut models = HashMap::new();
    models.insert("gemma-2-2b-it-GGUF".to_string(), ModelUrl {
        repo: "bartowski/gemma-2-2b-it-GGUF",
        filename: "gemma-2-2b-it-Q4_K_M.gguf",
        url: "https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/blob/main/gemma-2-2b-it-Q4_K_M.gguf"
    });
    
    models.insert("gemma-2-2b-GGUF".to_string(), ModelUrl {
        repo: "bartowski/gemma-2-2b-GGUF",
        filename: "gemma-2-2b-Q4_K_M.gguf",
        url: "https://huggingface.co/bartowski/gemma-2-2b-GGUF/blob/main/gemma-2-2b-Q4_K_M.gguf"
    });

    models
}