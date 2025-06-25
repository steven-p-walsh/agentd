#!/usr/bin/env python3
"""
Simple Python example demonstrating the default model functionality.

This example shows how to use agentd.open() without specifying a model name.
"""

import agentd

def main():
    print("AgentD Simple Example - Using Default Model")
    print("=" * 50)
    
    try:
        # Open the default model (first available)
        llm = agentd.open()
        print("✓ Opened default model successfully")
        
        # Show which model was selected
        config = llm.config()
        model_name = config.model_path.split('/')[-1]  # Get just the filename
        print(f"✓ Using model: {model_name}")
        
        # Simple test
        prompt = "What is the capital of Japan?"
        print(f"\nPrompt: {prompt}")
        response = llm.generate(prompt)
        print(f"Response: {response.strip()}")
        
        print("\n" + "=" * 50)
        print("✓ Example completed successfully!")
        
    except Exception as e:
        print(f"✗ Error: {e}")

if __name__ == "__main__":
    main()