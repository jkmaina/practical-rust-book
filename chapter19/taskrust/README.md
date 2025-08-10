# TaskRust

A command-line task manager written in Rust.

## Features

- Add tasks with descriptions, priorities, and due dates
- List tasks with various filtering options
- Mark tasks as complete
- Delete tasks
- Export/import tasks in JSON or CSV format
- Colorful terminal output
- Interactive confirmations

## Installation

```bash
cargo install taskrust
```

Or build from source:

```bash
git clone https://github.com/yourusername/taskrust.git
cd taskrust
cargo build --release
```

The binary will be available at `target/release/taskrust`.

## Usage

### Adding Tasks

```bash
# Add a task with default priority (medium) and no due date
taskrust add "Complete the Rust book"

# Add a task with high priority
taskrust add "Fix critical bug" --priority high

# Add a task with a due date
taskrust add "Submit report" --due 2025-05-15
```

### Listing Tasks

```bash
# List all incomplete tasks
taskrust list

# List high priority tasks
taskrust list --priority high

# List tasks due soon (within 3 days)
taskrust list --due-soon

# List completed tasks
taskrust list --completed
```

### Completing Tasks

```bash
# Mark task with ID 2 as completed
taskrust complete 2
```

### Deleting Tasks

```bash
# Delete task with ID 3
taskrust delete 3
```

### Exporting/Importing Tasks

```bash
# Export tasks to JSON
taskrust export tasks.json

# Export tasks to CSV
taskrust export tasks.csv

# Import tasks from a file
taskrust import tasks.json
```

## Development

### Running Tests

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test cli_tests
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## License

MIT