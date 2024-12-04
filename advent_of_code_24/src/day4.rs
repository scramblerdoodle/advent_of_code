use std::fs::read_to_string;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRECTIONS: [(i16, i16); 8] = [
    (-1, -1), // Top Left
    (-1, 0),  // Top
    (-1, 1),  // Top Right
    (0, -1),  // Left
    (0, 1),   // Right
    (1, -1),  // Down Left
    (1, 0),   // Down
    (1, 1),   // Down Right
];

fn check_match(board: &Vec<Vec<char>>, i: usize, j: usize, d: (i16, i16), l: usize) -> bool {
    let next_i = i as i16 + d.0;
    let next_j = j as i16 + d.1;

    if l == XMAS.len() {
        return true;
    }

    if next_i < 0 || next_j < 0 {
        return false;
    }

    let next_i = next_i as usize;
    let next_j = next_j as usize;

    match board.get(next_i) {
        None => false, // out of bounds
        Some(v) => {
            match v.get(next_j) {
                None => false, // out of bounds
                Some(c) => {
                    // Got a match for next letter
                    if *c == XMAS[l] {
                        // Repeat for next letter in next position
                        check_match(board, next_i, next_j, d, l + 1)
                    } else {
                        false
                    }
                }
            }
        }
    }
}

fn match_crosswords(board: Vec<Vec<char>>) -> u32 {
    let mut matches: u32 = 0;

    // for every element in board
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            // if it's X
            if board[i][j] == XMAS[0] {
                // check surroundings
                for d in DIRECTIONS {
                    matches += match check_match(&board, i, j, d, 1) {
                        true => 1,
                        false => 0,
                    };
                }
            }
        }
    }
    matches
}

fn match_crosswords_v2(board: Vec<Vec<char>>) -> u32 {
    0
}

fn parse_input(filepath: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![];
    for line in read_to_string(filepath).unwrap().lines() {
        result.push(line.chars().collect());
    }
    result
}
pub fn main(s: &str) -> u32 {
    match s {
        "example" => match_crosswords(parse_input("./tests/day4/example.txt")),
        "actual" => match_crosswords(parse_input("./tests/day4/actual.txt")),
        "example_v2" => match_crosswords_v2(parse_input("./tests/day4/example.txt")),
        "actual_v2" => match_crosswords_v2(parse_input("./tests/day4/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 18);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 0);
    }
}
