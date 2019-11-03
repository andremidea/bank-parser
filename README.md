# bank-parser

Simple CLI meant to translate pre-process CSVs exported from different banks using a declarative TOML file.

## Goal

Have one descriptive to declare the shape of the input, pre-processing steps (e.g: converting dates, renaming merchants, etc) and the shape of the output + post-processing steps (e.g:  automatically categorizing items)

## Usage
```bash
cargo run --bin main -- --config src/n26.toml /path-to.csv
```

## Supported banks
Currently there's only one parser created for [N26](https://n26.com)

- [ ]: Monzo
- [ ]: Revolut
