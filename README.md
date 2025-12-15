# Rust ETL Challenge

## 🚀 Overview

A high-performance ETL tool written in Rust that streams large datasets (CSV/NDJSON), applies transformations, and loads them into SQLite without eating up RAM.

## Key Features:

- **Streaming I/O:** Processes 100k+ records with constant memory usage.

- **Dual Operation:** Supports One-Shot (execute & exit) or Interactive Shell (REPL) for rapid testing.

- **Smart CLI:** Includes command aliases `(g, t)`, built-in `help`, and environment cleanup with `clean`.

- **Formats:** Full support for .csv and .ndjson.
- **Configurable Scale:** Easily adjust dataset size via `.env` variable (no recompile needed).

## 🛠️ Build & Run

Ensure you have Rust installed.

```bash
cargo build --release
```

## 🎮 Usage

1. Generate Data
   Creates a dummy dataset with >100k records.

Syntax: `cargo run --release -- generate <format>`

Examples:

```Bash
# Default (CSV)
cargo run --release -- generate

# NDJSON
cargo run --release -- generate ndjson
```

2. Run ETL Pipeline
   Streams the data, transforms it, and writes to output.db.

Syntax: cargo run --release -- transform <format>

Examples:

```Bash
# Process the default CSV file
cargo run --release -- transform

# Process the NDJSON file
cargo run --release -- transform ndjson
```

### ⌨️ Interactive Mode (REPL)

Run the tool without arguments to enter the command shell. Useful for running multiple workflows without recompiling/restarting.

```Bash
cargo run --release
```

| Command              | Alias | Description                                       |
| :------------------- | :---- | :------------------------------------------------ |
| `generate [format]`  | `g`   | Creates dummy dataset.                            |
| `transform [format]` | `t`   | Runs the ETL pipeline.                            |
| `clean`              | `cl`  | **Resets environment** (deletes DB & data files). |
| `help`               | `h`   | Shows available commands.                         |
| `exit`               | `q`   | Closes the application.                           |

## 📊 Sample Metrics

Typical performance on [Insert Your CPU Here]:

```
---------------------------------
Processing Metrics:
---------------------------------
Total records processed : 200000
Successful rows written : 174843
Failed rows (skipped)   : 25157
Total duration          : 1.53s
Throughput              : 114593 rows/sec
---------------------------------
```

---
