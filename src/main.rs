use std::{io};

const ROWS:usize=6;
const COLS:usize=7;

const BIGGEST_VALUE:i32 = 2147483647;
const SMALLEST_VALUE:i32 = -2147483647;

fn main() {
    let mut board:[[State;COLS];ROWS] = [[State::Blank;COLS];ROWS];
    // isplaying is a variable that chooses the player that is playing.
    //let's do a count variable, that sees who is playing. if odd then human , if even then robot. 
    let mut count:i8=1;
    let mut game_over=true;
    while game_over {
        let mut is_playing: State=State::Blank;
        clear_screen();
        //player 1
        if count%2!=0{
            is_playing = State::Red;
            print!("Player 1 ");
            //should add a loop for when the user enters a wrong thing by accident and he should be told to try again
            println!("Enter a column (1-7):");
            let mut col = String::new();
            ui(&board);
            io::stdin().read_line(&mut col).expect("An error happened while trying to enter the number");
            let col:usize = match col.trim().parse::<usize>(){
                Ok(n) if (1..=COLS).contains(&n)=>n-1,
                _ => {
                    println!("this is an invalid number :{}",col);
                    println!("Please enter a number between 1 and 7.");
                    continue;
                }
            };
            if is_col_available(&board, &col){
                let row = place_in_available_row(&board, &col);
                drop_inside_board(&mut board, &col, &row, &is_playing);
                ui(&board);
                clear_screen();
            }
            if is_winning(&board, &is_playing){
                ui(&board);
                println!("You win. Hooray!!");
                game_over=false;
            }
        }

        //player 2
        else {  
            is_playing = State::Yellow;
            println!("Robot:");
            // let col = pick_best_move(&board, &is_playing);
            let col = (minimax(&board, 3, true).0).unwrap();
            if is_col_available(&board, &col){
                let row = place_in_available_row(&board, &col);
                drop_inside_board(&mut board, &col, &row, &is_playing);
                ui(&board);
                clear_screen();
            }
            if is_winning(&board, &is_playing){
                ui(&board);
                println!("Mission failed, you'll get 'em next time!");
                game_over=false;
            }
        }
        count+=1;
    }
}
//I do not want to ask the user for a red or a yellow
//he should just fill.
//a loop is needed then till I do ctrl+c 

//now I should update the table how though
//should insert on the lowest row and the available column. 
fn is_col_available(board:&[[State;COLS];ROWS],col:&usize)->bool{
    return board[ROWS-1][*col] == State::Blank;
}

//if a specifc column is available we should check the row also and see if it is available. 
fn place_in_available_row(board:&[[State;COLS];ROWS],col:&usize)->usize{
    for row in 0..ROWS{
        if board[row][*col] == State::Blank{
            return row
        }
    }
    panic!("No available row")
}

//Place the disc in the right spot
//It can either be yellow, red or blank
fn drop_inside_board(board:&mut [[State;COLS];ROWS],col:&usize,row:&usize,player:&State){
    board[*row][*col] = *player
}

fn ui(board:&[[State;COLS];ROWS]){
    println!("========WELCOME TO THE FOUR IN A ROW GAME===========");
    println!("========You are 🔴 ======= The AI is 🟡============");
    println!();
    for row in board.iter().rev(){
        print!("|");
        for column in row{
            let symbol =match column{
                State::Blank=>"⚫",
                State::Red=>"🔴",
                State::Yellow=>"🟡",
            };
            print!(" {} |",symbol);
        }
        println!();
    }
    println!("├────┼────┼────┼────┼────┼────┼────┤");
    print!("│");
    for c in 1..=7 {
        print!("  {} │", c);
    }
    println!();
    println!();
}

fn clear_screen(){
    println!("\x1B[2J\x1B[H");
}

fn is_winning(board:&[[State;COLS];ROWS],player:&State)->bool{
    //Horizontal Check
    for row in 0..ROWS {
        for col in 0..COLS-3 {
            if board[row][col]==*player && board[row][col+1]==*player && board[row][col+2]==*player && board[row][col+3]==*player{
                return true
            }
        }
    }

    //Vertical Check
    for row in 0..ROWS-3 {
        for col in 0..COLS {
            if board[row][col]==*player && board[row+1][col]==*player && board[row+2][col]==*player && board[row+3][col]==*player{
                return true
            }
        }
    }

    //Positive Diagonal Check
    for row in 0..ROWS-3 {
        for col in 0..COLS-3 {
            if board[row][col]==*player && board[row+1][col+1]==*player && board[row+2][col+2]==*player && board[row+3][col+3]==*player{
                return true
            }
        }
    } 

    //Negative Diagonal Check
    for row in 3..ROWS {
        for col in 0..COLS-3 {
            if board[row][col]==*player && board[row-1][col+1]==*player && board[row-2][col+2]==*player && board[row-3][col+3]==*player{
                return true
            }
        }
    }    
    return false;
}
fn evaluate_window(window: &[State],player:&State)->i32{
    let mut score = 0;
    let mut opp = State::Red;
    if player==&State::Red{
        opp = State::Yellow;
    }
    if window.iter().filter(|x| **x==*player).count()==4{
        score+=100
    }
    else if window.iter().filter(|x| **x==*player).count()==3 && window.iter().filter(|x| **x==State::Blank).count()==1 {
        score+=10
    }
    else if window.iter().filter(|x| **x==*player).count()==2 && window.iter().filter(|x| **x==State::Blank).count()==2 {
        score+=5
    }

    if window.iter().filter(|x| **x==opp).count()==3 && window.iter().filter(|x|**x==State::Blank).count()==1{
        score -=80
    }

    return score;
}

