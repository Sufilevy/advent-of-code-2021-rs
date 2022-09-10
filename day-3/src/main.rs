use std::{cmp::Ordering, fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let mut input = input.split('\n').collect::<Vec<&str>>();
    input.pop();
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
            .for_each(|line| match line.chars().nth(11 - i).unwrap() {
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
    let filter_rating = |msb, lsb| {
        let mut vec = input.to_owned().clone();
        let mut i = 0;
        while vec.len() > 1 {
            let mut appearances = (0, 0);
            vec.iter()
                .for_each(|line| match line.chars().nth(i).unwrap() {
                    '1' => appearances.1 += 1,
                    '0' => appearances.0 += 1,
                    _ => {}
                });

            let filter = match appearances.1.cmp(&appearances.0) {
                Ordering::Greater => msb,
                Ordering::Equal => msb,
                Ordering::Less => lsb,
            };

            vec.retain(|num| num.to_owned().chars().nth(i).unwrap() == filter);
            i += 1;
        }

        vec[0].to_string().trim_end_matches('\r').to_owned()
    };

    let ogr = filter_rating('1', '0'); // Oxygen generator rating
    let co2sr = filter_rating('0', '1'); // CO2 scrubber rating

    i32::from_str_radix(&ogr, 2).unwrap() * i32::from_str_radix(&co2sr, 2).unwrap()
}
