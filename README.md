# agentd

A simple Rust interface for interacting with local Large Language Models (LLMs) via file descriptor interfaces.

## Features

- **Model Name Resolution**: Reference models by name instead of full paths
- **Auto-Discovery**: Automatically finds GGUF models in `~/.agentd/models/`
- **Configuration System**: TOML-based configuration with sensible defaults
- **CLI Interface**: Easy-to-use command-line interface
- **Installation Script**: One-command installation and setup
- **GGUF Support**: Native support for GGUF model files via llama.cpp

## Installation

```bash
git clone <repository>
cd agentd
./install.sh
```

This will:
- Build and install agentd to `~/.agentd/bin/`
- Create default configuration files
- Add agentd to your PATH
- Set up the models directory

## Quick Start

1. **Download a model:**
```bash
# Using huggingface-cli (for now)
huggingface-cli download bartowski/gemma-2-2b-it-GGUF gemma-2-2b-it-Q4_K_M.gguf --local-dir ~/.agentd/models/
```

2. **List available models:**
```bash
agentd list
```

3. **Generate text:**
```bash
agentd generate gemma-2-2b-it-Q4_K_M "What is the capital of France?"
```

## CLI Usage

### Generate Text
```bash
agentd generate <model-name> "<prompt>" [options]

Options:
  -t, --temperature <TEMP>    Temperature (0.0-2.0)
      --top-p <TOP_P>         Top-p sampling (0.0-1.0)
  -m, --max-tokens <TOKENS>   Maximum tokens to generate
```

### List Models
```bash
agentd list
```

### Model Information
```bash
agentd info <model-name>
```

### Download Models
```bash
agentd download <model-name>
```

## Configuration

Configuration files are stored in `~/.agentd/config/`:

### `config.toml`
```toml
[runtime]
default_backend = "llama.cpp"
llama_executable = "llama-cli"

[defaults]
temperature = 0.7
top_p = 0.9
repeat_penalty = 1.1
max_tokens = 256
```

### `models.toml`
```toml
[gemma-2-2b-it]
file = "gemma-2-2b-it-Q4_K_M.gguf"
description = "Gemma 2 2B Instruction Tuned (Q4_K_M)"
context_size = 8192
```

## Library Usage

```rust
use agentd::{LlmConfig, LlmInterface, backends::LlamaCppBackend};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using model name (recommended)
    let config = LlmConfig::from_model_name("gemma-2-2b-it-Q4_K_M")?;
    let llm = LlamaCppBackend::new(config)?;
    
    let response = llm.generate("What is machine learning?")?;
    println!("{}", response);
    
    Ok(())
}
```

## Directory Structure

```
~/.agentd/
├── bin/agentd           # Executable
├── models/              # GGUF model files
│   └── *.gguf
└── config/
    ├── config.toml      # Main configuration
    └── models.toml      # Model registry
```

## Supported Models

agentd automatically discovers any `.gguf` files in the models directory. Pre-configured models include:

- **gemma-2-2b-it**: Gemma 2 2B Instruction Tuned
- **gemma-2-2b**: Gemma 2 2B Base Model

## Examples

See the `examples/` directory:
- `basic_usage.rs`: Original path-based usage
- `model_name_usage.rs`: Model name-based usage
- `gemma_test.rs`: Testing with real Gemma model

## Error Handling

The library provides comprehensive error handling through the `LlmError` enum:
- `Io`: I/O operation errors
- `Utf8`: UTF-8 conversion errors
- `ProcessSpawn`: Process creation errors
- `ProcessExecution`: Process execution errors
- `InvalidModelPath`: Model file not found
- `EmptyResponse`: Empty response from LLM

## Requirements

- Rust 1.70+
- llama.cpp (installed via `brew install llama.cpp`)
- GGUF model files