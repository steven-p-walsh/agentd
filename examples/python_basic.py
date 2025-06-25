#!/usr/bin/env python3
"""
Basic Python example for agentd library.

This example demonstrates how to use the agentd Python bindings to interact
with local Large Language Models.
"""

import agentd

def main():
    print("AgentD Python Example")
    print("=" * 50)
    
    # List available models
    try:
        models = agentd.list_models()
        print(f"Available models: {models}")
    except Exception as e:
        print(f"Failed to list models: {e}")
        return
    
    if not models:
        print("No models available. Please download a model first.")
        return
    
    # Use the first available model (should be our Gemma model)
    model_name = models[0]
    print(f"Using model: {model_name}")
    
    try:
        # Method 1: Open the model by name
        llm = agentd.open(model_name)
        print("Model loaded successfully (by name)!")
        
        # Method 2: Open the default model (no arguments)
        llm_default = agentd.open()
        print("Default model loaded successfully!")
        
        # Get model configuration
        config = llm.config()
        print(f"Model path: {config.model_path}")
        print(f"Executable: {config.executable_path}")
        print(f"Arguments: {config.additional_args}")
        
        # Test prompts
        prompts = [
            "What is the capital of France?",
            "Explain machine learning in one sentence.",
            "Write a short greeting message."
        ]
        
        for i, prompt in enumerate(prompts, 1):
            print(f"\n--- Test {i} ---")
            print(f"Prompt: {prompt}")
            
            try:
                response = llm.generate(prompt)
                print(f"Response: {response.strip()}")
            except Exception as e:
                print(f"Error generating response: {e}")
        
        print("\n--- Testing with custom arguments ---")
        # Create a new LLM instance with custom arguments
        custom_args = ["--temp", "0.9", "--top-p", "0.8"]
        llm_custom = agentd.open_with_args(model_name, custom_args)
        
        prompt = "Write a creative haiku about programming."
        print(f"Prompt: {prompt}")
        
        try:
            response = llm_custom.generate(prompt)
            print(f"Response: {response.strip()}")
        except Exception as e:
            print(f"Error generating response: {e}")
            
    except Exception as e:
        print(f"Failed to open model '{model_name}': {e}")

if __name__ == "__main__":
    main()