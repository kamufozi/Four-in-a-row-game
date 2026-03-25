use std::io;

// Board dimensions: standard Connect 4 is 6 rows by 7 columns.
const ROWS: usize = 6;
const COLS: usize = 7;

// Bounds for Minimax scoring. These represent the best and worst possible
// outcomes so that any real heuristic score falls between them.
const BIGGEST_VALUE: i32 = 2147483647;
const SMALLEST_VALUE: i32 = -2147483647;

/// Cell state: each position on the board is either occupied by a player's
/// piece or blank. Red = human, Yellow = AI.
#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Red,
    Yellow,
    Blank,
}

fn main() {
    // The board is a fixed-size 2D array. Row 0 is the bottom; row 5 is the top.
    let mut board: [[State; COLS]; ROWS] = [[State::Blank; COLS]; ROWS];

    // Turn counter: odd = human (Red), even = AI (Yellow).
    let mut count: i8 = 1;
    let mut game_over = true;

    while game_over {
        let mut is_playing: State = State::Blank;
        clear_screen();

        // --- Human turn (Player 1 - Red) ---
        if count % 2 != 0 {
            is_playing = State::Red;
            print!("Player 1 ");
            println!("Enter a column (1-7):");
            let mut col = String::new();
            ui(&board);

            // Read and validate column input. On invalid input, restart the loop
            // so the player can try again without losing their turn.
            io::stdin()
                .read_line(&mut col)
                .expect("An error happened while trying to enter the number");
            let col: usize = match col.trim().parse::<usize>() {
                Ok(n) if (1..=COLS).contains(&n) => n - 1, // Convert 1-indexed to 0-indexed
                _ => {
                    println!("Invalid input: {}", col.trim());
                    println!("Please enter a number between 1 and 7.");
                    continue;
                }
            };

            // Drop the piece if the column isn't full.
            if is_col_available(&board, &col) {
                let row = place_in_available_row(&board, &col);
                drop_inside_board(&mut board, &col, &row, &is_playing);
                ui(&board);
                clear_screen();
            }

            if is_winning(&board, &is_playing) {
                ui(&board);
                println!("You win. Hooray!!");
                game_over = false;
            }
        }
        // --- AI turn (Player 2 - Yellow) ---
        else {
            is_playing = State::Yellow;
            println!("Robot:");

            // The AI picks the best column using Minimax with a search depth of 3.
            let col = (minimax(&board, 3, true).0).unwrap();

            if is_col_available(&board, &col) {
                let row = place_in_available_row(&board, &col);
                drop_inside_board(&mut board, &col, &row, &is_playing);
                ui(&board);
                clear_screen();
            }

            if is_winning(&board, &is_playing) {
                ui(&board);
                println!("Mission failed, you'll get 'em next time!");
                game_over = false;
            }
        }

        count += 1;
    }
}

/// Checks whether a column still has room for another piece.
/// A column is available if the top cell (highest row) is still blank.
fn is_col_available(board: &[[State; COLS]; ROWS], col: &usize) -> bool {
    board[ROWS - 1][*col] == State::Blank
}

/// Finds the lowest empty row in a given column.
/// Pieces fall to the bottom due to gravity, so we scan upward from row 0.
fn place_in_available_row(board: &[[State; COLS]; ROWS], col: &usize) -> usize {
    for row in 0..ROWS {
        if board[row][*col] == State::Blank {
            return row;
        }
    }
    panic!("No available row")
}

/// Places a piece on the board at the specified row and column.
fn drop_inside_board(
    board: &mut [[State; COLS]; ROWS],
    col: &usize,
    row: &usize,
    player: &State,
) {
    board[*row][*col] = *player;
}

