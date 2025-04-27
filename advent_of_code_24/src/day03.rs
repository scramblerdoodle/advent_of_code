use crate::utils::{get_test_file, FileNotFound, ACTUAL, EXAMPLE};
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

pub fn main(s: &str) -> Result<u32, FileNotFound> {
    match s {
        "example" => match get_test_file(EXAMPLE, "03") {
            Err(err) => Err(err),
            Ok(file) => Ok(compute_tokens(&file)),
        },
        "actual" => match get_test_file(ACTUAL, "03") {
            Err(err) => Err(err),
            Ok(file) => Ok(compute_tokens(&file)),
        },
        "example_v2" => match get_test_file(EXAMPLE, "03") {
            Err(err) => Err(err),
            Ok(file) => Ok(compute_tokens_v2(&file)),
        },
        "actual_v2" => match get_test_file(ACTUAL, "03") {
            Err(err) => Err(err),
            Ok(file) => Ok(compute_tokens_v2(&file)),
        },
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example").unwrap(), 161);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2").unwrap(), 8 * 5);
    }
}
