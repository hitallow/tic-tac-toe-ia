use super::models::Move;

#[derive(Copy, Clone, Debug)]
pub struct Metric {
    win: i32,
    lose: i32,
    old: i32,
    pos: Move,
}

pub struct Statistic {
    pub win: f32,
    pub lose: f32,
    pub old: f32,
    pub pos: Move,
}
impl Metric {
    fn new(pos: Move) -> Metric {
        return Metric {
            win: 0,
            lose: 0,
            old: 0,
            pos,
        };
    }
}

pub fn get_possibilities(
    mut board: [[char; 3]; 3],
    player: char,
    oponnent: char,
) -> Vec<Statistic> {
    let mut pos: Vec<Statistic> = vec![];
    for row in 0..3 {
        for column in 0..3 {
            // verify is possible
            if board[row][column] == '-' {
                board[row][column] = player;
                let mut m = Metric::new(Move::new(row as i8, column as i8));
                let metric = verify(board, &mut m, true, player, oponnent);
                let total = metric.old + metric.lose + metric.win;

                let old = metric.old as f32 / total as f32;
                let win = metric.win as f32 / total as f32;
                let lose = metric.lose as f32 / total as f32;
                pos.push(Statistic {
                    lose,
                    old,
                    win,
                    pos: Move::new(row as i8, column as i8),
                });
                board[row][column] = '-';
            }
        }
    }

    return pos;
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
enum Result {
    Win,
    Lose,
    Tie,
    NoOne,
}

fn evaluate(board: &[[char; 3]; 3], player: char, oponnent: char) -> Result {
    let decider = |value: char| -> Result {
        if value == player {
            return Result::Win;
        }
        if value == oponnent {
            return Result::Lose;
        }

        return Result::Tie;
    };

    // verify each row
    for row in 0..3 {
        if board[row][0] == board[row][1] && board[row][1] == board[row][2] && board[row][2] != '-'
        {
            return decider(board[row][0]);
        }
    }
    // verify columns
    for column in 0..3 {
        if board[0][column] == board[1][column]
            && board[1][column] == board[2][column]
            && board[2][column] != '-'
        {
            return decider(board[0][column]);
        }
    }
    // verify diagonal
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[2][2] != '-' {
        return decider(board[0][0]);
    }

    if !is_moves_left(&board) {
        return Result::Tie;
    }

    return Result::NoOne;
}

pub fn print_board(board: &[[char; 3]; 3]) {
    for row in 0..3 {
        for col in 0..3 {
            print!(" {}", board[row][col]);
            if col != 2 {
                print!(" |");
            }
        }
        println!("");
        if row != 2 {
            println!("------------");
        }
    }
    println!("");
}

fn verify(
    mut board: [[char; 3]; 3],
    metric: &mut Metric,
    is_player_turn: bool,
    player: char,
    oponnent: char,
) -> Metric {
    match evaluate(&board, player, oponnent) {
        Result::Win => {
            metric.win += 1;
            return *metric;
        }
        Result::Lose => {
            metric.lose += 1;
            return *metric;
        }
        Result::Tie => {
            metric.old = metric.old + 1;
            return *metric;
        }
        Result::NoOne => {
            for row in 0..3 {
                for col in 0..3 {
                    if board[row][col] == '-' {
                        if is_player_turn {
                            board[row][col] = 'X';
                        } else {
                            board[row][col] = 'O';
                        }
                        *metric = verify(board, metric, !is_player_turn, player, oponnent);
                        board[row][col] = '-';
                    }
                }
            }
        }
    }
    return *metric;
}
