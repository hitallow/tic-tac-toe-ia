mod game;

fn main() {
    let board: [[char; 3]; 3] = [
        ['-', 'X', 'O'],
        ['O', 'X', '-'],
        ['-', '-', '-'], //
    ];
    println!(
        "result {:?}",
        game::functions::find_best_move(board, 'O', 'X')
    );
}
