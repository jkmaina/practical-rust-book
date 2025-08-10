# Practical Rust: Real-World Applications and Programming

<div align="center">
  <img src="https://m.media-amazon.com/images/I/71MfL6auRLL._SL1500_.jpg" alt="Practical Rust Book Cover" width="300"/>
</div>

## About This Repository

This repository contains all the source code examples from **"Practical Rust: Real-World Applications and Programming"** by **James Karanja Maina**. The book takes a hands-on approach to learning Rust through building real applications rather than focusing on abstract concepts.

📚 **Get the book**: [Amazon Kindle Edition](https://www.amazon.com/Practical-Rust-Real-World-Applications-Programming-ebook/dp/B0FGHSS6LZ)

## What You'll Build

This isn't just another programming book with toy examples. You'll build real applications including:

- **Command-line tools** with argument parsing and file I/O
- **Web APIs** using Actix Web with proper error handling
- **HTTP clients** with colored terminal output
- **WebAssembly applications** for the browser
- **Frontend web apps** using the Yew framework
- **2D games** with the Bevy game engine
- **Data science pipelines** with CSV processing and visualization
- **Concurrent systems** with threading and async programming
- **Network applications** including TCP servers and HTTP clients

## Repository Structure

```
├── chapter1/          # Hello World and Rust basics
├── chapter2/          # Variables, data types, and operators
├── chapter3/          # Control flow and loops
├── chapter4/          # Functions and documentation
├── chapter5/          # Ownership and memory management
├── chapter6/          # References and borrowing
├── chapter7/          # Lifetimes
├── chapter8/          # Structs and methods
├── chapter9/          # Enums and pattern matching
├── chapter10/         # Modules and project organization
├── chapter11/         # Error handling
├── chapter12/         # Generics
├── chapter13/         # Traits
├── chapter14/         # Collections and iterators
├── chapter15/         # Smart pointers
├── chapter16/         # Concurrency and threading
├── chapter17/         # Testing
├── chapter18/         # Async programming
├── chapter19/         # TaskRust CLI - Complete task manager
├── chapter20/         # TaskRust API & Client - Web services
└── chapter21/         # Modern Rust ecosystem projects
```

## Getting Started

### Prerequisites

- **Rust** (latest stable version) - Install from [rustup.rs](https://rustup.rs/)
- **Git** for cloning the repository
- **Node.js** (for WebAssembly examples)
- **Python** (for development servers)

### Installation

```bash
# Clone the repository
git clone https://github.com/jmaina/practical-rust-book.git
cd practical-rust-book

# Each chapter contains multiple examples
cd chapter1/01_hello_world
cargo run

# For web examples
cd chapter20/taskrust-api
cargo run

# For WebAssembly examples
cd chapter21/01_wasm-example
wasm-pack build --target web
python serve.py  # or node serve.js
```

## Key Features

### 🛠️ **Production-Ready Code**
Every example follows Rust best practices and includes proper error handling, testing, and documentation.

### 🔄 **Cross-Platform**
All examples work on Windows, Linux, and macOS. Includes both PowerShell and bash scripts where needed.

### 📦 **Modern Ecosystem**
Uses current versions of popular crates:
- `clap` for CLI applications
- `actix-web` for web servers
- `reqwest` for HTTP clients
- `tokio` for async programming
- `yew` for frontend applications
- `bevy` for game development
- `serde` for serialization

### 🧪 **Comprehensive Testing**
Examples include unit tests, integration tests, and documentation tests showing how to properly test Rust applications.

## Highlighted Projects

### TaskRust CLI (Chapter 19)
A complete command-line task manager with:
- CRUD operations for tasks
- JSON file persistence
- Comprehensive error handling
- Full test coverage
- Cross-platform compatibility

### TaskRust API & Client (Chapter 20)
Full client-server architecture featuring:
- RESTful API with Actix Web
- HTTP client with colored output
- Proper error handling and logging
- JSON serialization with Serde

### Modern Rust Applications (Chapter 21)
Diverse examples showcasing Rust's versatility:
- **WebAssembly**: Browser-based applications
- **Yew Frontend**: Reactive web applications
- **Bevy Game**: 2D game with ECS architecture
- **Data Science**: CSV processing and visualization
- **Networking**: TCP servers and HTTP clients
- **Containerization**: Docker deployment

## Learning Path

1. **Start with Basics** (Chapters 1-4): Get comfortable with Rust syntax
2. **Master Core Concepts** (Chapters 5-9): Understand ownership, borrowing, and Rust's type system
3. **Advanced Features** (Chapters 10-16): Learn modules, error handling, generics, and concurrency
4. **Testing & Async** (Chapters 17-18): Write reliable, concurrent code
5. **Real Applications** (Chapters 19-21): Build production-ready systems

## Running Examples

Each chapter contains multiple examples. To run any example:

```bash
cd chapter_name/example_name
cargo run
```

For examples with additional setup requirements, check the individual README files in each directory.

## Contributing

Found an issue or want to improve an example? Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Support

- **Issues**: Report bugs or request features via GitHub Issues
- **Discussions**: Join the community discussions
- **Email**: Contact the author at practical-rust@zavora.ai

## About the Author

**James Karanja Maina** is a software engineer with extensive experience in systems programming and web development. This book represents his approach to teaching Rust through practical, real-world applications rather than abstract concepts.

## License

The code in this repository is provided for educational purposes. Please refer to the book for complete explanations and context.

---

**Happy Coding! 🦀**

*Remember: The best way to learn Rust is by building real applications. Each example in this repository is designed to teach you practical skills you'll use in production code.*