use ndarray::Array2;
use std::cmp::Ordering;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let cords = input.split(',').collect::<Vec<_>>();
        Self {
            x: cords[0].parse::<u32>().unwrap(),
            y: cords[1].parse::<u32>().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Dir {
    Hor,
    Vert,
}

#[derive(Debug)]
pub struct Line {
    src: Point,
    dst: Point,
    dir: Dir,
}

impl From<&String> for Line {
    fn from(input: &String) -> Self {
        let points = input.split(" -> ").collect::<Vec<_>>();
        let (p0, p1) = (Point::from(points[0]), Point::from(points[1]));

        let (src, dst, dir) = match p0.x.cmp(&p1.x) {
            Ordering::Less => (p0, p1, Dir::Hor),

            Ordering::Equal => match p0.y.cmp(&p1.y) {
                Ordering::Less => (p0, p1, Dir::Vert),
                Ordering::Equal => (p0, p1, Dir::Vert),
                Ordering::Greater => (p1, p0, Dir::Vert),
            },

            Ordering::Greater => (p1, p0, Dir::Hor),
        };

        Self { src, dst, dir }
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
    // const SIZE: usize = 12;

    pub fn fill_from(lines: Vec<Line>) -> Self {
        let mut table = Array2::zeros((Self::SIZE, Self::SIZE));

        for line in lines.iter() {
            if line.src.x != line.dst.x && line.src.y != line.dst.y {
                continue;
            }

            match line.dir {
                Dir::Hor => {
                    for p in table
                        .row_mut(line.src.y as usize)
                        .iter_mut()
                        .take(line.dst.x as usize + 1)
                        .skip(line.src.x as usize)
                    {
                        *p += 1
                    }
                }

                Dir::Vert => {
                    for p in table
                        .column_mut(line.src.x as usize)
                        .iter_mut()
                        .take(line.dst.y as usize + 1)
                        .skip(line.src.y as usize)
                    {
                        *p += 1
                    }
                }
            }
        }

        Self(table)
    }

    pub fn count_dangerous_areas(&self) -> u32 {
        // dbg!(&self.0);
        let mut areas = 0;
        for p in self.0.iter() {
            if *p >= 2 {
                areas += 1;
            }
        }
        areas
    }
}
