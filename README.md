# TicTacToe with Rust
A tictactoe implementation on the terminal (minimax algorithm), written in [Rust](https://www.rust-lang.org).

## Features
- Grid display on the terminal
- Bot or pass-to-play
- 'X'/'O' players

## Requirements
- **Rust** (stable) — install via [rustup](https://rustup.rs)

## Building the project
Clone the repository and build it in release mode:
```bash
git clone https://github.com/CieriA/minimax-tictactoe
cd minimax-tictactoe
cargo build --release
```

## Running the game
```bash
cargo run --release
```

## Controls
- 1-9 -> to index the board from the terminal

## Development notes
This project uses the following crates:
- colored

### Docs
To build the documentation locally:
```bash
cargo doc --open
```

## License
This project is licensed under the ISC License. For more info see the [LICENSE](LICENSE) file.
