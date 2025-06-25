pub mod llm;
pub mod error;
pub mod config;
pub mod cli;

pub use llm::{LlmInterface, open};
pub use error::LlmError;
pub use config::{AgentConfig, load_config, resolve_model_path, discover_models};

// Python bindings module
mod pybindings;

// Export the Python module
pub use pybindings::agentd;