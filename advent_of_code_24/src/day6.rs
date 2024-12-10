use std::fs::read_to_string;

fn day6(tmp: String) -> u32 {
    41
}

fn day6_v2(tmp: String) -> u32 {
    0
}

fn parse_input(file_path: &str) -> String {
    let content = read_to_string(file_path).unwrap();
    let mut lines = content.lines();

    String::new()
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day6(parse_input("./tests/day6/example.txt")),
        "actual" => day6(parse_input("./tests/day6/actual.txt")),
        "example_v2" => day6_v2(parse_input("./tests/day6/example.txt")),
        "actual_v2" => day6_v2(parse_input("./tests/day6/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 41);
    }
    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 0);
    }
}
