#!/bin/bash
# god forbid the installer 

# build it
cargo build --release

# move to a bin dir (simplified)
mkdir -p "$HOME/.local/bin"
cp target/release/ayuda "$HOME/.local/bin/ayuda"

# shell integration
AYUDA_FUNC='
ayuda() {
    if [[ "$1" == "go" ]]; then
        local target=$(command ayuda "$@")
        if [[ -d "$target" ]]; then
            cd "$target"
        else
            echo "$target"
        fi
    else
        command ayuda "$@"
    fi
}
'

# detect shell
case "$SHELL" in
    */zsh)
        CONFIG_FILE="$HOME/.zshrc"
        ;;
    */bash)
        CONFIG_FILE="$HOME/.bashrc"
        ;;
    *)
        CONFIG_FILE="$HOME/.profile"
        ;;
esac

if ! grep -q "ayuda() {" "$CONFIG_FILE"; then
    echo "$AYUDA_FUNC" >> "$CONFIG_FILE"
    echo "added shell integration to $CONFIG_FILE !"
fi

echo "ayuda installed. restart your shell or source $CONFIG_FILE !"
