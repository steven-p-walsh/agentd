#!/usr/bin/env python3
"""
GPU Support Example for agentd Python bindings.

This example demonstrates how to use agentd with GPU acceleration.
The GPU settings are configured in the config files.
"""

import agentd

def main():
    print("AgentD GPU Support Example")
    print("=" * 50)
    
    try:
        # Open the default model (should have GPU support enabled)
        llm = agentd.open()
        print("✓ Model loaded successfully")
        
        # Show configuration including GPU settings
        config = llm.config()
        print(f"✓ Model: {config.model_path.split('/')[-1]}")
        print(f"✓ Executable: {config.executable_path}")
        print("✓ Arguments:", config.additional_args)
        
        # Check if GPU layers are being used
        gpu_layers_found = False
        for i, arg in enumerate(config.additional_args):
            if arg == "--n-gpu-layers" and i + 1 < len(config.additional_args):
                layers = config.additional_args[i + 1]
                if layers != "0":
                    print(f"🚀 GPU acceleration enabled with {layers} layers offloaded to GPU")
                    gpu_layers_found = True
                else:
                    print("💻 GPU acceleration disabled (0 layers), running on CPU only")
                break
        
        if not gpu_layers_found:
            print("💻 GPU acceleration not configured (running on CPU only)")
            print("   To enable GPU, use: agentd.open_with_args(['--n-gpu-layers', '8'])")
        
        print("\n" + "=" * 50)
        print("Testing generation with GPU acceleration:")
        print("=" * 50)
        
        # Test prompts that benefit from GPU acceleration
        prompts = [
            "What is 2+2?",
            "Name the capital of France.",
            "List 3 colors."
        ]
        
        for i, prompt in enumerate(prompts, 1):
            print(f"\n--- Test {i} ---")
            print(f"Prompt: {prompt}")
            
            try:
                response = llm.generate(prompt)
                print(f"Response: {response}")
            except Exception as e:
                print(f"Error: {e}")
        
        # Test with custom GPU settings
        print(f"\n{'=' * 50}")
        print("Testing with custom GPU arguments:")
        print("=" * 50)
        
        # Demonstrate how to enable GPU manually (users can adjust layer count)
        print("Attempting to enable GPU acceleration manually...")
        try:
            custom_gpu_args = ["--n-gpu-layers", "4", "--temp", "0.8"]
            llm_custom = agentd.open_with_args(custom_gpu_args)
            
            config_custom = llm_custom.config()
            print("Custom configuration arguments:", config_custom.additional_args)
            
            prompt = "What is AI?"
            print(f"Prompt: {prompt}")
            
            response = llm_custom.generate(prompt)
            print(f"Response: {response}")
            
        except Exception as e:
            print(f"GPU acceleration failed, falling back to CPU: {e}")
            # Fallback to CPU-only
            cpu_args = ["--n-gpu-layers", "0", "--temp", "0.8"]
            llm_cpu = agentd.open_with_args(cpu_args)
            response = llm_cpu.generate(prompt)
            print(f"CPU Response: {response}")
            
        print(f"\n{'=' * 50}")
        print("✓ GPU example completed successfully!")
        
    except Exception as e:
        print(f"✗ Error: {e}")

if __name__ == "__main__":
    main()