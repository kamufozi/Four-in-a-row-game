# Puissance - Four in a Row (Connect 4) in Rust

A terminal-based **Four in a Row** (Connect 4) game written in pure Rust, featuring an **unbeatable AI opponent** powered by the Minimax algorithm.

```
========WELCOME TO THE FOUR IN A ROW GAME===========
========You are 🔴 ======= The AI is 🟡============

| ⚫ | ⚫ | ⚫ | ⚫ | ⚫ | ⚫ | ⚫ |
| ⚫ | ⚫ | ⚫ | ⚫ | ⚫ | ⚫ | ⚫ |
| ⚫ | ⚫ | ⚫ | ⚫ | ⚫ | ⚫ | ⚫ |
| ⚫ | ⚫ | ⚫ | 🟡 | ⚫ | ⚫ | ⚫ |
| ⚫ | ⚫ | 🟡 | 🔴 | ⚫ | ⚫ | ⚫ |
| ⚫ | ⚫ | 🔴 | 🔴 | 🟡 | ⚫ | ⚫ |
├────┼────┼────┼────┼────┼────┼────┤
│  1 │  2 │  3 │  4 │  5 │  6 │  7 │
```

## Features

- **Human vs AI** - Play against an AI that uses the Minimax algorithm to evaluate every possible move
- **Terminal UI** - Clean board display with emoji pieces (🔴 🟡 ⚫) and column numbers
- **Win detection** - Checks all four directions: horizontal, vertical, and both diagonals
- **Input validation** - Handles invalid inputs gracefully and prompts the player to try again
- **No external dependencies** - Built entirely with Rust's standard library

## How It Works

### Data Structures

| Structure | Purpose |
|-----------|---------|
| `[[State; COLS]; ROWS]` | 6x7 fixed-size 2D array representing the game board |
| `Vec<usize>` | Dynamic vector used to collect valid columns the AI can play |
| `State` enum | Represents each cell as `Red` (human), `Yellow` (AI), or `Blank` |

### AI - The Minimax Algorithm

The AI uses the **Minimax** algorithm with a configurable search depth (default: 3) to look ahead and evaluate future board states. It works by:

1. **Maximizing** the AI's score when it's the AI's turn (Yellow)
2. **Minimizing** the AI's score when simulating the human's turn (Red)
3. **Evaluating** board positions using a heuristic scoring function that considers:
   - Four-in-a-row (instant win/loss)
   - Three pieces with one blank (strong threat)
   - Two pieces with two blanks (developing position)
   - Center column control (strategic advantage)

The AI is extremely difficult to beat at the default depth - good luck!

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)

### Build and Run

```bash
# Clone the repository
git clone https://github.com/kamufozi/Four-in-a-row-game.git
cd Four-in-a-row-game

# Build and run
cargo run
```

### How to Play

1. You are **Player 1** (🔴) and always go first
2. Enter a column number (1-7) to drop your piece
3. The **AI** (🟡) responds automatically
4. First to connect **four in a row** (horizontally, vertically, or diagonally) wins!

## Project Structure

```
puissance/
├── Cargo.toml       # Project manifest
└── src/
    └── main.rs      # All game logic in a single file
```

### Key Functions

| Function | Description |
|----------|-------------|
| `main()` | Game loop - alternates between human and AI turns |
| `minimax()` | Recursive Minimax AI that returns the best column and score |
| `score_position()` | Heuristic evaluation of a board state |
| `evaluate_window()` | Scores a window of 4 cells based on piece counts |
| `is_winning()` | Checks all four directions for a four-in-a-row |
| `is_col_available()` | Checks if a column has space for another piece |
| `place_in_available_row()` | Finds the lowest empty row in a column |
| `drop_inside_board()` | Places a piece on the board |
| `get_all_valid_cols()` | Returns all columns that aren't full |
| `ui()` | Renders the board to the terminal |

## References

- [Connect 4 AI - Python Tutorial Playlist](https://www.youtube.com/playlist?list=PLFCB5Dp81iNV_inzM-R9AKkZZlePCZdtV) - The tutorial series that inspired the Minimax AI logic in this project

## License

This project is shared as a learning resource. Feel free to study, reference, and learn from it.
