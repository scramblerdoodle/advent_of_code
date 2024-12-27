use std::fs::read_to_string;

struct Input {
    foo: Vec<String>,
}

fn day14(input: Input) -> u32 {
    let mut result: u32 = 0;
    for line in input.foo {
        result += line.parse::<u32>().expect("Unexpected input");
    }
    result
}

fn day14_v2(input: Input) -> u32 {
    let mut result: u32 = 0;
    for line in input.foo {
        result += line.parse::<u32>().expect("Unexpected input");
    }
    result
}

fn parse_input(filepath: &str) -> Input {
    let mut result: Vec<String> = vec![];
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        result.push(l.to_string());
    });

    Input { foo: result }
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day14(parse_input("./tests/day14/example.txt")),
        "actual" => day14(parse_input("./tests/day14/actual.txt")),
        "example_v2" => day14_v2(parse_input("./tests/day14/example.txt")),
        "actual_v2" => day14_v2(parse_input("./tests/day14/actual.txt")),
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
        assert_eq!(main("example_v2"), 10);
    }
}
