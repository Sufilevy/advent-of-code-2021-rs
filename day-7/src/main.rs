use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
    Ok(())
}

fn puzzle_one(input: &[i32]) -> i32 {
    let mut least_fuel = 999999;
    let (&min, &max) = (input.iter().min().unwrap(), input.iter().max().unwrap());
    for i in min..=max {
        let fuel = input.iter().fold(0, |acc, &n| acc + (n - i).abs());
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }
    least_fuel
}

fn puzzle_two(input: &[i32]) -> i32 {
    let mut least_fuel = f64::INFINITY;
    let (&min, &max) = (input.iter().min().unwrap(), input.iter().max().unwrap());
    for i in min..=max {
        let fuel = input.iter().fold(0., |acc, &n| {
            let diff = (n - i).abs() as f64;
            acc + 0.5 * diff * diff + 0.5 * diff
        });
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }
    least_fuel as i32
}
