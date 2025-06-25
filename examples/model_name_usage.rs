use agentd::open;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using model name instead of full path - using the Gemma model
    let llm = open("gemma-3-12B-it-QAT-Q4_0")?;
    
    let prompt = "What is the capital of France?";
    println!("Prompt: {}", prompt);
    
    let prompts = vec![
        "What is the capital of France?",
        "Explain quantum computing in simple terms.",
        "Write a haiku about programming.",
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