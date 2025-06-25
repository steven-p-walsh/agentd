pub mod llm;
pub mod error;
pub mod config;
pub mod cli;

pub use llm::{LlmInterface, LlmConfig, backends};
pub use error::LlmError;
pub use config::{AgentConfig, load_config, resolve_model_path, discover_models};