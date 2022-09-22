mod displays;

use displays::Display;
use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.lines().collect::<Vec<_>>();
    println!("{}", puzzle_one(&input));
    Ok(())
}

fn puzzle_one(input: &[&str]) -> i32 {
    let displays = input
        .iter()
        .map(|&line| Display::from_str(line))
        .collect::<Vec<_>>();
    let mut count = 0;
    for d in displays {
        count += d.count_unique_segments();
    }
    count
}
