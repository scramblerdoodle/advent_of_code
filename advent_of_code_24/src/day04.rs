use crate::utils::{Board, Direction};
use std::fs::read_to_string;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn check_match(board: &Board<char>, pos: (usize, usize), d: Direction, l: usize) -> bool {
    let next_pos = board.add_direction(&d, pos);

    if l == XMAS.len() {
        return true;
    }

    if next_pos.is_none() {
        return false;
    }

    let next_pos = next_pos.unwrap();

    let c = board[next_pos.0][next_pos.1];

    // Got a match for next letter
    if c == XMAS[l] {
        // Repeat for next letter in next position
        check_match(board, next_pos, d, l + 1)
    } else {
        false
    }
}

fn match_crosswords(board: Board<char>) -> u32 {
    let mut matches: u32 = 0;

    // for every element in board
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            // if it's X
            if board[i][j] == XMAS[0] {
                // check surroundings
                for d in Direction::DIRECTIONS {
                    matches += match check_match(&board, (i, j), d, 1) {
                        true => 1,
                        false => 0,
                    };
                }
            }
        }
    }
    matches
}

fn check_match_v2(board: &Board<char>, pos: (usize, usize)) -> bool {
    /*
     *   SOLUTION 2:
     *       start with A
     *       count surrounding diagonals
     *       count if gravitational pull is toward centre
     */

    let mut m_gravity = (0, 0);
    let mut s_gravity = (0, 0);
    for d in Direction::DIAGONALS {
        let next_pos = board.add_direction(&d, pos);
        if next_pos.is_none() {
            return false;
        }

        let next_pos = next_pos.unwrap();

        let step = d.get_direction();
        match board[next_pos.0][next_pos.1] {
            'M' => {
                m_gravity.0 += step.0;
                m_gravity.1 += step.1;
            }
            'S' => {
                s_gravity.0 += step.0;
                s_gravity.1 += step.1;
            }
            _ => break,
        }
    }
    return (m_gravity.0.abs() == 2 || m_gravity.1.abs() == 2)
        && (s_gravity.0.abs() == 2 || s_gravity.1.abs() == 2);
}

fn match_crosswords_v2(board: Board<char>) -> u32 {
    let mut matches: u32 = 0;

    // for every element in board
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            // if it's A
            if board[i][j] == 'A' {
                if check_match_v2(&board, (i, j)) {
                    matches += 1;
                }
            }
        }
    }
    matches
}

fn parse_input(filepath: &str) -> Board<char> {
    let mut result: Vec<Vec<char>> = vec![];
    for line in read_to_string(filepath).unwrap().lines() {
        result.push(line.chars().collect());
    }
    Board::new(result)
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
