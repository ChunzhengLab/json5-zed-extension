# JSON5 Language Support for Zed

Adds JSON5 language support to the [Zed](https://zed.dev) code editor using
[tree-sitter-json5](https://github.com/Joakker/tree-sitter-json5).

## Features

- Syntax highlighting for comments, trailing commas, single-quoted strings,
  unquoted keys, hexadecimal numbers, `Infinity`, `NaN`, and other JSON5 syntax
- Bracket matching and automatic indentation
- Document outline entries for object properties
- Comment text objects in Vim mode
- Automatic activation for `.json5` files

## Installation

Install **JSON5** from Zed's Extensions view (`zed: extensions`).

To test a development checkout, run `zed: install dev extension` from the
command palette and select this repository.

## Development

After changing the grammar revision or Tree-sitter query files, install the
repository as a dev extension and open [`testdata/sample.json5`](testdata/sample.json5).
Check `zed: open log` if the extension or a query fails to load.

Published releases must increment `version` in `extension.toml`. The matching
version and submodule commit must then be updated in
[`zed-industries/extensions`](https://github.com/zed-industries/extensions).

See [CHANGELOG.md](CHANGELOG.md) for release notes.

## Credits

This extension uses the [tree-sitter-json5](https://github.com/Joakker/tree-sitter-json5) grammar by [@Joakker](https://github.com/Joakker).  
All grammar-level credit goes to the original author — this project only integrates it into Zed.

## License

[MIT](LICENSE)