fn score_position(board:&[[State;COLS];ROWS],player:&State)->i32{
    let mut score = 0;

    //bring the points up if they started in the middle
    let mut center_list =Vec::with_capacity(COLS);
    for row in 0..ROWS  {
        let center_item = board[row][3];
        center_list.push(center_item);
    }
    score += (center_list.iter().filter(|x| **x ==*player).count() * 6) as i32;
    //Scoring Horizontal
    for row in 0..ROWS {
        let row_items: [State; 7] = board[row];
        for col in 0..COLS-3 {
            let window: &[State] = &row_items[col..col+4];
            //this function is meant to count the number of reds and yellows in a row.
            score+=evaluate_window(window, player);
        }
    }

    // Scoring Vertical
    for col in 0..COLS{
        for row in 0..ROWS-3{
            let window =[
                board[row][col],
                board[row+1][col],
                board[row+2][col],
                board[row+3][col]
            ];
            score+=evaluate_window(&window, player);
        }
    }

    //Scoring Diagonal
    for row in 0..ROWS-3 {
        for col in 0..COLS-3{
            //this is a loop to go over the window length
           let window =[
                board[row][col],
                board[row+1][col+1],
                board[row+2][col+2],
                board[row+3][col+3]
            ];
            score+=evaluate_window(&window, player);
        } 
    }

    //Scoring in the negative Diagonals
    for row in 0..ROWS-3 {
        for col in 0..COLS-3{
            let window = [
                board[row+3][col],
                board[row+3-1][col+1],
                board[row+3-2][col+2],
                board[row+3-3][col+3]
            ];
            score+=evaluate_window(&window, player);
        }
    }
    return score
}

//Getting all valid columns that AI can drop a piece into
fn get_all_valid_cols(board:&[[State;COLS];ROWS])->Vec<usize>{
    let mut valid_cols: Vec<usize> = Vec::with_capacity(COLS);
    for col in 0..COLS{
        if is_col_available(board, &col){
            valid_cols.push(col);
        }
    }
    return valid_cols;
}

//Second best algorithm 

// fn pick_best_move(board:&[[State;COLS];ROWS],player:&State)->usize{
//     let valid_locations = get_all_valid_cols(&board);
//     let mut best_score:i32 =-1000;
//     //generating a random column that will be our best column for now crying face emoji
//     let mut best_col=valid_locations[0];

//     for col in valid_locations {
//         let row = place_in_available_row(&board, &col);
//         let mut temp_board = board.clone();
//         drop_inside_board(&mut temp_board, &col, &row, player);
//         ui(&temp_board);
//         let score = score_position(&temp_board, player);
//         println!("The best score now is {score}");
//         if score>best_score{
//             best_score=score;
//             best_col=col;
//         }
//     }
//     return best_col;
// }

fn is_terminal_node(board:&[[State;COLS];ROWS])->bool{
    return is_winning(board, &State::Red) || is_winning(board, &State::Yellow) || get_all_valid_cols(board).len() ==0
}

fn minimax(board:&[[State;COLS];ROWS],depth:i32,maximising_player:bool )->(Option<usize>, i32){
    let all_valid_cols = get_all_valid_cols(board);
    let mut column = all_valid_cols[0];
    if depth == 0 || is_terminal_node(board){
        if is_terminal_node(board){
            if is_winning(board, &State::Yellow){
                //this number is the limit for i32.
                return (None,BIGGEST_VALUE);
            }
            else if is_winning(board, &State::Red){
                return (None,SMALLEST_VALUE);
            }
            else {
                return (None,0);
            }
        }
        else {
           return (None,score_position(board, &State::Yellow));
        }
    }

    else if maximising_player {
        let mut value = SMALLEST_VALUE;
        for col in all_valid_cols{
            let row=place_in_available_row(board, &col);
            let mut board_copy = board.clone();
            drop_inside_board(&mut board_copy, &col, &row, &State::Yellow);
            let new_score = minimax(& board_copy, depth-1, false).1;
            println!("{new_score}");
            if new_score > value {
                value = new_score;
                column = col;
            }
        }
        return (Some(column),value)
    }
    //This is the minimizing player
    else {
        let mut value = BIGGEST_VALUE;
        for col in all_valid_cols{
            let row =place_in_available_row(board, &col);
            let mut board_copy = board.clone();
            drop_inside_board(&mut board_copy, &col, &row, &State::Red);
            let new_score = minimax(&board_copy, depth-1, true).1;
            if new_score < value{
                value = new_score;
                column =col;
            }
        }
        return (Some(column),value);
    }

}


#[derive(Clone,Copy,PartialEq,Debug)]
enum State{
    Red,
    Yellow,
    Blank
}