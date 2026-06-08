#!/usr/bin/env bash
# Install jdw: copy binary to /usr/local/bin, config to ~/.config,
# synthdefs to /usr/local/share/jdw.
set -e

echo "Installing jdw..."

# Binary
if [[ -f jdw ]]; then
    sudo cp jdw /usr/local/bin/jdw
    sudo chmod +x /usr/local/bin/jdw
    echo "  jdw -> /usr/local/bin/jdw"
else
    echo "  WARNING: jdw binary not found in current directory"
fi

# Config
if [[ ! -f ~/.config/jdw.toml ]]; then
    mkdir -p ~/.config
    cp example.jdw.toml ~/.config/jdw.toml
    echo "  example.jdw.toml -> ~/.config/jdw.toml"
else
    echo "  ~/.config/jdw.toml already exists, skipping"
fi

# Synthdefs
sudo mkdir -p /usr/local/share/jdw
sudo cp synthdefs.scd /usr/local/share/jdw/
echo "  synthdefs.scd -> /usr/local/share/jdw/synthdefs.scd"

echo ""
echo "Done. Run: jdw all"
