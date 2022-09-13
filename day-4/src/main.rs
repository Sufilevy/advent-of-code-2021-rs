use std::{fs, io::Error};

use crate::bingo::Bingo;

mod bingo;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input
        .split("\n\n")
        .map(|e| e.to_owned())
        .collect::<Vec<String>>();
    let mut bingo = Bingo::from(input);
    let (first_board, last_board) = bingo.get_first_last_boards();
    println!("{}", first_board.calculate_score());
    println!("{}", last_board.calculate_score());
    Ok(())
}
