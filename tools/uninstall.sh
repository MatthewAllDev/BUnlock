#!/bin/bash

BIN_PATH="$HOME/.local/bin/bunlock"
SERVICE_PATH="$HOME/.config/systemd/user/bunlock.service"
CONFIG_DIR="$HOME/.config/bunlock"
PROFILE_FILES=("$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile")

if bunlock service is_active; then
    bunlock service stop
fi

bunlock service disable

if [ -f "$SERVICE_PATH" ]; then
    rm "$SERVICE_PATH"
    systemctl --user daemon-reload
fi

if [ -d "$BIN_PATH" ]; then
    rm -rf "$BIN_PATH"
fi

if [ -d "$CONFIG_DIR" ]; then
    rm -rf "$CONFIG_DIR"
fi

for PROFILE_FILE in "${PROFILE_FILES[@]}"; do
    if [ -f "$PROFILE_FILE" ]; then
        sed -i "\|export PATH=\"$HOME/.local/bin:\$PATH\"|d" "$PROFILE_FILE"
    fi
done

if [ -n "$ZSH_VERSION" ]; then
    unhash -r
    source "$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    hash -r
    source "$HOME/.bashrc"
else
    source "$HOME/.profile"
fi

echo "BUnlock has been uninstalled successfully."
