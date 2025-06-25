#!/bin/bash
set -e

AGENTD_HOME="${HOME}/.agentd"
AGENTD_BIN="${AGENTD_HOME}/bin"
AGENTD_MODELS="${AGENTD_HOME}/models"
AGENTD_CONFIG="${AGENTD_HOME}/config"

echo "Installing agentd..."

# Create directory structure
mkdir -p "${AGENTD_BIN}"
mkdir -p "${AGENTD_MODELS}"
mkdir -p "${AGENTD_CONFIG}"

# Build the project
echo "Building agentd..."
cargo build --release

# Create symlink for binary
echo "Creating symlink for binary in ${AGENTD_BIN}..."
ln -sf "$(pwd)/target/release/agentd" "${AGENTD_BIN}/agentd"

# Create default config
echo "Creating default configuration..."
cat > "${AGENTD_CONFIG}/config.toml" << 'EOF'
[runtime]
default_backend = "llama.cpp"
llama_executable = "llama-cli"
use_gpu = false
gpu_layers = 0

[models]
# Models will be auto-discovered in ~/.agentd/models/
# You can also add custom model definitions here

[defaults]
temperature = 0.7
top_p = 0.9
repeat_penalty = 1.1
max_tokens = 256
EOF

# Create model registry
echo "Creating model registry..."
cat > "${AGENTD_CONFIG}/models.toml" << 'EOF'
# Model Registry
# Models are automatically discovered, but you can override settings here
# Add your model configurations below following this format:
#
# [your-model-name]
# file = "your-model-file.gguf"
# description = "Description of your model"
# context_size = 4096
EOF

# Move any existing GGUF models from models/ directory
if [ -d "models" ] && [ "$(ls -A models/*.gguf 2>/dev/null)" ]; then
    echo "Moving existing models to agentd models directory..."
    mv models/*.gguf "${AGENTD_MODELS}/" 2>/dev/null || true
fi

# Add to PATH if not already there
SHELL_RC=""
CURRENT_SHELL=$(basename "$SHELL")
if [ "$CURRENT_SHELL" = "zsh" ]; then
    SHELL_RC="${HOME}/.zshrc"
elif [ "$CURRENT_SHELL" = "bash" ]; then
    SHELL_RC="${HOME}/.bashrc"
fi

if [ -n "$SHELL_RC" ] && [ -f "$SHELL_RC" ]; then
    if ! grep -q "agentd" "$SHELL_RC"; then
        echo "" >> "$SHELL_RC"
        echo "# agentd" >> "$SHELL_RC"
        echo "export PATH=\"\$PATH:${AGENTD_BIN}\"" >> "$SHELL_RC"
        echo "Added agentd to PATH in $SHELL_RC"
    fi
fi

echo ""
echo "✅ agentd installed successfully!"
echo ""
echo "Installation directory: ${AGENTD_HOME}"
echo "Binary location: ${AGENTD_BIN}/agentd"
echo "Models directory: ${AGENTD_MODELS}"
echo "Config directory: ${AGENTD_CONFIG}"
echo ""
echo "To complete installation:"
echo "1. Restart your shell or run: export PATH=\"\$PATH:${AGENTD_BIN}\""
echo "2. Test with: agentd --help"
echo ""
echo "To download models, place them in: ${AGENTD_MODELS}"
echo ""
echo "To run inference:"
echo "  agentd generate <model-name> \"What is the capital of France?\""

exec "$SHELL"