use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    
    #[error("Process spawn error: {0}")]
    ProcessSpawn(String),
    
    #[error("Process execution error: {0}")]
    ProcessExecution(String),
    
    #[error("Invalid model path: {0}")]
    InvalidModelPath(String),
    
    #[error("Empty response from LLM")]
    EmptyResponse,
}