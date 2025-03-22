# Bitcoin Binary Router

A unified command-line interface for Bitcoin Core tools. This project provides a single `bitcoin` command that routes subcommands to their respective Bitcoin Core binaries.

## Overview

This tool provides a unified interface for Bitcoin Core tools. When you run `bitcoin <command>`, it automatically routes to the appropriate Bitcoin Core binary. For example:

- `bitcoin cli` routes to `bitcoin-cli`
- `bitcoin daemon` routes to `bitcoind` (special case)
- `bitcoin <command>` would route to `bitcoin-<command>` (if available)

## Prerequisites

### Runtime Requirements

- Bitcoin Core binaries installed and available in your PATH:
  - `bitcoin-cli`
  - `bitcoind`
  - Whatever `bitcoin-<command>` you want to invoke as `bitcoin command`

### Development Requirements

- Rust 1.85.1

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/bitcoin-bin.git
   cd bitcoin-bin
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Link the binary to your PATH:

   ```bash
   ln -s $(pwd)/target/release/bitcoin ~/.local/bin/bitcoin
   ```

## Usage

Basic usage:

```bash
bitcoin <COMMAND> [ARGS]...
```

Examples:

```bash
# Start Bitcoin daemon
bitcoin daemon --testnet

# Query blockchain info
bitcoin cli getblockchaininfo

# Get help for a subcommand
bitcoin cli --help
```

## Available Commands

- `cli`: Invoke bitcoin-cli (command-line RPC interface)
- `daemon`: Start the Bitcoin daemon (bitcoind)
- Any other command will be routed to `bitcoin-<command>` if available

## Development

1. Install dependencies:

   ```bash
   cargo build
   ```

2. Run tests:

   ```bash
   cargo test
   ```

## License

MIT License - See LICENSE file for details
