mod vents;

use std::{fs, io::Error};
use vents::*;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();
    let lines = Line::parse_lines(&input);
    let table = Table::fill_from(lines);
    println!("{}", table.count_dangerous_areas());
    Ok(())
}
