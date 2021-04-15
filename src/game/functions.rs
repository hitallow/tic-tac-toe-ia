use std::cmp::{max, min};

use super::models::Move;

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

// Verifica se há um vencedor no board
// caso o player vença, retorn 10
// caso o oponnent vença retonar -10
// caso não haja vencedores retorna 0
fn evaluate(board: &[[char; 3]; 3], player: char, oponnent: char) -> i32 {
    // check rows

    let decider = |value: char| -> i32 {
        if value == player {
            return 10;
        }
        if value == oponnent {
            return -10;
        }

        return 0;
    };

    // verify each row
    for row in 0..3 {
        if board[row][0] == board[row][1] && board[row][1] == board[row][2] {
            return decider(board[row][0]);
        }
    }
    // verify columns
    for column in 0..3 {
        if board[0][column] == board[1][column] && board[1][column] == board[2][column] {
            return decider(board[0][column]);
        }
    }
    // verify diagonal
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return decider(board[0][0]);
    }

    return 0;
}

fn min_max(
    mut board: [[char; 3]; 3],
    depth: i32,
    is_player_turn: bool,
    player: char,
    oponnent: char,
) -> i32 {
    let score = evaluate(&board, player, oponnent);

    if score == 10 {
        return score - depth;
    }

    if score == -10 {
        return score + depth;
    }

    if !is_moves_left(&board) {
        return 0;
    }

    if is_player_turn {
        let mut best_move = -100;
        for row in 0..3 {
            for col in 0..3 {
                if board[row][col] == '-' {
                    board[row][col] = player;
                    best_move = max(
                        best_move,
                        min_max(board, depth + 1, !is_player_turn, player, oponnent),
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
                    board[row][col] = oponnent;
                    best_move = min(
                        best_move,
                        min_max(board, depth + 1, !is_player_turn, player, oponnent),
                    );
                    board[row][col] = '-';
                }
            }
        }
        return best_move;
    }
}

pub fn find_best_move(
    mut board: [[char; 3]; 3],
    current_player: char,
    current_oponnent: char,
) -> Move {
    let mut best_move = Move::new(-1, -1);

    let mut best_op = -100;
    for row in 0..3 {
        for column in 0..3 {
            if board[row][column] == '-' {
                board[row][column] = current_player;
                let current_move = min_max(board, 0, true, current_player, current_oponnent);
                board[row][column] = '-';

                if current_move > best_op {
                    best_move.set_row(row as i8 + 1);
                    best_move.set_column(column as i8 + 1);
                    best_op = current_move;
                }
            }
        }
    }
    return best_move;
}
