use crate::board::Board;
use regex::Regex;
use std::error::Error;

pub fn get_random_board() -> Board {
    let b = fetch().unwrap();
    b
}

fn fetch() -> Result<Board, Box<dyn Error>> {
    let res = reqwest::blocking::get("https://sugoku.herokuapp.com/board?difficulty=hard")?
        .text()
        .unwrap();
    let re = Regex::new(r"\d").unwrap();
    let matches: Vec<u8> = re
        .find_iter(res.as_str())
        .filter_map(|d| d.as_str().parse().ok())
        .collect();
    let b = Board::new_from_vec(matches);
    Ok(b)
}
