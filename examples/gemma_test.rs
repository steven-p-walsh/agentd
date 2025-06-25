use agentd::open;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Gemma 3 12B GGUF test...");

    let llm = open("gemma-3-12B-it-QAT-Q4_0")?;

    let prompt = "What is the capital of France?";
    println!("Prompt: {}", prompt);
    
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