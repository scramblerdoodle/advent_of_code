use std::{fs::read_to_string, num::ParseIntError};

enum State {
    FILE,
    FREE,
}

impl State {
    fn change_state(&mut self) -> Self {
        match self {
            State::FILE => State::FREE,
            State::FREE => State::FILE,
        }
    }
}

#[derive(Debug)]
struct Input {
    file_blocks: String,
}

impl Input {
    fn new() -> Self {
        Self {
            file_blocks: String::new(),
        }
    }

    fn push_n(&mut self, s: String, n: u32) {
        for _ in 0..n {
            self.file_blocks.push_str(&s[..]);
        }
    }
}

fn day9(input: Input) -> Result<u32, ParseIntError> {
    let mut result: u32 = 0;
    // for input in inputs {
    //     result += input.file_blocks.parse::<u32>()?;
    // }
    Ok(result)
}

fn day9_v2(input: Input) -> Result<u32, ParseIntError> {
    let mut result: u32 = 0;
    // for input in inputs {
    //     result += input.file_blocks.parse::<u32>()?;
    // }
    Ok(result)
}

fn parse_input(filepath: &str) -> Input {
    let mut state: State = State::FILE;
    let mut result: Input = Input::new();
    let mut file_id: u32 = 0;
    read_to_string(filepath)
        .unwrap()
        .trim()
        .chars()
        .for_each(|c| {
            let d = c.to_digit(10).unwrap();
            match state {
                State::FILE => {
                    result.push_n(file_id.to_string(), d);
                    file_id += 1;
                }
                State::FREE => result.push_n(".".to_string(), d),
            }
            state = state.change_state();
        });

    println!("{:?}", result);
    result
}

pub fn main(s: &str) -> u32 {
    let result = match s {
        "example" => day9(parse_input("./tests/day9/example.txt")),
        "actual" => day9(parse_input("./tests/day9/actual.txt")),
        "example_v2" => day9_v2(parse_input("./tests/day9/example.txt")),
        "actual_v2" => day9_v2(parse_input("./tests/day9/actual.txt")),
        _ => todo!(),
    };
    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 10);
    }

    #[test]
    fn test_input() {
        assert_eq!(
            parse_input("./tests/day9/example.txt").file_blocks,
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 0);
    }
}
