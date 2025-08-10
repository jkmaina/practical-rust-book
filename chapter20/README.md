# Chapter 20: TaskRust Complete System

A complete web service ecosystem built with Rust, demonstrating both server and client development.

## 🏗️ System Architecture

```
TaskRust Ecosystem
├── taskrust-api/          # RESTful API Server (Actix Web)
├── taskrust-client/       # Command-line Client (reqwest)
├── Scripts/               # Testing and demo scripts
│   ├── test_api.ps1      # PowerShell API tests
│   ├── test_api.sh       # Bash API tests
│   ├── demo_system.ps1   # PowerShell system demo
│   ├── demo_system.sh    # Bash system demo
│   └── run_examples.sh   # Bash system manager
└── Documentation/         # Comprehensive docs
```

## 🚀 Quick Start

### For Linux/macOS Users

1. **Make scripts executable**:
   ```bash
   chmod +x *.sh
   chmod +x taskrust-api/test_api.sh
   ```

2. **Use the system manager**:
   ```bash
   # Start the server
   ./run_examples.sh server
   
   # Run the client
   ./run_examples.sh client
   
   # Run tests
   ./run_examples.sh test
   
   # Run complete demo
   ./run_examples.sh demo
   
   # Stop the server
   ./run_examples.sh stop
   ```

### For Windows Users

1. **Start the server**:
   ```powershell
   cd taskrust-api
   cargo run
   ```

2. **Run the client** (in another terminal):
   ```powershell
   cd taskrust-client
   cargo run -- list
   ```

3. **Run tests**:
   ```powershell
   powershell -ExecutionPolicy Bypass -File test_api.ps1
   ```

## 📋 Available Scripts

### Cross-Platform Scripts

| Script | Windows | Linux/macOS | Description |
|--------|---------|-------------|-------------|
| API Tests | `test_api.ps1` | `test_api.sh` | Test all API endpoints |
| System Demo | `demo_system.ps1` | `demo_system.sh` | Complete system demonstration |
| System Manager | ❌ | `run_examples.sh` | All-in-one system management |

### Linux/macOS System Manager Commands

```bash
./run_examples.sh <command>

Commands:
  server              Start the TaskRust API server
  stop                Stop the TaskRust API server  
  client [args]       Run the TaskRust client
  test                Run API tests
  demo                Run complete system demo
  build               Build both server and client
  status              Check system status
  help                Show help message
```

## 🎯 Usage Examples

### Basic Operations

```bash
# Linux/macOS
./run_examples.sh server                    # Start server
./run_examples.sh client                    # List tasks
./run_examples.sh client create "My task"   # Create task
./run_examples.sh test                      # Run tests
./run_examples.sh stop                      # Stop server

# Windows
cd taskrust-api && cargo run               # Start server
cd taskrust-client && cargo run -- list    # List tasks
cargo run -- create "My task"              # Create task
```

### Advanced Client Usage

```bash
# Create task with details
./run_examples.sh client create "Important task" -d "Task description" -p high --due 2025-03-15

# Update task
./run_examples.sh client update <task-id> --completed true

# Get task details
./run_examples.sh client get <task-id>

# Delete task
./run_examples.sh client delete <task-id>
```

## 🧪 Testing

### API Testing

The system includes comprehensive API tests:

**Linux/macOS:**
```bash
./run_examples.sh test
# or directly:
cd taskrust-api && ./test_api.sh
```

**Windows:**
```powershell
cd taskrust-api
powershell -ExecutionPolicy Bypass -File test_api.ps1
```

### System Demo

Run the complete system demonstration:

**Linux/macOS:**
```bash
./run_examples.sh demo
```

**Windows:**
```powershell
powershell -ExecutionPolicy Bypass -File demo_system.ps1
```

## 🔧 System Requirements

### Required
- **Rust** (latest stable)
- **Cargo** (comes with Rust)
- **curl** (for API testing)

### Optional
- **jq** (for better JSON formatting in tests)
  - Ubuntu/Debian: `sudo apt-get install jq`
  - macOS: `brew install jq`
  - Windows: Download from [jq website](https://jqlang.github.io/jq/)

## 📊 System Status

Check the current system status:

```bash
# Linux/macOS
./run_examples.sh status

# Windows
# Check manually:
# - Server: http://localhost:8080/
# - Client: cargo run -- info
```

## 🏃‍♂️ Development Workflow

### Typical Development Session

1. **Start the system**:
   ```bash
   ./run_examples.sh server
   ```

2. **Develop and test**:
   ```bash
   ./run_examples.sh client create "Test feature X"
   ./run_examples.sh test
   ```

3. **Run demos**:
   ```bash
   ./run_examples.sh demo
   ```

4. **Clean shutdown**:
   ```bash
   ./run_examples.sh stop
   ```

### Building from Source

```bash
# Build everything
./run_examples.sh build

# Or build individually
cd taskrust-api && cargo build
cd taskrust-client && cargo build
```

## 🎓 Learning Objectives

This complete system demonstrates:

1. **RESTful API Development** with Actix Web
2. **HTTP Client Implementation** with reqwest
3. **CLI Application Design** with clap
4. **Error Handling** across network boundaries
5. **Type Safety** in distributed systems
6. **Cross-Platform Development** with Rust
7. **Testing Strategies** for web services
8. **Documentation** and user experience

## 🔍 Troubleshooting

### Common Issues

1. **Server won't start**:
   - Check if port 8080 is available
   - Ensure no other TaskRust instance is running

2. **Client can't connect**:
   - Verify server is running: `curl http://localhost:8080/`
   - Check firewall settings

3. **Permission denied on scripts**:
   ```bash
   chmod +x *.sh
   chmod +x taskrust-api/test_api.sh
   ```

4. **jq not found warnings**:
   - Install jq for better JSON formatting
   - Scripts work without jq but with less formatting

### Getting Help

```bash
# System help
./run_examples.sh help

# Client help
./run_examples.sh client --help

# Specific command help
./run_examples.sh client create --help
```

## 📚 Further Reading

- [TaskRust API Documentation](taskrust-api/README.md)
- [TaskRust Client Documentation](taskrust-client/README.md)
- [Chapter 20 Implementation Summary](taskrust-api/CHAPTER_SUMMARY.md)
- [Chapter 20 Continuation](CONTINUATION.md)

## 🎉 Conclusion

This complete TaskRust system showcases modern Rust development practices for building robust, type-safe web services and clients. The cross-platform scripts ensure a smooth experience regardless of your operating system.

Happy coding! 🦀