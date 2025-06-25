use agentd::{open, LlmError};
use std::io::Write;
use tempfile::NamedTempFile;
use std::fs;

// Helper to create a dummy config file with a specific model name
fn setup_with_model_name(model_name: &str) {
    let config_dir = dirs::home_dir().unwrap().join(".agentd/config");
    fs::create_dir_all(&config_dir).unwrap();
    let models_toml = config_dir.join("models.toml");
    let mut file = fs::File::create(models_toml).unwrap();
    file.write_all(format!("[{}]\nfile = \"gemma-3-12B-it-QAT-Q4_0.gguf\"\n", model_name).as_bytes()).unwrap();

    let model_dir = dirs::home_dir().unwrap().join(".agentd/models");
    fs::create_dir_all(&model_dir).unwrap();
    // Copy the actual model file to the test location
    let src_model = "/Users/stevenwalsh/repositories/agentd/models/gemma-3-12B-it-QAT-Q4_0.gguf";
    let dst_model = model_dir.join("gemma-3-12B-it-QAT-Q4_0.gguf");
    if std::path::Path::new(src_model).exists() {
        fs::copy(src_model, dst_model).unwrap();
    } else {
        panic!("Source model file not found: {}", src_model);
    }
}

// Helper to create a dummy config file
fn setup() {
    setup_with_model_name("test-model");
}

// Helper to clean up created files
fn cleanup() {
    let agentd_dir = dirs::home_dir().unwrap().join(".agentd");
    if agentd_dir.exists() {
        // Use ignore errors in case directories don't exist or are already cleaned
        let _ = fs::remove_dir_all(agentd_dir);
    }
}

fn create_mock_executable() -> std::io::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "#!/bin/sh")?;
    writeln!(file, "echo Mock LLM response")?;
    Ok(file)
}

fn create_mock_model() -> std::io::Result<NamedTempFile> {
    NamedTempFile::new()
}

#[test]
fn test_open_with_model_name() {
    cleanup(); // Clean up any previous state
    setup_with_model_name("test-model-1");
    let llm = open("test-model-1");
    match llm {
        Ok(_) => {},
        Err(e) => panic!("Failed to open model: {:?}", e)
    }
    cleanup();
}

#[test]
fn test_open_with_invalid_model_name() {
    cleanup(); // Clean up any previous state
    setup();
    let model_name = "non-existent-model";
    let result = open(model_name);
    assert!(matches!(result, Err(LlmError::InvalidModelPath(_))));
    cleanup();
}

#[test]
fn test_with_args() {
    cleanup(); // Clean up any previous state
    setup_with_model_name("test-model-2");
    let model_name = "test-model-2";
    let llm = open(model_name)
        .unwrap()
        .with_args(vec!["--temp".to_string(), "0.5".to_string()]);
    
    let config = llm.config();
    assert!(config.additional_args.contains(&"--temp".to_string()));
    assert!(config.additional_args.contains(&"0.5".to_string()));
    cleanup();
}