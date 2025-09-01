# Rusty Chess

A simple chess game implementation in Rust using clean architecture principles.

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

## Project Structure

The project follows clean architecture principles:

```
src/
├── domain/          # Core business logic
│   ├── piece.rs     # Piece types and colors
│   ├── position.rs  # Board positions
│   ├── board.rs     # Chess board state
│   ├── game.rs      # Game logic and rules
│   └── move_record.rs # Move history
└── presentation/    # UI layer
    └── chess_ui.rs  # Graphics and user interaction
```

## Architecture

- **Domain Layer**: Contains pure business logic with no dependencies on UI or external libraries
- **Presentation Layer**: Handles user interface using macroquad for graphics
- **Clean Separation**: Domain models are completely independent of the UI implementation

## Dependencies

- `macroquad`: Simple cross-platform graphics library for the UI

This implementation is designed to be simple and educational, perfect for learning Rust while building a functional chess game.