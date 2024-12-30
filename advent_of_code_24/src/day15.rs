use std::{fmt, fs::read_to_string};

use crate::utils::{Board, Direction};

#[derive(PartialEq)]
enum State {
    Wall,
    Empty,
    Box,
    Robot,
}

impl State {
    fn from_char(c: char) -> State {
        match c {
            '#' => State::Wall,
            '.' => State::Empty,
            'O' => State::Box,
            '@' => State::Robot,
            _ => panic!("Unexpected symbol in input"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            State::Wall => '#',
            State::Empty => '.',
            State::Box => 'O',
            State::Robot => '@',
        }
    }
}

impl fmt::Display for Board<State> {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            concat!("Board:\n\t{}"),
            self.board
                .iter()
                .map(|v| v.iter().map(|s| s.to_char()).collect())
                .collect::<Vec<String>>()
                .join("\n\t"),
        )
    }
}

struct Input {
    board: Board<State>,
    robot_pos: (usize, usize),
    moves: Vec<Direction>,
}

impl Input {
    fn move_robot(&mut self, d: &Direction) {
        let next_pos = self.board.add_direction(d, self.robot_pos).unwrap();
        let next_state = self.board.get_pos(next_pos).unwrap();

        match *next_state {
            State::Empty => {
                // Move into empty position
                self.board.update_pos(self.robot_pos, State::Empty);
                self.board.update_pos(next_pos, State::Robot);
                self.robot_pos = next_pos;
            }
            State::Wall => {
                // Do nothing
            }
            State::Box => {
                // Try to push box
                if self.push_box(d, next_pos) {
                    self.board.update_pos(self.robot_pos, State::Empty);
                    self.board.update_pos(next_pos, State::Robot);
                    self.robot_pos = next_pos;
                };
            }
            State::Robot => panic!("There should only be one robot"),
        }

        // println!("{}", self.board);
    }

    fn push_box(&mut self, d: &Direction, pos: (usize, usize)) -> bool {
        let next_pos = self.board.add_direction(d, pos).unwrap();
        let next_state = self.board.get_pos(next_pos).unwrap();

        match *next_state {
            State::Empty => {
                self.board.update_pos(pos, State::Empty);
                self.board.update_pos(next_pos, State::Box);
                true
            }
            State::Wall => false,
            State::Box => {
                let is_box_pushable = self.push_box(d, next_pos);
                if is_box_pushable {
                    self.board.update_pos(pos, State::Empty);
                    self.board.update_pos(next_pos, State::Box);
                }
                is_box_pushable
            }
            State::Robot => panic!("How is the Robot pushing the box unto itself"),
        }
    }

    fn sum_box_gps(self) -> u32 {
        let mut result = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == State::Box {
                    result += 100 * i + j;
                }
            }
        }

        result as u32
    }
}

fn day15(mut input: Input) -> u32 {
    for d in input.moves.clone() {
        input.move_robot(&d);
    }
    input.sum_box_gps()
}

fn day15_v2(input: Input) -> u32 {
    let mut result: u32 = 0;
    result
}

fn parse_input(filepath: &str) -> Input {
    let mut board: Vec<Vec<State>> = vec![];
    let mut moves: Vec<Direction> = vec![];

    enum ReadingMode {
        Board,
        Moves,
    }
    let mut reading_mode = ReadingMode::Board;
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        if l == "" {
            reading_mode = ReadingMode::Moves;
        } else {
            match reading_mode {
                ReadingMode::Board => {
                    board.push(
                        l.chars()
                            .map(|c| State::from_char(c))
                            .collect::<Vec<State>>(),
                    );
                }
                ReadingMode::Moves => {
                    l.chars().for_each(|c| moves.push(Direction::from_char(c)));
                }
            }
        }
    });

    // TODO: Could be refactored
    let mut robot_pos: (usize, usize) = (0, 0);
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == State::Robot {
                robot_pos = (i, j);
            }
        }
    }
    let input = Input {
        board: Board::new(board),
        robot_pos,
        moves,
    };
    // println!("{}", input.board);
    input
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day15(parse_input("./tests/day15/example.txt")),
        "actual" => day15(parse_input("./tests/day15/actual.txt")),
        "example_v2" => day15_v2(parse_input("./tests/day15/example.txt")),
        "actual_v2" => day15_v2(parse_input("./tests/day15/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 10092);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 10);
    }
}
