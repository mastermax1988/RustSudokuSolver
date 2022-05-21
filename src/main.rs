mod board;
mod cell;

use board::Board;
use std::collections::VecDeque;
fn main() {
    let mut boards: VecDeque<Board> = VecDeque::new();
    let board = Board::new();
    board.print();
    boards.push_back(board.clone());

    solve(boards)
}

fn solve(mut boards: VecDeque<Board>) {
    loop {
        let b = match boards.pop_front() {
            Some(x) => x,
            None => {
                println!("unsolvable");
                return;
            }
        };
        b.print();
        println!("{:?}", b.get_empty_cell_with_least_possible_values());
        if b.is_solved() {
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
            //println!("({},{}) -> {}", best_cell.0, best_cell.1, values[i as usize]);
            boards.push_back(b_clone);
        }
    }
    //recursiv_solve(boards);
}
