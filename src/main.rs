mod board;
mod cell;
mod downloader;

use board::Board;
use std::collections::VecDeque;

fn main() {
    let board = downloader::get_random_board();
    board.print();

    solve(board)
}

fn solve(mut board: Board) {
    let mut boards: VecDeque<Board> = VecDeque::new();
    board.autofill_cells();
    board.print();
    boards.push_back(board);
    loop {
        let b = match boards.pop_front() {
            Some(x) => x,
            None => {
                println!("unsolvable");
                return;
            }
        };

        if b.is_solved() {
            println!("solved:");
            b.print();
            return;
        }
        if !b.is_solvable() {
            println!("leave recursion");
            continue;
        }

        let best_cell = b.get_empty_cell_with_least_possible_values();
        let values = b.get_cells()[best_cell.0][best_cell.1].get_all_possible_values();

        for i in 0..best_cell.2 {
            let mut b_clone = b.clone();
            b_clone.set_cell_value(best_cell.0, best_cell.1, values[i as usize]);
            boards.push_back(b_clone);
        }
    }
    //recursiv_solve(boards);
}
