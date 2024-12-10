use std::fs::read_to_string;

#[derive(Clone)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Clone)]
struct GuardBoard {
    board: Vec<Vec<char>>,
    guard_dir: Direction,
    pos: (usize, usize),
    visited_board: Vec<Vec<i8>>,
    count: u32,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Direction::UP => '^',
            Direction::RIGHT => '>',
            Direction::DOWN => 'v',
            Direction::LEFT => '<',
        }
    }

    fn get_step(&self) -> (i8, i8) {
        match self {
            Direction::UP => (-1, 0),
            Direction::RIGHT => (0, 1),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
        }
    }
}

impl GuardBoard {
    fn new(board: Vec<Vec<char>>, starting_pos: (usize, usize)) -> Self {
        let guard_dir = match board[starting_pos.0][starting_pos.1] {
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            '<' => Direction::LEFT,
            other => panic!(
                "Unmatched direction. Found {other} in pos ({},{}).",
                starting_pos.0, starting_pos.1
            ),
        };

        let mut visited_board = vec![vec![0; board[0].len()]; board.len()];
        visited_board[starting_pos.0][starting_pos.1] = 1;
        GuardBoard {
            board,
            guard_dir,
            pos: starting_pos,
            visited_board,
            count: 0,
        }
    }

    fn walk(mut self) -> u32 {
        let mut pos = self.pos;
        loop {
            let step = self.guard_dir.get_step();
            let next_pos = (pos.0 as i16 + step.0 as i16, pos.1 as i16 + step.1 as i16);

            if next_pos.0 < 0 || next_pos.1 < 0 {
                self.count += 1;
                break;
            }

            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

            match self.board.get(next_pos.0) {
                None => {
                    self.count += 1;
                    break;
                } // Out of bounds
                Some(line) => match line.get(next_pos.1) {
                    None => {
                        self.count += 1;
                        break;
                    } // Out of bounds
                    Some(c) => match c {
                        '.' => {
                            self.board[pos.0][pos.1] = 'X';
                            // self.visited_board[pos.0][pos.1] = true;
                            self.board[next_pos.0][next_pos.1] = self.guard_dir.as_char();
                            pos = next_pos;
                            self.count += 1;
                        }
                        '#' => self.guard_dir = self.guard_dir.next(),
                        'X' => {
                            self.board[next_pos.0][next_pos.1] = self.guard_dir.as_char();
                            self.board[pos.0][pos.1] = 'X';
                            pos = next_pos;
                        }
                        other => panic!("Unexpected char {other}"),
                    },
                },
            }
        }

        // self.print_board();

        self.count
    }

    #[allow(dead_code)]
    fn print_board(&self) -> () {
        use log_update::LogUpdate;
        use std::{io::stdout, thread::sleep, time::Duration};

        let mut log_update = LogUpdate::new(stdout()).unwrap();

        let lines = self
            .board
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<String>>();

        sleep(Duration::from_millis(1));
        log_update.render(&format!("{}", lines.join("\n"))).unwrap();
    }

    fn is_loop(&mut self) -> bool {
        loop {
            let step = self.guard_dir.get_step();
            let next_pos = (
                self.pos.0 as i16 + step.0 as i16,
                self.pos.1 as i16 + step.1 as i16,
            );

            if next_pos.0 < 0 || next_pos.1 < 0 {
                return false;
            }

            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

            // Next position
            match self.board.get(next_pos.0) {
                None => {
                    return false;
                } // Out of bounds
                Some(line) => match line.get(next_pos.1) {
                    None => {
                        return false;
                    } // Out of bounds
                    Some(c) => match c {
                        '.' => {
                            self.board[self.pos.0][self.pos.1] = 'X';
                            self.visited_board[next_pos.0][next_pos.1] += 1;
                            self.board[next_pos.0][next_pos.1] = self.guard_dir.as_char();
                            self.pos = next_pos;
                        }
                        '#' | 'O' => {
                            self.guard_dir = self.guard_dir.next();
                            self.board[self.pos.0][self.pos.1] = self.guard_dir.as_char();
                        }
                        'X' => {
                            self.visited_board[next_pos.0][next_pos.1] += 1;
                            if self.visited_board[next_pos.0][next_pos.1] == 4 {
                                return true;
                            };

                            self.board[self.pos.0][self.pos.1] = 'X';
                            self.board[next_pos.0][next_pos.1] = self.guard_dir.as_char();
                            self.pos = next_pos;
                        }
                        other => panic!("Unexpected char {other}"),
                    },
                },
            }
        }
    }

    fn place_obstacles(mut self) -> u32 {
        loop {
            let step = self.guard_dir.get_step();
            let next_pos = (self.pos.0 as i8 + step.0, self.pos.1 as i8 + step.1);

            if next_pos.0 < 0 || next_pos.1 < 0 {
                break;
            }

            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

            match self.board.get(next_pos.0) {
                None => {
                    break;
                } // Out of bounds
                Some(line) => match line.get(next_pos.1) {
                    None => {
                        break;
                    } // Out of bounds
                    Some(c) => match c {
                        '.' => {
                            // What happens if we place an obstacle in next_pos?

                            let mut self_clone = self.clone();

                            self_clone.board[next_pos.0][next_pos.1] = 'O';

                            if self_clone.is_loop() {
                                self.count += 1;
                            }

                            self.board[self.pos.0][self.pos.1] = 'X';
                            self.board[next_pos.0][next_pos.1] = self.guard_dir.as_char();
                            self.pos = next_pos;
                        }
                        '#' => self.guard_dir = self.guard_dir.next(),
                        'X' => {
                            // What happens if we place an obstacle in next_pos?
                            self.board[self.pos.0][self.pos.1] = 'X';
                            self.board[next_pos.0][next_pos.1] = self.guard_dir.as_char();
                            self.pos = next_pos;
                        }
                        other => panic!("Unexpected char {other}"),
                    },
                },
            }
        }

        self.count
    }
}

fn day6(guard_board: GuardBoard) -> u32 {
    guard_board.walk()
}

fn day6_v2(guard_board: GuardBoard) -> u32 {
    guard_board.place_obstacles()
}

fn parse_input(file_path: &str) -> GuardBoard {
    let content = read_to_string(file_path).unwrap();
    let lines = content.lines();
    let board: Vec<Vec<char>> = lines.map(|l| l.chars().collect::<Vec<char>>()).collect();
    let mut starting_pos = (0, 0);

    let mut i: usize = 0;
    'board_loop: loop {
        if i == board.len() {
            break;
        }

        for j in 0..board.get(i).unwrap().len() {
            if board[i][j] == '^' {
                starting_pos = (i, j);
                break 'board_loop;
            }
        }

        i += 1;
    }

    GuardBoard::new(board, starting_pos)
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day6(parse_input("./tests/day6/example.txt")),
        "actual" => day6(parse_input("./tests/day6/actual.txt")),
        "example_v2" => day6_v2(parse_input("./tests/day6/example.txt")),
        "actual_v2" => day6_v2(parse_input("./tests/day6/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 41);
    }
    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 6);
    }
}
