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

const DIAGONALS: [(i16, i16); 4] = [
    DIRECTIONS[0], // Top Left
    DIRECTIONS[2], // Top Right
    DIRECTIONS[5], // Down Left
    DIRECTIONS[7], // Down Right
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

fn check_match_v2(board: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    /*
     *   SOLUTION 2:
     *       start with A
     *       count surrounding diagonals
     *       if count('M') == 2 and count('S') == 2 {
     *           return true
     *       } else {
     *           return false
     *       }
     *
     *   PROBLEM:
     *       if we just count Ms and Ss, we get false positives with
     *           M . S
     *           . A .
     *           S . M
     */

    let mut m_gravity: (i16, i16) = (0, 0);
    let mut s_gravity: (i16, i16) = (0, 0);
    for d in DIAGONALS {
        let next_i = i as i16 + d.0;
        let next_j = j as i16 + d.1;
        if next_i < 0 || next_j < 0 {
            return false;
        }

        let next_i = next_i as usize;
        let next_j = next_j as usize;

        match board.get(next_i) {
            None => break, // out of bounds
            Some(v) => {
                match v.get(next_j) {
                    None => break, // out of bounds
                    Some(c) => {
                        match *c {
                            'M' => {
                                m_gravity.0 += d.0;
                                m_gravity.1 += d.1;
                            }
                            'S' => {
                                s_gravity.0 += d.0;
                                s_gravity.1 += d.1;
                            }
                            _ => break,
                        };
                    }
                }
            }
        }
    }
    if (m_gravity.0.abs() == 2 || m_gravity.1.abs() == 2)
        && (s_gravity.0.abs() == 2 || s_gravity.1.abs() == 2)
    {
        return true;
    } else {
        return false;
    }
}

fn match_crosswords_v2(board: Vec<Vec<char>>) -> u32 {
    let mut matches: u32 = 0;

    let mut board_copy: Vec<Vec<char>> = vec![];

    // for every element in board
    for i in 0..board.len() {
        let mut b: Vec<char> = vec![];

        for j in 0..board[i].len() {
            // if it's A
            if board[i][j] == 'A' {
                let is_matching = check_match_v2(&board, i, j);
                if is_matching {
                    matches += 1;
                    b.push('A');
                } else {
                    b.push('.');
                }
            } else {
                b.push('.');
            }
        }

        board_copy.push(b);
    }
    matches
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
        "example" => match_crosswords(parse_input("./tests/day04/example.txt")),
        "actual" => match_crosswords(parse_input("./tests/day04/actual.txt")),
        "example_v2" => match_crosswords_v2(parse_input("./tests/day04/example.txt")),
        "actual_v2" => match_crosswords_v2(parse_input("./tests/day04/actual.txt")),
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
        assert_eq!(main("example_v2"), 9);
    }
}
