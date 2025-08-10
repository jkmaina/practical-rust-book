# Yew Counter Example

A simple counter application built with Yew (Rust frontend framework).

## How to Run

### Prerequisites
- Rust (latest stable)
- `trunk` (for serving Yew applications)

### Install Trunk
```bash
cargo install trunk
```

### Run the Application
```bash
# In the project directory
trunk serve
```

Then open your browser to: http://localhost:8080

### Build for Production
```bash
trunk build --release
```

## What This Demonstrates

- **Yew Framework**: Rust-based frontend framework
- **WebAssembly**: Rust compiled to run in browsers
- **Component State**: Using `use_state` hook
- **Event Handling**: Button click callbacks
- **HTML Templating**: Using the `html!` macro

## Features

- ➕ Increment counter
- ➖ Decrement counter  
- 🔄 Reset to zero
- Responsive design with CSS styling

## Project Structure

```
02_yew_example/
├── src/
│   └── main.rs          # Main Yew application
├── index.html           # HTML template
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Troubleshooting

If you get errors:

1. **Install trunk**: `cargo install trunk`
2. **Check Rust version**: `rustup update`
3. **Clear cache**: `cargo clean`
4. **Rebuild**: `trunk serve`

The application will automatically reload when you make changes to the code.