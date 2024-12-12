use std::{fs::read_to_string, num::ParseIntError};

struct Input {
    foo: String,
}

fn day0(inputs: Vec<Input>) -> Result<u32, ParseIntError> {
    let mut result: u32 = 0;
    for input in inputs {
        result += input.foo.parse::<u32>()?;
    }
    Ok(result)
}

fn day0_v2(inputs: Vec<Input>) -> Result<u32, ParseIntError> {
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

pub fn main(s: &str) -> Result<u32, ParseIntError> {
    match s {
        "example" => day0(parse_input("./tests/day0/example.txt")),
        "actual" => day0(parse_input("./tests/day0/actual.txt")),
        "example_v2" => day0_v2(parse_input("./tests/day0/example.txt")),
        "actual_v2" => day0_v2(parse_input("./tests/day0/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example").unwrap(), 10);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example").unwrap(), 10);
    }
}
