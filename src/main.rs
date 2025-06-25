use agentd::cli::run_cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_cli()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_mock_executable() -> std::io::Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "#!/bin/bash")?;
        writeln!(file, "echo 'Mock LLM response'")?;
        file.flush()?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = file.as_file().metadata()?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
            file.as_file().set_permissions(permissions)?;
        }
        
        Ok(file)
    }

    fn create_mock_model() -> std::io::Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "mock model data")?;
        file.flush()?;
        Ok(file)
    }

    #[test]
    fn test_llm_config_creation() {
        let config = LlmConfig::new("test_executable", "test_model.gguf");
        assert_eq!(config.executable_path, "test_executable");
        assert_eq!(config.model_path, "test_model.gguf");
        assert!(config.additional_args.is_empty());
    }

    #[test]
    fn test_llm_config_with_args() {
        let config = LlmConfig::new("test_executable", "test_model.gguf")
            .with_args(vec!["--temp".to_string(), "0.7".to_string()]);
        
        assert_eq!(config.additional_args, vec!["--temp", "0.7"]);
    }

    #[test]
    fn test_llamacpp_backend_invalid_model() {
        let config = LlmConfig::new("test_executable", "nonexistent_model.gguf");
        let result = LlamaCppBackend::new(config);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), agentd::LlmError::InvalidModelPath(_)));
    }

    #[test]
    fn test_llamacpp_backend_generation() -> Result<(), Box<dyn std::error::Error>> {
        let mock_executable = create_mock_executable()?;
        let mock_model = create_mock_model()?;
        
        let config = LlmConfig::new(
            mock_executable.path().to_string_lossy().to_string(),
            mock_model.path().to_string_lossy().to_string()
        );
        
        let llm = LlamaCppBackend::new(config)?;
        let response = llm.generate("Test prompt")?;
        
        assert!(!response.trim().is_empty());
        assert!(response.contains("Mock LLM response"));
        
        Ok(())
    }
}