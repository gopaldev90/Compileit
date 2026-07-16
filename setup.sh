#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="$HOME/.local/bin"
SOURCE="./compileit.axe"
TARGET="$BIN_DIR/compileit"
# add this function in your .bashrc file to run compiled binary
chlao(){
    local src="$1"
    local kahan_they="$PWD"
    shift
    local fname=$(basename "$src")
    local dest="$HOME/$fname"
    cleanup(){
        if [ -f "$dest" ]; then
          rm "$dest"
          cd "$kahan_they"
          echo -e "${LIGHT_GREEN}Shree shivaay namashtubhyam${RESET}"
        fi
        cd "$kahan_they"
    }
    trap cleanup RETURN INT TERM
    if [ -f "$src" ]; then
        cp "$src" "$dest" || return 1
        cd "$HOME"
        chmod +x "$fname"
        ./"$fname" "$@"
        cleanup
    else
        echo -e "${RED}$src hai hee nahi${RESET}"
    fi
}
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