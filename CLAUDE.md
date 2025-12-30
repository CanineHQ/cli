# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build              # Build the project
cargo build --release    # Build optimized release binary
cargo run -- <args>      # Run the CLI with arguments
cargo test               # Run all tests
cargo test <test_name>   # Run a single test
cargo clippy             # Run linter
cargo fmt                # Format code
```

## Architecture

Canine CLI (`k9`) is a Rust command-line tool for interacting with the Canine platform (canine.sh). It allows users to manage projects and authentication.

### Key Files

- `src/main.rs` - CLI entry point using clap for argument parsing. Defines command structure (`Namespace` → `AuthCmd`/`ProjectCmd` → actions) and handles `CanineConfig` for storing credentials in `~/.canine/canine.yaml`.
- `src/client.rs` - HTTP client (`CanineClient`) for API communication using reqwest. Handles authentication via API key header (`X-API-KEY`) and defines error types (`CanineError`, `ApiError`).

### CLI Structure

```
k9
├── auth
│   ├── login --token <TOKEN> [--host <HOST>]
│   ├── status
│   └── logout
└── project
    ├── shell --name <NAME> [--container <CONTAINER>]
    └── list [--all] [--json]
```

### Authentication

Credentials are stored in YAML format at `~/.canine/canine.yaml` with `host` and `token` fields. The default API host is `https://canine.sh`. Environment variable `CANINE_API_TOKEN` is also supported.