/// Renders the game board to the terminal with emoji pieces and column labels.
fn ui(board: &[[State; COLS]; ROWS]) {
    println!("========WELCOME TO THE FOUR IN A ROW GAME===========");
    println!("========You are 🔴 ======= The AI is 🟡============");
    println!();

    // Print rows from top to bottom (board is stored bottom-up).
    for row in board.iter().rev() {
        print!("|");
        for column in row {
            let symbol = match column {
                State::Blank => "⚫",
                State::Red => "🔴",
                State::Yellow => "🟡",
            };
            print!(" {} |", symbol);
        }
        println!();
    }

    // Column separator and labels.
    println!("├────┼────┼────┼────┼────┼────┼────┤");
    print!("│");
    for c in 1..=7 {
        print!("  {} │", c);
    }
    println!();
    println!();
}

/// Clears the terminal screen using ANSI escape codes.
fn clear_screen() {
    println!("\x1B[2J\x1B[H");
}

/// Checks whether the given player has four pieces in a row.
/// Scans all four possible directions: horizontal, vertical,
/// positive diagonal (bottom-left to top-right), and negative diagonal
/// (top-left to bottom-right).
fn is_winning(board: &[[State; COLS]; ROWS], player: &State) -> bool {
    // Horizontal: check every row for 4 consecutive pieces.
    for row in 0..ROWS {
        for col in 0..COLS - 3 {
            if board[row][col] == *player
                && board[row][col + 1] == *player
                && board[row][col + 2] == *player
                && board[row][col + 3] == *player
            {
                return true;
            }
        }
    }

    // Vertical: check every column for 4 consecutive pieces stacked upward.
    for row in 0..ROWS - 3 {
        for col in 0..COLS {
            if board[row][col] == *player
                && board[row + 1][col] == *player
                && board[row + 2][col] == *player
                && board[row + 3][col] == *player
            {
                return true;
            }
        }
    }

    // Positive diagonal (↗): bottom-left to top-right.
    for row in 0..ROWS - 3 {
        for col in 0..COLS - 3 {
            if board[row][col] == *player
                && board[row + 1][col + 1] == *player
                && board[row + 2][col + 2] == *player
                && board[row + 3][col + 3] == *player
            {
                return true;
            }
        }
    }

    // Negative diagonal (↘): top-left to bottom-right.
    for row in 3..ROWS {
        for col in 0..COLS - 3 {
            if board[row][col] == *player
                && board[row - 1][col + 1] == *player
                && board[row - 2][col + 2] == *player
                && board[row - 3][col + 3] == *player
            {
                return true;
            }
        }
    }

    false
}

/// Scores a window (slice of 4 cells) based on how favorable it is for the
/// given player. Higher scores = better for the player.
///
/// Scoring weights:
///   +100  four of player's pieces (win)
///   +10   three pieces + one blank (one move from winning)
///   +5    two pieces + two blanks (developing position)
///   -80   three opponent pieces + one blank (must block)
fn evaluate_window(window: &[State], player: &State) -> i32 {
    let mut score = 0;

    // Determine the opponent's color.
    let opp = if *player == State::Red {
        State::Yellow
    } else {
        State::Red
    };

    let player_count = window.iter().filter(|x| **x == *player).count();
    let blank_count = window.iter().filter(|x| **x == State::Blank).count();
    let opp_count = window.iter().filter(|x| **x == opp).count();

    if player_count == 4 {
        score += 100;
    } else if player_count == 3 && blank_count == 1 {
        score += 10;
    } else if player_count == 2 && blank_count == 2 {
        score += 5;
    }

    // Penalize positions where the opponent is one move away from winning.
    if opp_count == 3 && blank_count == 1 {
        score -= 80;
    }

    score
}

