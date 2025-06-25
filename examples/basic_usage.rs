use agentd::{LlmConfig, LlmInterface, backends::LlamaCppBackend};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = LlmConfig::new("llama.cpp", "models/llama-2-7b-chat.gguf")
        .with_args(vec![
            "--temp".to_string(), "0.7".to_string(),
            "--top-p".to_string(), "0.9".to_string(),
            "--repeat-penalty".to_string(), "1.1".to_string(),
        ]);
    
    let llm = LlamaCppBackend::new(config)?;
    
    let prompts = vec![
        "What is the capital of France?",
        "Explain quantum computing in simple terms.",
        "Write a haiku about programming.",
    ];
    
    for prompt in prompts {
        println!("Prompt: {}", prompt);
        match llm.generate(prompt) {
            Ok(response) => println!("Response: {}\n", response.trim()),
            Err(e) => eprintln!("Error: {}\n", e),
        }
    }
    
    Ok(())
}