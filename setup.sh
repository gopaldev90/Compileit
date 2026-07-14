#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="$HOME/.local/bin"
SOURCE="./compileit.axe"
TARGET="$BIN_DIR/compileit"

main() {
    if [[ ! -f "$SOURCE" ]]; then
        echo "Error: '$SOURCE' not found."
        exit 1
    fi

    mkdir -p "$BIN_DIR"

    install -m 755 "$SOURCE" "$TARGET"

    echo "Installed compileit to:"
    echo "  $TARGET"

    if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
        cat <<EOF

$BIN_DIR is not in your PATH.

Add this line to your ~/.bashrc (or ~/.zshrc):

    export PATH="\$HOME/.local/bin:\$PATH"

Then restart your terminal or run:

    source ~/.bashrc

EOF
    fi

    echo "Installation complete."
    echo "Run: compileit --help"
}

main "$@"