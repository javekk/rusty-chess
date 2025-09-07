# Rusty Chess

A simple chess game implementation in Rust.

## Features

- ✅ Complete chess board with all pieces
- ✅ Drag and drop piece movement with mouse
- ✅ Basic move validation for all piece types
- ✅ Move history with undo/redo functionality
- ✅ Game restart capability
- ✅ Turn-based gameplay (White starts first)

## How to Play

### Controls

- **Mouse**: Click and drag pieces to move them
- **U Key**: Undo the last move
- **Ctrl + R**: Redo a move
- **Ctrl + N**: Start a new game

### Rules

- The game follows standard chess rules
- White always moves first
- You can only move pieces of the current player's color
- Invalid moves will be rejected with a message in the console

## Running the Game

```bash
cargo run
```

## Dependencies

- `macroquad`: Simple cross-platform graphics library for the UI

This implementation is designed to be simple and educational, perfect for learning Rust while building a functional chess game.
