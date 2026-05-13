# ayuda

> "because we're too lazy to use man and too proud to use a gui"

AYUDA is a CLI tool for people who spend too much time in the terminal and want a little more personality (and spite) in their workflow.

## modules

- **`ayuda calc`**: math evaluator that judges your inability to do basic arithmetic. handles `x` instead of `*` because we're human.
- **`ayuda go`**: fuzzy directory teleporter. remembers where you've been. (requires shell integration).
- **`ayuda ? <cmd>`**: explains commands. has a safety trigger for `rm -rf /` that makes you type humiliating phrases.
- **`ayuda hack [target]`**: simulates a 90s movie breach. usually results in an opsec failure because you forgot your vpn proxy tunnel.

## installation

```bash
curl -sSL https://raw.githubusercontent.com/hnpf/ayuda/master/scripts/install.sh | bash
```

## dev

requires:
- Rust (Cargo)
- C++ (std17)
- ncurses

build:
```bash
cargo build
```

## license
gpl-3.0
