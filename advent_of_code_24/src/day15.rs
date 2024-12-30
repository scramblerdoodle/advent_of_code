use std::{fmt, fs::read_to_string};

use crate::utils::{Board, Direction};

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
    moves: Vec<Direction>,
}

fn day15(input: Input) -> u32 {
    let mut result: u32 = 0;
    result
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
    let input = Input {
        board: Board::new(board),
        moves,
    };
    println!("{}", input.board);
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
        assert_eq!(main("example"), 10);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 10);
    }
}
