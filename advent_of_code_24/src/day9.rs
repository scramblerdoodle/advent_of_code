use std::{fs::read_to_string, num::ParseIntError};

struct Input {
    foo: String,
}

fn day9(inputs: Vec<Input>) -> Result<u32, ParseIntError> {
    let mut result: u32 = 0;
    for input in inputs {
        result += input.foo.parse::<u32>()?;
    }
    Ok(result)
}

fn day9_v2(inputs: Vec<Input>) -> Result<u32, ParseIntError> {
    let mut result: u32 = 0;
    for input in inputs {
        result += input.foo.parse::<u32>()?;
    }
    Ok(result)
}

fn parse_input(filepath: &str) -> Vec<Input> {
    let mut result: Vec<Input> = vec![];
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        result.push(Input { foo: l.to_string() });
    });

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
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 0);
    }
}
