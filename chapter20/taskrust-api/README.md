# TaskRust API

A RESTful web service for task management built with Rust and Actix Web.

## Features

- **RESTful API**: Full CRUD operations for task management
- **JSON API**: All requests and responses use JSON format
- **Type Safety**: Leverages Rust's type system for robust error handling
- **High Performance**: Built with Actix Web for excellent performance
- **UUID Support**: Uses UUIDs for task identification
- **Logging**: Comprehensive request logging with env_logger

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | API information and documentation |
| GET | `/tasks` | List all tasks |
| GET | `/tasks/{id}` | Get a specific task by ID |
| POST | `/tasks` | Create a new task |
| PUT | `/tasks/{id}` | Update an existing task |
| DELETE | `/tasks/{id}` | Delete a task |

## Data Models

### Task
```json
{
  "id": "uuid",
  "title": "string",
  "description": "string (optional)",
  "completed": "boolean",
  "created_at": "datetime",
  "due_date": "datetime (optional)",
  "priority": "Low | Medium | High"
}
```

### Create Task Request
```json
{
  "title": "string (required)",
  "description": "string (optional)",
  "due_date": "datetime (optional)",
  "priority": "Low | Medium | High (optional, defaults to Medium)"
}
```

### Update Task Request
```json
{
  "title": "string (optional)",
  "description": "string (optional)",
  "completed": "boolean (optional)",
  "due_date": "datetime (optional)",
  "priority": "Low | Medium | High (optional)"
}
```

## Quick Start

1. **Clone and build**:
   ```bash
   cd taskrust-api
   cargo build
   ```

2. **Run the server**:
   ```bash
   RUST_LOG=info cargo run
   ```

3. **Test the API**:
   ```bash
   # Get API info
   curl http://localhost:8080/
   
   # List all tasks
   curl http://localhost:8080/tasks
   ```

## Usage Examples

### Create a Task
```bash
curl -X POST http://localhost:8080/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn Rust web development",
    "description": "Build a RESTful API with Actix Web",
    "priority": "High"
  }'
```

### List All Tasks
```bash
curl http://localhost:8080/tasks
```

### Get a Specific Task
```bash
curl http://localhost:8080/tasks/{task-id}
```

### Update a Task
```bash
curl -X PUT http://localhost:8080/tasks/{task-id} \
  -H "Content-Type: application/json" \
  -d '{
    "completed": true,
    "priority": "Low"
  }'
```

### Delete a Task
```bash
curl -X DELETE http://localhost:8080/tasks/{task-id}
```

## Configuration

The application can be configured using environment variables:

- `SERVER_ADDR`: Server bind address (default: `127.0.0.1:8080`)
- `LOG_LEVEL`: Logging level (default: `info`)
- `RUST_ENV`: Environment name (default: `development`)

Example:
```bash
SERVER_ADDR=0.0.0.0:3000 LOG_LEVEL=debug cargo run
```

## Development

### Project Structure
```
src/
├── main.rs         # Application entry point
├── models.rs       # Data models and types
├── handlers.rs     # HTTP request handlers
├── repository.rs   # Data access layer
├── errors.rs       # Error types and handling
└── config.rs       # Configuration management
```

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing
```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo test
```

## Sample Data

The application starts with sample tasks for testing:
- "Complete Rust web service" (High priority)
- "Write API documentation" (Medium priority)  
- "Add unit tests" (Low priority)

## Error Handling

The API returns appropriate HTTP status codes:
- `200 OK`: Successful GET/PUT requests
- `201 Created`: Successful POST requests
- `204 No Content`: Successful DELETE requests
- `400 Bad Request`: Invalid request data
- `404 Not Found`: Task not found
- `500 Internal Server Error`: Server errors

Error responses include a JSON object with error details:
```json
{
  "error": "Task with ID {id} not found"
}
```

## License

MIT