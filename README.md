# agentd

A simple command-line tool for interacting with local Large Language Models (LLMs). Built in Rust with a focus on simplicity and performance.

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
# Using huggingface-cli to download Gemma models
huggingface-cli download lmstudio-community/gemma-3-12B-it-qat-GGUF gemma-3-12B-it-QAT-Q4_0.gguf --local-dir ~/.agentd/models/
```

2. **List available models:**
```bash
agentd list
```

3. **Generate text:**
```bash
agentd generate gemma-3-12B-it-QAT-Q4_0 "What is the capital of France?"
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
[gemma-3-12B-it-QAT-Q4_0]
file = "gemma-3-12B-it-QAT-Q4_0.gguf"
description = "Gemma 3 12B Instruction Tuned (QAT Q4_0)"
context_size = 8192
```

## File Descriptor Interface

`agentd` uses a file descriptor-based interface to communicate with the underlying llama.cpp process. This provides efficient streaming communication and allows for real-time interaction with the model.

### How it works:
1. **Process Spawning**: When you open a model, agentd spawns a llama.cpp process with the specified model and parameters
2. **Stdin/Stdout Communication**: Text prompts are sent via stdin, and model responses are read from stdout  
3. **Streaming Output**: Responses are streamed token-by-token, allowing for real-time display
4. **Process Management**: The process lifecycle is managed automatically, with proper cleanup on exit

This approach ensures low overhead and efficient resource usage while maintaining compatibility with the full llama.cpp feature set.

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

agentd automatically discovers any `.gguf` files in the models directory. The examples use:

- **gemma-3-12B-it-QAT-Q4_0**: Gemma 3 12B Instruction Tuned (QAT Q4_0 quantization)

Any GGUF model compatible with llama.cpp will work with agentd.

## Examples

The `examples/` directory contains working examples:

```bash
# Run the basic usage example
cargo run --example basic_usage

# Test with different prompts
cargo run --example gemma_test

# Demonstrate model name resolution
cargo run --example model_name_usage
```

All examples use the Gemma model and demonstrate different aspects of the agentd library.

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