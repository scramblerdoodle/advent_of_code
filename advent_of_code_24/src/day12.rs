use std::fs::read_to_string;

use crate::utils::Board;

struct Garden {
    board: Board<char>,
    visited: Board<bool>,
}

fn day12(garden: Garden) -> u32 {
    println!("{}", garden.board);
    println!("{}", garden.visited);
    0
}

fn day12_v2(garden: Garden) -> u32 {
    println!("{}", garden.board);
    println!("{}", garden.visited);
    0
}

fn parse_input(filepath: &str) -> Garden {
    let mut board: Vec<Vec<char>> = vec![];
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        board.push(l.chars().collect());
    });

    let visited = vec![vec![false; board[0].len()]; board.len()];

    Garden {
        board: Board::new(board),
        visited: Board::new(visited),
    }
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
        assert_eq!(main("example"), 10);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example"), 10);
    }
}
