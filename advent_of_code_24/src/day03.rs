use regex::Regex;
use std::fs::read_to_string;

#[derive(PartialEq)]
enum State {
    Dont,
    Do,
}

fn compute_tokens_v2(filepath: &str) -> u32 {
    let input = &read_to_string(filepath).unwrap()[..];
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\([0-9]+,[0-9]+\))").unwrap();
    let mut tokens: Vec<&str> = vec![];
    for (_, [val]) in re.captures_iter(input).map(|c| c.extract()) {
        tokens.push(val);
    }

    let mut result: u32 = 0;

    let mut state = State::Do;
    for t in tokens {
        // println!("{:?}", t);
        match t {
            "do()" => state = State::Do,
            "don't()" => state = State::Dont,
            s => {
                if state == State::Dont {
                    continue;
                }
                // println!("{:?}", s);
                let cs = s.to_owned();
                // println!("{:?}", cs);
                let mut cs = cs.get(4..cs.len() - 1).unwrap().split(",");
                // println!("{:?}", cs);
                let n1: u32 = cs.next().unwrap().parse().unwrap();
                let n2: u32 = cs.next().unwrap().parse().unwrap();

                result += n1 * n2;
            }
        }
    }

    result
}

fn compute_tokens(filepath: &str) -> u32 {
    let input = &read_to_string(filepath).unwrap()[..];
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut tokens = vec![];
    for (_, [n1, n2]) in re.captures_iter(input).map(|c| c.extract()) {
        tokens.push((
            n1.to_owned().parse::<u32>().unwrap(),
            n2.to_owned().parse::<u32>().unwrap(),
        ));
    }

    let mut result: u32 = 0;
    for (n1, n2) in tokens {
        result += n1 * n2;
    }
    result
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => compute_tokens("./tests/day03/example.txt"),
        "actual" => compute_tokens("./tests/day03/actual.txt"),
        "example_v2" => compute_tokens_v2("./tests/day03/example.txt"),
        "actual_v2" => compute_tokens_v2("./tests/day03/actual.txt"),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 161);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 8 * 5);
    }

    #[test]
    fn test_actual() {
        assert!(main("actual") > 157725712);
    }
}
