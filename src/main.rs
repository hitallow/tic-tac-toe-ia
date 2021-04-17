use game::possibilities::print_board;

mod game;

fn main() {
    let board: [[char; 3]; 3] = [
        ['O', '-', '-'],
        ['-', 'X', '-'],
        ['-', '-', '-'], //
    ];
    print_board(&board);

    println!(
        "Melhor passo {:?} para X",
        game::functions::find_best_move(board, 'X', 'O')
    );

    let possibilities = game::possibilities::get_possibilities(board, 'X', 'O');

    for metric in possibilities.iter() {
        println!(
            "Vitória {:.2}%, Derrota {:.2}%, Empate {:.2}% em {}X{}",
            metric.win * 100.0,
            metric.lose * 100.0,
            metric.old * 100.0,
            metric.pos.get_row() + 1,
            metric.pos.get_column() + 1
        );
    }
}
