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

    fn push_n(&mut self, s: String, n: u64) {
        for _ in 0..n {
            self.file_blocks.push_str(&s[..]);
        }
    }
}

fn day9(input: Input) -> Result<u64, ParseIntError> {
    let mut result: u64 = 0;
    let mut files: Vec<char> = input.file_blocks.trim_matches('.').chars().collect();

    let mut j: usize = files.len() - 1;

    for i in 0..files.len() {
        // println!("i: {i}, j: {j}");
        // println!("{:?}", String::from_iter(&files));
        if files[i] == '.' {
            files[i] = files[j];
            files[j] = '.';
            while files[j] == '.' {
                j -= 1;
            }
        }

        if i >= j {
            break;
        }
    }

    println!("{:?}", String::from_iter(&files));

    let blocks: Vec<u64> = files
        .iter()
        .filter(|c| **c != '.')
        .map(|c| c.to_digit(10).unwrap().into())
        .collect();
    for i in 0..blocks.len() {
        result += (i as u64) * blocks[i];
    }
    Ok(result)
}

fn day9_v2(input: Input) -> Result<u64, ParseIntError> {
    let mut result: u64 = 0;
    // for input in inputs {
    //     result += input.file_blocks.parse::<u64>()?;
    // }
    Ok(result)
}

fn parse_input(filepath: &str) -> Input {
    let mut state: State = State::FILE;
    let mut result: Input = Input::new();
    let mut file_id: u64 = 0;
    read_to_string(filepath)
        .unwrap()
        .trim()
        .chars()
        .for_each(|c| {
            let d = c.to_digit(10).unwrap();
            match state {
                State::FILE => {
                    result.push_n(file_id.to_string(), d.into());
                    file_id += 1;
                }
                State::FREE => result.push_n(".".to_string(), d.into()),
            }
            state = state.change_state();
        });

    println!("{:?}", result);
    result
}

pub fn main(s: &str) -> u64 {
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
        assert_eq!(main("example"), 1928);
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

    #[test]
    fn test_actual() {
        assert!(main("actual") > 91337315408);
    }
}
