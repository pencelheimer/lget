# lget

![lget demo](assets/demo.gif)

A fast, interactive command-line tool to fetch open-source licenses directly from the [SPDX repository](https://github.com/spdx/license-list-data).

## Installation

```bash
cargo install lget
```

Or with nix (there is no binary cache yet):
```bash
nix run 'github:pencelheimer/lget'
```

## Usage

Run interactively to select a license from a TUI menu:
```bash
lget
```

Or pass a license directly to bypass the menu (great for scripts):
```bash
lget -l mit
```

Force overwrite an existing LICENSE file without prompting:
```bash
lget -f -l apache2
```

Suppress all output using the quiet flag:
```bash
lget -f -l gpl3 -q
```

## TODO
- Shell completions
- Man page
- TLDR page
