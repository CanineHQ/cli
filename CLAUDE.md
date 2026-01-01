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

Canine CLI (`canine`/`k9`) is a Rust command-line tool for interacting with the Canine platform (canine.sh). It manages projects, clusters, authentication, and account switching.

### Key Files

- `src/main.rs` - CLI entry point using clap for argument parsing. Defines command structure (`Namespace` → subcommands → actions) and handles `CanineConfig` for storing credentials in `~/.k9/canine.yaml`.
- `src/client.rs` - HTTP client (`CanineClient`) for API communication using reqwest. Handles authentication via `X-API-KEY` header and account selection via `X-ACCOUNT-ID` header.
- `src/kubeconfig.rs` - Kubernetes config schema and helpers. Parses/serializes kubeconfig YAML for cluster access. Also validates kubectl installation.

### CLI Structure

```
k9
├── auth
│   ├── login --token <TOKEN> [--host <HOST>] [--account <ACCOUNT>]
│   ├── status
│   └── logout
├── account
│   └── change-account <ACCOUNT>
├── project
│   ├── list [--all] [--json]
│   ├── shell --project <PROJECT>
│   ├── deploy --name <NAME> [--skip-build]
│   └── processes --project <PROJECT>
└── cluster
    └── download-kubeconfig --name <NAME>
```

### Authentication & Config

Credentials stored at `~/.k9/canine.yaml` with `host`, `token`, and `account` fields. Kubeconfig saved to `~/.k9/kubeconfig.yaml`. Default API host: `https://canine.sh`.
