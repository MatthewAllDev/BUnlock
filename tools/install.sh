#!/bin/bash

BIN_DIR="$HOME/.local/bin/bunlock"
PROFILE_FILES=("$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile")

mkdir -p "$BIN_DIR"

if [ -f "bunlock" ]; then
    cp -rf bunlock "$BIN_DIR"
elif [ -f "../target/release/bunlock" ]; then
    cp -rf ../target/release/bunlock "$BIN_DIR"
else
    echo "Bunlock file not found."
fi

for PROFILE_FILE in "${PROFILE_FILES[@]}"; do
    if [ -f "$PROFILE_FILE" ]; then
        if ! grep -q "export PATH=\"$BIN_DIR:\$PATH\"" "$PROFILE_FILE"; then
            echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$PROFILE_FILE"
        fi
    fi
done

if [ -n "$ZSH_VERSION" ]; then
    source "$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    source "$HOME/.bashrc"
else
    source "$HOME/.profile"
fi

bunlock config
bunlock service enable
bunlock service start

echo "Installation and setup completed successfully."