# paste these functions in .bashrc file
loadbin(){
    local bindirname="compiled_own_binaries"
    local kahan_they="$PWD"
    cd "$HOME"
    mkdir -p "$bindirname"
    cd "$bindirname/"
    rm -f -- *
    cod
    mkdir -p "$bindirname"
    cd "$bindirname/"
    for f in *.axe; do
        [ "$f" = "compileit.axe" ] && continue
        mv -- "$f" "${f%.axe}"
    done
    cd ..
    cp "$bindirname/"* "$HOME/$bindirname/" 2>/dev/null
    chmod +x "$HOME/$bindirname/"* 2>/dev/null
    cd "$kahan_they"
    echo "binary loaded"
}
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
