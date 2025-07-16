# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a command-line todo application written in Rust. It allows users to create, list, and view todo items stored as individual files in the filesystem.

## Common Commands

### Building and Running
```bash
cargo build          # Build the project
cargo run            # Run the application
cargo run -- [cmd]   # Run with specific command
```

### Testing
```bash
cargo test           # Run tests
cargo run -- test   # Run the application's test command
```

### Development
```bash
cargo check          # Check for compilation errors without building
cargo clippy         # Run linting
cargo fmt            # Format code
```

## Application Commands

The todo application supports these commands:
- `add` - Add a new todo item (interactive)
- `list` - List all todo files
- `list -c` - List all todo files with their contents
- `view [hash]` - View content of a specific todo file by hash ID
- `edit` - Edit functionality (not implemented yet)
- `help` - Show available commands
- `test` - Test command

## Code Architecture

### Core Structure
- `main.rs` - Entry point with command parsing and routing
- `commands.rs` - Command implementations and core logic
- `commands/file_format.rs` - TodoFile struct and file operations

### Key Components

**TodoFile Structure** (`src/commands/file_format.rs`):
- Represents a todo item with hash, timestamp, content, tags, and connections
- Uses hash-based identification (generated from content + timestamp)
- Handles serialization to custom file format
- Manages tag operations (add/remove/list)
- Manages connection operations (add/remove/list) for linking todo items

**Command System** (`src/commands.rs`):
- `add()` - Interactive todo creation with hash-based identification
- `list()` - Directory listing with hash IDs, optional content display
- `view()` - File content reading and display using hash parameter
- `help()` - Command documentation

### File Storage
- Todo files stored in `added/` directory with `.todo` extension
- Hash-based filenames: `{hash}.todo`
- Custom file format with sections: [content], [timestamp], [tags], [connections]

### Dependencies
- `chrono` - For timestamp generation

## Development Notes

- The `edit` command is marked as `todo!()` and not implemented
- Error handling uses `std::io::Result` throughout
- File operations create directories as needed
- Hash-based identification ensures unique todo identifiers
- Connection system allows linking related todo items