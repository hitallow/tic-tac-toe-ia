use std::cmp::{max, min};

#[derive(Debug)]
struct Move {
    row: i8,
    column: i8,
}

fn find_best_move(mut board: [[char; 3]; 3], player: char, oponnent: char) -> Option<Move> {
    let mut best_move = Move {
        row: -1,
        column: -1,
    };
    let mut best_op = -100;
    for row in 0..3 {
        for column in 0..3 {
            if board[row][column] == '-' {
                board[row][column] = player;
                let current_move = find(board, player, oponnent);
                board[row][column] = '-';

                if current_move > best_op {
                    best_move.row = row as i8;
                    best_move.column = column as i8;
                    best_op = current_move;
                }
            }
        }
    }
    return Some(best_move);
}

fn is_moves_left(board: &[[char; 3]; 3]) -> bool {
    for row in 0..3 {
        for column in 0..3 {
            if board[row][column] == '-' {
                return true;
            }
        }
    }
    return false;
}
fn find(board: [[char; 3]; 3], player: char, oponnent: char) -> i32 {
    return min_max(board, 0, false, player, oponnent);
}
fn min_max(
    mut board: [[char; 3]; 3],
    depth: i32,
    is_max: bool,
    player: char,
    oponnent: char,
) -> i32 {
    let score = evaluate(&board, 'X', 'O');

    if score == 10 || score == -10 {
        return score.clone();
    }

    if !is_moves_left(&board) {
        return 0;
    }

    if is_max {
        let mut best_move = -100;
        for row in 0..3 {
            for col in 0..3 {
                if board[row][col] == '-' {
                    board[row][col] = 'X';
                    best_move = max(
                        best_move,
                        min_max(board, depth + 1, !is_max, player, oponnent),
                    );
                    board[row][col] = '-';
                }
            }
        }
        return best_move;
    } else {
        let mut best_move = 100;
        for row in 0..3 {
            for col in 0..3 {
                if board[row][col] == '-' {
                    board[row][col] = 'O';
                    best_move = min(
                        best_move,
                        min_max(board, depth + 1, !is_max, player, oponnent),
                    );
                    board[row][col] = '-';
                }
            }
        }
        return best_move;
    }
}

fn evaluate(board: &[[char; 3]; 3], player: char, oponnent: char) -> i32 {
    // check rows
    for row in 0..3 {
        if board[row][0] == board[row][1] && board[row][1] == board[row][2] {
            if board[row][0] == player {
                return 10;
            } else if board[row][0] == oponnent {
                return -10;
            }
        }
    }
    // verify columns
    for column in 0..3 {
        if board[0][column] == board[1][column] && board[1][column] == board[2][column] {
            if board[0][column] == player {
                return 10;
            } else if board[0][column] == oponnent {
                return -10;
            }
        }
    }
    // verify diagonal
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        if board[0][0] == player {
            return 10;
        } else if board[0][0] == oponnent {
            return -10;
        }
    }

    return 0;
}

fn main() {
    let board: [[char; 3]; 3] = [['O', 'O', '-'], ['X', 'X', '-'], ['0', 'X', '-']];
    println!("result {:?}", find_best_move(board, 'X', 'O'));
}
