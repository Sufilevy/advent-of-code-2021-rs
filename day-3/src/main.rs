#![feature(drain_filter)]

use std::{cmp::Ordering, fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.split('\n').collect::<Vec<&str>>();
    println!("{}", puzzle_one(&input));
    print!("{}", puzzle_two(&input));
    Ok(())
}

fn puzzle_one(input: &[&str]) -> i32 {
    let mut epsilon = 0;
    let mut gamma = 0;

    for i in 0..12 {
        let mut appearances = (0, 0);
        input
            .iter()
            .for_each(|line| match line.chars().nth(11 - i).unwrap_or(' ') {
                '1' => appearances.1 += 1,
                '0' => appearances.0 += 1,
                _ => {}
            });
        if appearances.1 > appearances.0 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    epsilon * gamma
}

fn puzzle_two(input: &[&str]) -> i32 {
    let mut ogr = input.to_owned().clone(); // Oxygen generator rating
    let mut co2sr = input.to_owned().clone(); // CO2 scrubber rating

    for i in 0..12 {
        let mut appearances = (0, 0);
        input
            .iter()
            .for_each(|line| match line.chars().nth(11 - i).unwrap_or(' ') {
                '1' => appearances.1 += 1,
                '0' => appearances.0 += 1,
                _ => {}
            });

        let (ogr_filter, co2sr_filter) = match appearances.0.cmp(&appearances.1) {
            Ordering::Less => ('1', '0'),
            Ordering::Equal => ('1', '0'),
            Ordering::Greater => ('0', '1'),
        };

        if ogr.len() > 1 {
            ogr = ogr
                .iter()
                .filter(|num| num.to_owned().chars().nth(11 - i).unwrap_or(' ') == ogr_filter)
                .map(|e| e.to_owned())
                .collect();
        }
        if co2sr.len() > 1 {
            co2sr = co2sr
                .iter()
                .filter(|num| num.to_owned().chars().nth(11 - i).unwrap_or(' ') == co2sr_filter)
                .map(|e| e.to_owned())
                .collect();
        }
    }

    i32::from_str_radix(ogr[0], 2).unwrap() * i32::from_str_radix(co2sr[0], 2).unwrap()
}
