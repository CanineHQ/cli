# Canine CLI

A command-line tool for interacting with the [Canine](https://canine.sh) platform. Create shells into running projects, list projects, and more.

## Installation

```bash
curl -sSL https://raw.githubusercontent.com/CanineHQ/cli/main/install.sh | sh
```

### macOS (Homebrew)

```bash
brew tap CanineHQ/canine
brew install canine
```

### From source

```bash
cargo install --path .
```

## Usage

### Authentication

Login with your API token:

```bash
canine auth login --token <YOUR_TOKEN>
```

Optionally specify a custom host:

```bash
canine auth login --token <YOUR_TOKEN> --host https://your-host.com
```

Check your authentication status:

```bash
canine auth status
```

Logout:

```bash
canine auth logout
```

Credentials are stored in `~/.canine/canine.yaml`.

### Projects

List your projects:

```bash
canine project list
canine project list --all    # Include archived projects
canine project list --json   # Output as JSON
```

Open a shell into a project:

```bash
canine project shell --name <PROJECT_NAME>
canine project shell --name <PROJECT_NAME> --container <CONTAINER_NAME>
```
