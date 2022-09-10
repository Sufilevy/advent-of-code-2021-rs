use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.split('\n').collect::<Vec<&str>>();
    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
    Ok(())
}

fn puzzle_one(input: &[&str]) -> i32 {
    let mut count = 0;
    input.iter().zip(input.iter().skip(1)).for_each(|(a, b)| {
        let (a, b) = (a.parse::<i32>(), b.parse::<i32>());
        if a.is_ok() && b.is_ok() && b.unwrap() > a.unwrap() {
            count += 1;
        }
    });
    count
}

fn puzzle_two(input: &[&str]) -> i32 {
    let mut count = 0;
    let mut last_sum = 9999;
    input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .for_each(|(ab, c)| {
            let (a, b, c) = (ab.0.parse::<i32>(), ab.1.parse::<i32>(), c.parse::<i32>());
            if a.is_err() || b.is_err() || c.is_err() {
                return;
            }
            let sum = a.unwrap() + b.unwrap() + c.unwrap();
            if sum > last_sum {
                count += 1
            }
            last_sum = sum;
        });
    count
}
