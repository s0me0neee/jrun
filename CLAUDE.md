# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`jrun2` is a CLI tool written in Rust that compiles and runs Java source files. It discovers all `java`/`javac` installations on the system, lets the user pick a specific toolchain version, and delegates compilation to `javac` via the `duct` crate.

## Commands

```bash
# Build
cargo build

# Build release
cargo build --release

# Compile and run a Java file (uses config defaults)
cargo run -- <path/to/file.java>

# List all detected Java/JavaC installations
cargo run -- --list

# Use a specific javac/jvm version (prefix match: "21" matches "21.0.2")
cargo run -- --javac 21 <file.java>
cargo run -- --javac 21 --jvm 17 <file.java>   # split compiler/runtime
cargo run -- --jvm 17 <file.java>               # config javac, explicit jvm

# Override output directory for .class files
cargo run -- --output <dir> <file.java>

# Persist the selected toolchain as the new default
cargo run -- --javac 21 --set-default

# Format (rustfmt.toml: max_width=80)
cargo fmt

# Lint
cargo clippy
```

`RUST_LOG=info` is set in `.env` and loaded at startup via `dotenvy`. Log output is controlled by the standard `RUST_LOG` env var.

## Architecture

### Entry point and startup (`src/main.rs`)

`main()` runs two initialization steps before doing any work:
1. **`setting_init()`** — reads or creates `setting.json` (via `default_setting_path!()` macro → `~/.config/jrun2/setting.json` on Linux/macOS). `Setting` holds only one field: the path to `config.json`.
2. **`config_init(path)`** — reads or creates `config.json` at the path from `setting.json`. `Config` stores the resolved paths to the default `java` and `javac` executables (detected from `$PATH` on first run via `which`).

### Config layer (`src/config.rs`)

Two structs serialized/deserialized as JSON:
- `Setting` — pointer to the config file location.
- `Config` — actual paths to `jvm_path` and `javac_path`.

Both use `serde_json` for I/O; no schema migration exists yet.

### Toolchain discovery (`src/versions.rs`)

- `find_jvm()` / `find_javac()` — call `which::which_all_global` to find every `java`/`javac` on `$PATH`, then probe each with `--version` to get the version string.
- `query()` — resolves a user-supplied version string or path to a concrete `Toolchain`. Matching is done by exact path, then by version prefix (e.g. `"21"` matches `"21.0.2"`).
- `list_available()` — prints discovered tools grouped by version.

### Compilation and execution (`src/java.rs`)

`compile(&toolchain, path, outpath)` invokes `javac` via `duct::cmd!`. Output defaults to a `build/` subdirectory next to the source file. `run(jvm, class_dir, class_name)` invokes the JVM with `-cp <class_dir> <class_name>`; the class name is derived from the source filename stem.

### File validation (`src/file.rs`)

`validate_file()` checks that the path exists, is not a directory, and has a `.java` extension. Also defines the `default_setting_path!` and `default_config_path!` macros used across modules.

### Logging macros (`src/log.rs`)

Three macros — `error!`, `warning!`, `info!` — that format colored terminal output using `owo_colors`. `info!` takes a tag as the first argument (e.g. `info!("Compile", "...")`).

## Known limitations

- Class name for `java::run` is derived from the source filename stem. Java requires the public class name to match the filename, so this works for standard files but not for packages or multi-class files where the entry point differs from the filename.
