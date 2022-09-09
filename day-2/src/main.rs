use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.split('\n').collect::<Vec<&str>>();
    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
    Ok(())
}

fn puzzle_one(input: &[&str]) -> i32 {
    let mut pos = 0;
    let mut depth = 0;

    input
        .iter()
        .for_each(|line| match line.chars().next().unwrap_or(' ') {
            'f' => pos += line.split(' ').last().unwrap().parse::<i32>().unwrap(),
            'u' => depth -= line.split(' ').last().unwrap().parse::<i32>().unwrap(),
            'd' => depth += line.split(' ').last().unwrap().parse::<i32>().unwrap(),
            _ => {}
        });

    pos * depth
}

fn puzzle_two(input: &[&str]) -> i32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    input
        .iter()
        .for_each(|line| match line.chars().next().unwrap_or(' ') {
            'f' => {
                let value = line.split(' ').last().unwrap().parse::<i32>().unwrap();
                pos += value;
                depth += value * aim;
            }
            'u' => aim -= line.split(' ').last().unwrap().parse::<i32>().unwrap(),
            'd' => aim += line.split(' ').last().unwrap().parse::<i32>().unwrap(),
            _ => {}
        });

    pos * depth
}
