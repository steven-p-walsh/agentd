use agentd::{LlmConfig, LlmInterface, backends::LlamaCppBackend};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using model name instead of full path - replace with your model name
    let config = LlmConfig::from_model_name("your-model-name")?;
    
    let llm = LlamaCppBackend::new(config)?;
    
    let prompts = vec![
        "What is the capital of France?",
        "Explain machine learning in one sentence.",
        "Write a short greeting message.",
    ];
    
    for prompt in prompts {
        println!("=== Prompt: {} ===", prompt);
        match llm.generate(prompt) {
            Ok(response) => {
                println!("Response: {}", response.trim());
                println!();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                println!();
            }
        }
    }
    
    Ok(())
}