use std::fs::read_to_string;

use crate::utils::{Board, Direction};

struct Garden {
    board: Board<char>,
    visited: Board<bool>,
    perimeter: u32,
    area: u32,
}

impl Garden {
    fn new(board: Vec<Vec<char>>) -> Self {
        let visited = vec![vec![false; board[0].len()]; board.len()];
        Garden {
            board: Board::new(board),
            visited: Board::new(visited),
            perimeter: 0,
            area: 0,
        }
    }

    fn find_region(&mut self, pos: (usize, usize)) {
        self.board
            .get(pos.0)
            .expect("i index of bounds")
            .get(pos.1)
            .expect("j index out of bounds");

        self.area += 1;
        self.visited[pos.0][pos.1] = true;

        for dir in Direction::ORTHOGONALS {
            let next_pos = self.board.add_direction(&dir, pos);
            // println!("Curr pos: {:?}, Next pos: {:?}", pos, next_pos);
            match next_pos {
                None => self.perimeter += 1,
                Some(next_pos) => {
                    if self.board[next_pos.0][next_pos.1] != self.board[pos.0][pos.1] {
                        // println!("Next position is not part of region");
                        self.perimeter += 1;
                    } else if !self.visited[next_pos.0][next_pos.1] {
                        // println!("Next position is part of region");
                        self.find_region(next_pos);
                    }
                }
            }
        }
    }
}

fn day12(mut garden: Garden) -> u32 {
    // println!("{}", garden.board);
    // println!("{}", garden.visited);

    let mut result: u32 = 0;
    for i in 0..garden.board.len() {
        for j in 0..garden.board[i].len() {
            if !garden.visited[i][j] {
                garden.find_region((i, j));
                // println!(
                //     "{}: Area {} * Perimeter {}",
                //     garden.board[i][j], garden.area, garden.perimeter
                // );
                result += garden.area * garden.perimeter;
                garden.area = 0;
                garden.perimeter = 0;
            }
        }
    }

    // println!("{}", garden.visited);
    result
}

fn day12_v2(garden: Garden) -> u32 {
    // println!("{}", garden.board);
    // println!("{}", garden.visited);
    0
}

fn parse_input(filepath: &str) -> Garden {
    let mut board: Vec<Vec<char>> = vec![];
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        board.push(l.chars().collect());
    });

    Garden::new(board)
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day12(parse_input("./tests/day12/example.txt")),
        "actual" => day12(parse_input("./tests/day12/actual.txt")),
        "example_v2" => day12_v2(parse_input("./tests/day12/example.txt")),
        "actual_v2" => day12_v2(parse_input("./tests/day12/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 1930);
    }

    // #[test]
    // fn test_example_v2() {
    //     assert_eq!(main("example_v2"), 10);
    // }
}
