use std::{fs, io::Error};

use crate::bingo::Bingo;

mod bingo;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input
        .split("\n\n")
        .map(|e| e.to_owned())
        .collect::<Vec<String>>();
    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
    Ok(())
}

fn puzzle_one(input: &Vec<String>) -> i32 {
    let mut bingo = Bingo::from(input.to_owned());
    let board = bingo.get_first_board();
    board.calculate_score()
}

fn puzzle_two(input: &Vec<String>) -> i32 {
    let mut bingo = Bingo::from(input.to_owned());
    let board = bingo.get_last_board();
    board.calculate_score()
}
