use crate::error::LlmError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub runtime: RuntimeConfig,
    pub models: HashMap<String, ModelEntry>,
    pub defaults: DefaultParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub default_backend: String,
    pub llama_executable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    pub file: String,
    pub description: Option<String>,
    pub context_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultParams {
    pub temperature: f32,
    pub top_p: f32,
    pub repeat_penalty: f32,
    pub max_tokens: u32,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            runtime: RuntimeConfig {
                default_backend: "llama.cpp".to_string(),
                llama_executable: "llama-cli".to_string(),
            },
            models: HashMap::new(),
            defaults: DefaultParams {
                temperature: 0.7,
                top_p: 0.9,
                repeat_penalty: 1.1,
                max_tokens: 256,
            },
        }
    }
}

pub fn get_agentd_home() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".agentd")
}

pub fn get_config_dir() -> PathBuf {
    get_agentd_home().join("config")
}

pub fn get_models_dir() -> PathBuf {
    get_agentd_home().join("models")
}

pub fn load_config() -> Result<AgentConfig, LlmError> {
    let config_path = get_config_dir().join("config.toml");
    
    if !config_path.exists() {
        return Ok(AgentConfig::default());
    }
    
    let content = fs::read_to_string(&config_path)
        .map_err(|e| LlmError::Io(e))?;
    
    let config: AgentConfig = toml::from_str(&content)
        .map_err(|e| LlmError::ProcessExecution(format!("Failed to parse config: {}", e)))?;
    
    Ok(config)
}

pub fn discover_models() -> Result<HashMap<String, ModelEntry>, LlmError> {
    let models_dir = get_models_dir();
    let mut models = HashMap::new();
    
    if !models_dir.exists() {
        return Ok(models);
    }
    
    for entry in fs::read_dir(&models_dir).map_err(LlmError::Io)? {
        let entry = entry.map_err(LlmError::Io)?;
        let path = entry.path();
        
        if let Some(extension) = path.extension() {
            if extension == "gguf" {
                if let Some(file_name) = path.file_name() {
                    if let Some(name) = file_name.to_str() {
                        let model_name = name.trim_end_matches(".gguf");
                        models.insert(model_name.to_string(), ModelEntry {
                            file: name.to_string(),
                            description: Some(format!("Auto-discovered model: {}", name)),
                            context_size: Some(4096),
                        });
                    }
                }
            }
        }
    }
    
    Ok(models)
}

pub fn resolve_model_path(model_name: &str) -> Result<PathBuf, LlmError> {
    let config = load_config()?;
    let discovered_models = discover_models()?;
    
    // Check config first, then discovered models
    let model_entry = config.models.get(model_name)
        .or_else(|| discovered_models.get(model_name))
        .ok_or_else(|| LlmError::InvalidModelPath(format!("Model '{}' not found", model_name)))?;
    
    let models_dir = get_models_dir();
    let model_path = models_dir.join(&model_entry.file);
    
    if !model_path.exists() {
        return Err(LlmError::InvalidModelPath(format!(
            "Model file '{}' not found at path: {}", 
            model_entry.file, 
            model_path.display()
        )));
    }
    
    Ok(model_path)
}