/// Evaluates the entire board and returns a heuristic score for the given player.
/// Scans all possible windows of 4 in every direction, plus awards a bonus
/// for controlling the center column.
fn score_position(board: &[[State; COLS]; ROWS], player: &State) -> i32 {
    let mut score = 0;

    // Center column bonus: controlling the center gives more connect-4 opportunities.
    let mut center_list = Vec::with_capacity(COLS);
    for row in 0..ROWS {
        center_list.push(board[row][3]);
    }
    score += (center_list.iter().filter(|x| **x == *player).count() * 6) as i32;

    // Score horizontal windows.
    for row in 0..ROWS {
        let row_items: [State; 7] = board[row];
        for col in 0..COLS - 3 {
            let window: &[State] = &row_items[col..col + 4];
            score += evaluate_window(window, player);
        }
    }

    // Score vertical windows.
    for col in 0..COLS {
        for row in 0..ROWS - 3 {
            let window = [
                board[row][col],
                board[row + 1][col],
                board[row + 2][col],
                board[row + 3][col],
            ];
            score += evaluate_window(&window, player);
        }
    }

    // Score positive diagonal (↗) windows.
    for row in 0..ROWS - 3 {
        for col in 0..COLS - 3 {
            let window = [
                board[row][col],
                board[row + 1][col + 1],
                board[row + 2][col + 2],
                board[row + 3][col + 3],
            ];
            score += evaluate_window(&window, player);
        }
    }

    // Score negative diagonal (↘) windows.
    for row in 0..ROWS - 3 {
        for col in 0..COLS - 3 {
            let window = [
                board[row + 3][col],
                board[row + 2][col + 1],
                board[row + 1][col + 2],
                board[row][col + 3],
            ];
            score += evaluate_window(&window, player);
        }
    }

    score
}

/// Returns a vector of all column indices that still have space for a piece.
fn get_all_valid_cols(board: &[[State; COLS]; ROWS]) -> Vec<usize> {
    let mut valid_cols: Vec<usize> = Vec::with_capacity(COLS);
    for col in 0..COLS {
        if is_col_available(board, &col) {
            valid_cols.push(col);
        }
    }
    valid_cols
}

/// Checks whether the game has reached a terminal state: either player has won,
/// or the board is completely full (draw).
fn is_terminal_node(board: &[[State; COLS]; ROWS]) -> bool {
    is_winning(board, &State::Red)
        || is_winning(board, &State::Yellow)
        || get_all_valid_cols(board).is_empty()
}

/// Minimax algorithm: recursively explores future board states to find the
/// optimal column for the AI to play.
///
/// - `depth`: how many moves ahead to search (higher = stronger but slower)
/// - `maximising_player`: true when it's the AI's turn (Yellow), false for human (Red)
///
/// Returns a tuple of (best_column, best_score). At terminal/leaf nodes the
/// column is None since no move is being made.
fn minimax(
    board: &[[State; COLS]; ROWS],
    depth: i32,
    maximising_player: bool,
) -> (Option<usize>, i32) {
    let all_valid_cols = get_all_valid_cols(board);
    let mut column = all_valid_cols[0];

    // Base case: stop recursing at depth 0 or if the game is over.
    if depth == 0 || is_terminal_node(board) {
        if is_terminal_node(board) {
            if is_winning(board, &State::Yellow) {
                return (None, BIGGEST_VALUE); // AI wins - best outcome
            } else if is_winning(board, &State::Red) {
                return (None, SMALLEST_VALUE); // Human wins - worst outcome
            } else {
                return (None, 0); // Draw
            }
        } else {
            // Depth exhausted: evaluate the board heuristically.
            return (None, score_position(board, &State::Yellow));
        }
    }

    // Maximizing: AI (Yellow) wants the highest score.
    if maximising_player {
        let mut value = SMALLEST_VALUE;
        for col in all_valid_cols {
            let row = place_in_available_row(board, &col);
            let mut board_copy = board.clone();
            drop_inside_board(&mut board_copy, &col, &row, &State::Yellow);
            let new_score = minimax(&board_copy, depth - 1, false).1;
            if new_score > value {
                value = new_score;
                column = col;
            }
        }
        (Some(column), value)
    }
    // Minimizing: Human (Red) wants the lowest score.
    else {
        let mut value = BIGGEST_VALUE;
        for col in all_valid_cols {
            let row = place_in_available_row(board, &col);
            let mut board_copy = board.clone();
            drop_inside_board(&mut board_copy, &col, &row, &State::Red);
            let new_score = minimax(&board_copy, depth - 1, true).1;
            if new_score < value {
                value = new_score;
                column = col;
            }
        }
        (Some(column), value)
    }
}
