use crate::error::LlmError;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub executable_path: String,
    pub model_path: String,
    pub additional_args: Vec<String>,
}

impl LlmConfig {
    pub fn new(executable_path: impl Into<String>, model_path: impl Into<String>) -> Self {
        Self {
            executable_path: executable_path.into(),
            model_path: model_path.into(),
            additional_args: Vec::new(),
        }
    }

    pub fn from_model_name(model_name: &str) -> Result<Self, LlmError> {
        let config = crate::config::load_config()?;
        let model_path = crate::config::resolve_model_path(model_name)?;
        
        Ok(Self {
            executable_path: config.runtime.llama_executable,
            model_path: model_path.to_string_lossy().to_string(),
            additional_args: vec![
                "--temp".to_string(), config.defaults.temperature.to_string(),
                "--top-p".to_string(), config.defaults.top_p.to_string(),
                "--repeat-penalty".to_string(), config.defaults.repeat_penalty.to_string(),
                "--n-predict".to_string(), config.defaults.max_tokens.to_string(),
            ],
        })
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.additional_args = args;
        self
    }
}

pub fn open(model_name: &str) -> Result<Box<dyn LlmInterface + Send + Sync>, LlmError> {
    let config = LlmConfig::from_model_name(model_name)?;
    let backend = llamacpp::LlamaCppBackend::new(config)?;
    Ok(Box::new(backend))
}

pub trait LlmInterface: Send + Sync {
    fn generate(&self, prompt: &str) -> Result<String, LlmError>;
    fn config(&self) -> &LlmConfig;
    fn with_args(self: Box<Self>, args: Vec<String>) -> Box<dyn LlmInterface + Send + Sync>;
}

pub mod backends {
    pub use super::llamacpp::LlamaCppBackend;
}

mod llamacpp {
    use super::*;
    use std::process::{Command, Stdio};
    use std::io::Write;

    #[derive(Debug)]
    pub struct LlamaCppBackend {
        config: LlmConfig,
    }

    impl LlamaCppBackend {
        pub fn new(config: LlmConfig) -> Result<Self, LlmError> {
            if !Path::new(&config.model_path).exists() {
                return Err(LlmError::InvalidModelPath(config.model_path.clone()));
            }
            
            Ok(Self { config })
        }
    }

    impl LlmInterface for LlamaCppBackend {
        fn generate(&self, prompt: &str) -> Result<String, LlmError> {
            let mut cmd = Command::new(&self.config.executable_path);
            cmd.args(&["--model", &self.config.model_path])
               .args(&self.config.additional_args)
               .stdin(Stdio::piped())
               .stdout(Stdio::piped())
               .stderr(Stdio::piped());

            let mut child = cmd.spawn()
                .map_err(|e| LlmError::ProcessSpawn(format!("Failed to spawn {}: {}", self.config.executable_path, e)))?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(prompt.as_bytes())
                    .map_err(LlmError::Io)?;
            }

            let output = child.wait_with_output()
                .map_err(LlmError::Io)?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(LlmError::ProcessExecution(format!("Process failed with status {}: {}", output.status, stderr)));
            }

            let response = String::from_utf8(output.stdout)?;
            
            if response.trim().is_empty() {
                return Err(LlmError::EmptyResponse);
            }

            Ok(response)
        }

        fn config(&self) -> &LlmConfig {
            &self.config
        }

        fn with_args(mut self: Box<Self>, args: Vec<String>) -> Box<dyn LlmInterface + Send + Sync> {
            self.config.additional_args = args;
            self
        }
    }
}