use std::fs::read_to_string;

fn match_crosswords(board: Vec<Vec<char>>) -> u32 {
    0
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
