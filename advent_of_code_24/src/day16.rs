use std::{fmt, fs::read_to_string};

use crate::utils::{Board, Direction};

enum State {
    Start,
    End,
    Wall,
    Empty,
    Visited(Direction),
}

impl State {
    fn from_char(c: char) -> State {
        match c {
            '#' => State::Wall,
            '.' => State::Empty,
            'S' => State::Start,
            'E' => State::End,
            _ => panic!("Unexpected symbol in input"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            State::Wall => '#',
            State::Empty => '.',
            State::Start => 'S',
            State::End => 'E',
            State::Visited(d) => d.as_char(),
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
    start: (usize, usize),
    minimum_score: u32,
}

impl Input {
    fn new(board: Vec<Vec<State>>) -> Self {
        let board = Board::new(board);
        let mut start = (0, 0);

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                match Some(board.get_pos((i, j)).unwrap()) {
                    Some(State::Start) => start = (i, j),
                    _ => (),
                }
            }
        }

        Self {
            board,
            start,
            minimum_score: u32::MAX,
        }
    }

    // FIXME: THIS IS INSANELY COMPLEX
    //        Change it to implement a modified BFS with priority queue for lowest score
    fn walk(&mut self, pos: (usize, usize), curr_dir: &Direction, score: u32) {
        /*
         * General idea:
         *   first step: set start as visited, dir EAST
         *
         *   Recursive:
         *   set pos as visited
         *   Iterate over Direction::ORTHOGONALS (Up, Right, Down, Left),
         *       match next_pos
         *           wall | visited: stop
         *           empty: acc score, add 1000 if new direction != curr direction; walk
         *           end: add 1 step; update min
         *           start: panic (shouldnt happen)
         *
         *       when coming back, unset visited, figure out if acc is being reduced
         *       (should be if on stack)
         */

        // 1. Set pos as visited
        self.board.update_pos(pos, State::Visited(curr_dir.clone()));

        // 2. Iterate over orthogonals
        for d in &Direction::ORTHOGONALS {
            let next_pos = self.board.add_direction(d, pos).unwrap();
            let next_state = self.board.get_pos(next_pos).unwrap();

            match next_state {
                State::Wall | State::Visited(_) => {}
                State::Empty => {
                    let mut new_score = score + 1;
                    if *d != *curr_dir {
                        new_score += 1000;
                    }

                    if new_score < self.minimum_score {
                        self.walk(next_pos, d, new_score);
                    }
                }
                State::End => {
                    self.update_min(score + 1);
                }
                State::Start => panic!("How"),
            }
        }

        // 3. At the end, unvisit pos
        self.board.update_pos(pos, State::Empty);
    }

    fn update_min(&mut self, other: u32) {
        if self.minimum_score > other {
            self.minimum_score = other;

            println!("Min: {}\n{}", self.minimum_score, self.board);
        }
    }
}

fn day16(mut input: Input) -> u32 {
    println!("{}", input.board);

    input.walk(input.start, &Direction::Right, 0);
    input.minimum_score
}

fn day16_v2(input: Input) -> u32 {
    let mut result: u32 = 0;
    result
}

fn parse_input(filepath: &str) -> Input {
    let mut board: Vec<Vec<State>> = vec![];

    read_to_string(filepath).unwrap().lines().for_each(|l| {
        board.push(
            l.chars()
                .map(|c| State::from_char(c))
                .collect::<Vec<State>>(),
        );
    });

    Input::new(board)
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day16(parse_input("./tests/day16/example.txt")),
        "example2" => day16(parse_input("./tests/day16/example2.txt")),
        "actual" => day16(parse_input("./tests/day16/actual.txt")),
        "example_v2" => day16_v2(parse_input("./tests/day16/example.txt")),
        "example2_v2" => day16_v2(parse_input("./tests/day16/example2.txt")),
        "actual_v2" => day16_v2(parse_input("./tests/day16/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(main("example"), 7036);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(main("example2"), 11048);
    }

    #[test]
    fn test_example_1_v2() {
        assert_eq!(main("example_v2"), 0);
    }

    #[test]
    fn test_example_2_v2() {
        assert_eq!(main("example2_v2"), 0);
    }

    #[test]
    fn test_actual() {
        assert!(main("actual") < 215808);
    }
}
