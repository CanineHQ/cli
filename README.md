# Canine CLI

A command-line tool for interacting with the [Canine](https://canine.sh) platform. Create shells into running projects, list projects, and more.

## Installation

```bash
cargo install --path .
```

## Usage

### Authentication

Login with your API token:

```bash
k9 auth login --token <YOUR_TOKEN>
```

Optionally specify a custom host:

```bash
k9 auth login --token <YOUR_TOKEN> --host https://your-host.com
```

Check your authentication status:

```bash
k9 auth status
```

Logout:

```bash
k9 auth logout
```

Credentials are stored in `~/.canine/canine.yaml`.

### Projects

List your projects:

```bash
k9 project list
k9 project list --all    # Include archived projects
k9 project list --json   # Output as JSON
```

Open a shell into a project:

```bash
k9 project shell --name <PROJECT_NAME>
k9 project shell --name <PROJECT_NAME> --container <CONTAINER_NAME>
```
