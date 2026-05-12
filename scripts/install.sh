#!/bin/bash
# god forbid the installer 

# helpfully failing if things are missing
command -v cargo >/dev/null 2>&1 || { echo >&2 "cargo not found. install rust first. or dont, i'm just a script."; exit 1; }
command -v g++ >/dev/null 2>&1 || { echo >&2 "g++ not found. need it for the c++ math bridge."; exit 1; }

echo "this script will automatically install ayuda."
for i in {5..1}; do
    echo -ne "starting in $i seconds... (ctrl+c to cancel)\r"
    sleep 1
done
echo -e "\n--- continuing install process! ---"

TMP_DIR=$(mktemp -d)
echo "--- cloning to $TMP_DIR ---"
git clone https://github.com/hnpf/ayuda.git "$TMP_DIR" --depth 1 || { echo "clone failed. maybe check your internet?"; exit 1; }

cd "$TMP_DIR" || exit 1

echo "--- building (this takes a sec, go grab coffee) ---"
cargo build --release || { echo "build failed. maybe you're missing ncurses-dev?"; exit 1; }

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

# detect shell and pick a config file that actually exists
if [[ "$SHELL" == */zsh ]]; then
    CONFIG_FILE="$HOME/.zshrc"
elif [[ "$SHELL" == */bash ]]; then
    CONFIG_FILE="$HOME/.bashrc"
    [[ ! -f "$CONFIG_FILE" ]] && CONFIG_FILE="$HOME/.bash_profile"
else
    CONFIG_FILE="$HOME/.profile"
fi

touch "$CONFIG_FILE"

if ! grep -q "ayuda() {" "$CONFIG_FILE"; then
    echo "$AYUDA_FUNC" >> "$CONFIG_FILE"
    echo "--- added shell integration to $CONFIG_FILE ---"
fi

echo "--- installation finished ---"
echo "ayuda is now in ~/.local/bin/ayuda"
echo "restart your shell or run: source $CONFIG_FILE"

# cleanup
rm -rf "$TMP_DIR"
