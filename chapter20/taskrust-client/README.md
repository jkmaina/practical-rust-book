# TaskRust Client

A command-line client for the TaskRust API, demonstrating HTTP client implementation and REST API consumption in Rust.

## Features

- **Full API Coverage**: Supports all TaskRust API endpoints
- **User-Friendly CLI**: Intuitive command-line interface with colored output
- **Error Handling**: Comprehensive error handling for network and API issues
- **Type Safety**: Shared data models ensure consistency with the server
- **Async Operations**: Non-blocking HTTP requests using reqwest and tokio

## Installation

```bash
cd taskrust-client
cargo build --release
```

## Usage

Make sure the TaskRust API server is running on `http://localhost:8080` (or specify a different server with `--server`).

### Basic Commands

```bash
# Show help
cargo run -- --help

# Show API information
cargo run -- info

# List all tasks
cargo run -- list

# Create a new task
cargo run -- create "Learn Rust HTTP clients" -d "Build a client for the TaskRust API" -p high

# Get a specific task by ID
cargo run -- get <task-id>

# Update a task
cargo run -- update <task-id> -t "Updated title" --completed true

# Mark a task as completed
cargo run -- complete <task-id>

# Delete a task
cargo run -- delete <task-id>
```

### Advanced Usage

```bash
# Use a different server
cargo run -- --server http://api.example.com list

# Create a task with due date
cargo run -- create "Submit report" -d "Quarterly report" -p high --due 2025-03-15

# Update multiple fields
cargo run -- update <task-id> -t "New title" -d "New description" -p low --completed true
```

## Command Reference

| Command | Description | Arguments |
|---------|-------------|-----------|
| `info` | Show API information | None |
| `list` | List all tasks | None |
| `get <id>` | Get specific task | Task UUID |
| `create <title>` | Create new task | Title (required), -d description, -p priority, --due date |
| `update <id>` | Update existing task | Task UUID, -t title, -d description, -c completed, -p priority, --due date |
| `complete <id>` | Mark task as completed | Task UUID |
| `delete <id>` | Delete task | Task UUID |

## Options

- `--server <URL>`: API server URL (default: http://localhost:8080)
- `-h, --help`: Show help information

## Examples

### Creating Tasks
```bash
# Simple task
cargo run -- create "Buy groceries"

# Task with description and priority
cargo run -- create "Fix bug #123" -d "Critical authentication issue" -p high

# Task with due date
cargo run -- create "Prepare presentation" -p medium --due 2025-02-20
```

### Managing Tasks
```bash
# List all tasks to see IDs
cargo run -- list

# Get detailed information about a task
cargo run -- get a1b2c3d4-e5f6-7890-abcd-ef1234567890

# Update task status
cargo run -- complete a1b2c3d4-e5f6-7890-abcd-ef1234567890

# Update multiple fields
cargo run -- update a1b2c3d4-e5f6-7890-abcd-ef1234567890 -t "Updated title" -p low
```

## Error Handling

The client provides clear error messages for common issues:

- **Network errors**: Connection refused, timeout, etc.
- **API errors**: 404 Not Found, 400 Bad Request, etc.
- **Input validation**: Invalid UUIDs, date formats, priority values
- **JSON parsing**: Malformed server responses

## Output Format

The client uses colored output to enhance readability:
- 🚀 **Blue**: Application header
- 📡 **Cyan**: Operation status
- ✅ **Green**: Success messages
- ❌ **Red**: Error messages
- ⏳ **Yellow**: Pending/incomplete tasks
- **Dimmed**: Completed tasks

Task priorities are color-coded:
- **Red**: High priority
- **Yellow**: Medium priority
- **Green**: Low priority

## Development

### Project Structure
```
src/
├── main.rs         # Application entry point
├── models.rs       # Shared data models
├── client.rs       # HTTP client implementation
├── cli.rs          # Command-line interface
└── commands.rs     # Command implementations
```

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with logging
RUST_LOG=debug cargo run -- list
```

### Testing with Server
```bash
# Terminal 1: Start the API server
cd ../taskrust-api
cargo run

# Terminal 2: Use the client
cd ../taskrust-client
cargo run -- list
```

## Dependencies

- **reqwest**: HTTP client library
- **tokio**: Async runtime
- **serde**: JSON serialization/deserialization
- **clap**: Command-line argument parsing
- **chrono**: Date and time handling
- **uuid**: UUID support
- **anyhow**: Error handling
- **colored**: Terminal colors

## License

MIT