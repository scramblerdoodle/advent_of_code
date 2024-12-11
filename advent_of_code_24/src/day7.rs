use std::fs::read_to_string;

fn day7(tmp: String) -> u32 {
    0
}

fn day7_v2(tmp: String) -> u32 {
    0
}

fn parse_input(file_path: &str) -> String {
    let content = read_to_string(file_path).unwrap();
    let lines = content.lines();

    "a".to_string()
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day7(parse_input("./tests/day7/example.txt")),
        "actual" => day7(parse_input("./tests/day7/actual.txt")),
        "example_v2" => day7_v2(parse_input("./tests/day7/example.txt")),
        "actual_v2" => day7_v2(parse_input("./tests/day7/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 0);
    }
    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 0);
    }
}
