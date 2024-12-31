use std::fs::read_to_string;

struct Input {
    foo: Vec<String>,
}

fn day16(input: Input) -> u32 {
    let mut result: u32 = 0;
    for line in input.foo {
        result += line.parse::<u32>().expect("Unexpected input");
    }
    result
}

fn day16_v2(input: Input) -> u32 {
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
        "example" => day16(parse_input("./tests/day16/example.txt")),
        "example2" => day16(parse_input("./tests/day16/example2.txt")),
        "actual" => day16(parse_input("./tests/day16/actual.txt")),
        "example_v2" => day16_v2(parse_input("./tests/day16/example.txt")),
        "example2_v2" => day16_v2(parse_input("./tests/day16/example2.txt")),
        "actual_v2" => day16_v2(parse_input("./tests/day16/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(main("example"), 7036);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(main("example2"), 11048);
    }

    #[test]
    fn test_example_1_v2() {
        assert_eq!(main("example_v2"), 0);
    }

    #[test]
    fn test_example_2_v2() {
        assert_eq!(main("example2_v2"), 0);
    }
}
