mod cell;
mod board;
use board::Board;
fn main() {
    let mut boards:Vec<Board> = Vec::new();
    let board = Board::new();

    boards.push(board.clone());
    board.print();
    println!("{:?}",board.get_cell_with_least_possible_values());
}
