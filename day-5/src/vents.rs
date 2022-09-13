use ndarray::Array2;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let cords = input.split(',').collect::<Vec<_>>();
        Self {
            x: cords[0].parse::<i32>().unwrap(),
            y: cords[1].parse::<i32>().unwrap(),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

enum Dir {
    Pos,
    Neg,
    None,
}

impl Dir {
    fn to_i32(&self) -> i32 {
        match self {
            Dir::Pos => 1,
            Dir::Neg => -1,
            Dir::None => 0,
        }
    }
}

pub struct Line {
    src: Point,
    dst: Point,
    dir_x: Dir,
    dir_y: Dir,
}

impl From<&String> for Line {
    fn from(input: &String) -> Self {
        let points = input.split(" -> ").collect::<Vec<_>>();
        let (p0, p1) = (Point::from(points[0]), Point::from(points[1]));

        let dir_x = match p0.x.cmp(&p1.x) {
            Ordering::Less => Dir::Pos,
            Ordering::Equal => Dir::None,
            Ordering::Greater => Dir::Neg,
        };

        let dir_y = match p0.y.cmp(&p1.y) {
            Ordering::Less => Dir::Pos,
            Ordering::Equal => Dir::None,
            Ordering::Greater => Dir::Neg,
        };

        Self {
            src: p0,
            dst: p1,
            dir_x,
            dir_y,
        }
    }
}

impl Line {
    pub fn parse_lines(input: &[String]) -> Vec<Line> {
        input.iter().map(Line::from).collect()
    }
}

pub struct Table(Array2<u32>);

impl Table {
    const SIZE: usize = 989;

    pub fn fill_from(lines: Vec<Line>) -> Self {
        let mut table = Array2::zeros((Self::SIZE, Self::SIZE));

        for line in lines.iter() {
            let mut pos = line.src;
            while pos != line.dst {
                *table.get_mut((pos.x as usize, pos.y as usize)).unwrap() += 1;

                pos.x += line.dir_x.to_i32();
                pos.y += line.dir_y.to_i32();
            }
            *table.get_mut((pos.x as usize, pos.y as usize)).unwrap() += 1;
        }

        Self(table)
    }

    pub fn count_dangerous_areas(&self) -> u32 {
        let mut areas = 0;
        for p in self.0.iter() {
            if *p >= 2 {
                areas += 1;
            }
        }
        areas
    }
}
