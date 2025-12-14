# Rust-ETL

## Overview

`cargo run`

after Rust-ETL starts up we generate the test data
run: `generate [format]`

Supported formats (default `csv`):

- `csv`
- `ndjson`

now to stream the data into the sqlite DB we simply run
`transform [format]`.

### Commands

#### Main

- g | gen | generate
- t | transform

#### Util

- h | help
- exit | quit | q
- c | cl | clean
