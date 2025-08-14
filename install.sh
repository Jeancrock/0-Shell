#!/bin/bash

# Install.sh pour 0-Shell
# Ce script installe Rust, compile le shell

set -e

echo "=== Installation de Rust ==="
# Vérifie si rustc est installé
if ! command -v rustc &> /dev/null
then
    echo "Rust non trouvé, installation via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust déjà installé : $(rustc --version)"
fi

echo "=== Installation des dépendances Cargo ==="
cargo add rustyline
cargo add whoami
cargo add dirs
cargo add chrono

echo "=== Compilation du projet ==="
cargo build --release

echo "=== Installation terminée ==="
