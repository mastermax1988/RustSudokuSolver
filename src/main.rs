mod board;
mod cell;
mod downloader;

use board::Board;
use std::collections::VecDeque;
use std::time::SystemTime;

fn main() {
    let board = downloader::get_random_board();
    //let board = Board::new();
    solve(board)
}

fn solve(mut board: Board) {
    let now = SystemTime::now();
    board.print();
    let mut boards: VecDeque<Board> = VecDeque::new();

    board.autofill_cells();
    board.set_fix_cells();
    println!("autofill all certain cells");
    board.print();
    boards.push_back(board);
    let mut nrofsolutions = 0;

    loop {
        let b = match boards.pop_front() {
            Some(x) => x,
            None => {
                println!("{} solutions found", nrofsolutions);
                return;
            }
        };

        if b.is_solved() {
            nrofsolutions += 1;
            println!(
                "{}. solution {}ms:",
                nrofsolutions,
                now.elapsed().unwrap().as_millis()
            );
            b.print();
            continue;
            //return;
        }
        if !b.is_solvable() {
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
