# Rust TCP Chat Application Monorepo

This repository contains two Rust projects:

- `chat_server`: A multi-client TCP chat server.
- `chat_client`: A command-line TCP chat client.

## Structure

- `chat_server/` — The server application
- `chat_client/` — The client application

See each folder for details, or read below for a quick start.

## Quick Start

### Prerequisites
- Rust (https://www.rust-lang.org/tools/install)

### Build and Run

#### Server
```zsh
cd chat_server
cargo run --release
```

#### Client
```zsh
cd chat_client
cargo run --release
```

You can run multiple clients in separate terminals. Type `:quit` to exit the client.

## License
MIT
