#[derive(Debug)]
pub struct Board {
    board: Vec<Vec<(String, bool)>>,
    last_num: i32,
}

impl From<String> for Board {
    fn from(input: String) -> Self {
        Self {
            board: input
                .lines()
                .collect::<Vec<_>>()
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|numbers| (numbers.to_owned(), false))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            last_num: 0,
        }
    }
}

impl Board {
    fn check_rows(&self) -> bool {
        for row in &self.board {
            if row.iter().all(|num| num.1) {
                return true;
            }
        }
        false
    }

    fn check_cols(&self) -> bool {
        for i in 0..self.board.len() {
            let col = self
                .board
                .iter()
                .map(|row| row.get(i).unwrap())
                .collect::<Vec<_>>();
            if col.iter().all(|num| num.1) {
                return true;
            }
        }
        false
    }

    pub fn solve(&mut self, numbers: &[String]) -> i32 {
        for (i, num) in numbers.iter().enumerate() {
            for row in self.board.iter_mut() {
                for col in row.iter_mut() {
                    if col.0 == *num {
                        col.1 = true;
                    }
                }
            }

            if self.check_cols() || self.check_rows() {
                self.last_num = num.parse::<i32>().unwrap();
                return i as i32 + 1;
            }
        }
        0
    }

    pub fn calculate_score(&self) -> i32 {
        let mut unmarked_sum = 0;
        self.board.iter().for_each(|line| {
            line.iter().for_each(|(num, marked)| {
                if !marked {
                    unmarked_sum += num.parse::<i32>().unwrap()
                }
            })
        });
        unmarked_sum * self.last_num
    }
}

#[derive(Debug)]
pub struct Bingo {
    numbers: Vec<String>,
    boards: Vec<Board>,
}

impl From<Vec<String>> for Bingo {
    fn from(input: Vec<String>) -> Self {
        Self {
            numbers: input[0]
                .split(',')
                .map(|num| num.to_owned())
                .collect::<Vec<_>>(),
            boards: input
                .iter()
                .skip(1)
                .map(|board| Board::from(board.to_owned()))
                .collect::<Vec<_>>(),
        }
    }
}

impl Bingo {
    pub fn get_first_last_boards(&mut self) -> (&Board, &Board) {
        let mut first_board: (i32, usize) = (self.numbers.len() as i32 + 1, 0);
        let mut last_board: (i32, usize) = (0, 0);
        for (i, board) in self.boards.iter_mut().enumerate() {
            let moves = board.solve(&self.numbers);
            if moves > last_board.0 {
                last_board = (moves, i);
            }
            if moves < first_board.0 {
                first_board = (moves, i);
            }
        }
        (&self.boards[first_board.1], &self.boards[last_board.1])
    }
}
