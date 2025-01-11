# Terminal Ping Pong Game

A simple terminal-based ping pong game written in Rust using the crossterm library for terminal manipulation.

## Features

- Two-player gameplay
- Real-time paddle movement
- Score tracking
- Dynamic ball speed that increases during rallies
- Simple ASCII graphics
- Smooth terminal rendering

## Prerequisites

Before running this game, make sure you have:
- Rust installed (https://rustup.rs/)
- Cargo (Rust's package manager, comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone [your-repository-url]
cd ping-pong
```

2. Build the project:
```bash
cargo build
```

## Running the Game

To start the game, run:
```bash
cargo run
```

## Controls

- Left Paddle:
  - 'W' key: Move Up
  - 'S' key: Move Down

- Right Paddle:
  - Up Arrow: Move Up
  - Down Arrow: Move Down

- General:
  - 'Q': Quit the game

## Game Rules

- Each player controls a paddle on their side of the screen
- The ball bounces off paddles and walls
- If a player misses the ball, their opponent scores a point
- The ball speeds up each time it hits a paddle
- The game continues until a player quits

## Project Structure

```
ping_pong/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

## Dependencies

This project uses the following dependencies:
- crossterm (0.25): For terminal manipulation and input handling
- rand (0.8): For random ball direction after scoring

## Contributing

Feel free to fork the project and submit pull requests with improvements. Some ideas for enhancements:
- Add AI opponent
- Implement different difficulty levels
- Add power-ups
- Add sound effects
- Add a main menu
- Add pause functionality

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by classic Pong arcade game
- Built using Rust and crossterm library
