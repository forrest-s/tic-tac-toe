use std::io;

    fn main() {
        let mut board = [[' '; 3]; 3]; // Initialize an empty board
        let mut turn = 'X';

        loop {
            println!("{} | {} | {}", board[0][0], board[0][1], board[0][2]);
            println!("---------");
            println!("{} | {} | {}", board[1][0], board[1][1], board[1][2]);
            println!("---------");
            println!("{} | {} | {}", board[2][0], board[2][1], board[2][2]);

            let mut input = String::new();
            println!("Player {} turn. Enter row and column separated by a space, center being 1 1:", turn);
            io::stdin().read_line(&mut input).unwrap();
            let mut inputs = input.split_whitespace();
            let row: usize = inputs.next().unwrap().parse().unwrap();
            let col: usize = inputs.next().unwrap().parse().unwrap();

            if row >= board.len() || col >= board[0].len() {
                println!("Invalid cell. Try again.");
                continue;
            }
            

            if board[row][col] != ' ' {
                if is_game_tied(board) {
                    let mut exit_game = false;
                    loop {
                        println!("The game is tied. Would you like to try again?");
                        println!("1. Exit");
                        println!("2. Restart");
                    
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                    
                        match input.trim() {
                            "1" => {
                                println!("Goodbye!");
                                exit_game = true;
                                break;
                            },
                            "2" => {
                                println!("Restarting, good luck!");
                                // Restart the game here
                                board = [[' '; 3]; 3]; 
                                break;
                            },
                            _ => {
                                println!("Invalid option, please try again.");
                                continue;
                            },
                        }
                    }
                    
                    if exit_game {
                        break;
                    }
                    continue;
                }

                println!("Cell is already occupied. Try again.");
                continue;
            }

            board[row][col] = turn;

            if is_game_over(board) {
                println!("{} wins!", turn);
                break;
            }

            if turn == 'X' {
                turn = 'O';
            } else {
                turn = 'X';
            }
        }
    }

fn is_game_over(board: [[char; 3]; 3]) -> bool {
    // Check rows
    for i in 0..3 {
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != ' ' {
            return true;
        }
    }

    // Check columns
    for i in 0..3 {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != ' ' {
            return true;
        }
    }

    // Check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != ' ' {
        return true;
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != ' ' {
        return true;
    }

    false
}

fn is_game_tied(board: [[char; 3]; 3]) -> bool {
    let mut spaces = 0;

    // Count blank spaces
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == ' ' {
                spaces += 1;
            }
        }
    }

    // Return true if no more blank spaces
    spaces == 0
}

