use std::fs::read_to_string;

enum State {
    A,
    B,
    Prize,
}

impl State {
    fn next(&mut self) {
        match self {
            Self::A => *self = Self::B,
            Self::B => *self = Self::Prize,
            Self::Prize => *self = Self::A,
        }
    }
}
struct LinearSystem {
    foo: Vec<String>,
}

fn day13(input: LinearSystem) -> u32 {
    let mut result: u32 = 0;
    for line in input.foo {
        result += line.parse::<u32>().expect("Unexpected input");
    }
    result
}

fn day13_v2(input: LinearSystem) -> u32 {
    let mut result: u32 = 0;
    for line in input.foo {
        result += line.parse::<u32>().expect("Unexpected input");
    }
    result
}

fn parse_input(filepath: &str) -> LinearSystem {
    let mut result: Vec<String> = vec![];
    let mut equations: Vec<[u32; 2]> = vec![];
    let mut state = State::A;
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        if l.eq("\n") {
            match state {
                State::A | State::B => {
                    // Parse Button A and B
                    // values = ["X+x", "Y+y"]
                    let mut values = l.split(": ").last().unwrap().split(", ");
                    let x = values.next().unwrap().split("+").last().unwrap();
                    let y = values.next().unwrap().split("+").last().unwrap();

                    equations.push([x.parse().unwrap(), y.parse().unwrap()]);

                    state.next();
                }
                State::Prize => {
                    // Parse Prize
                    // values = ["X=x", "Y=y"]
                    let mut values = l.split(": ").last().unwrap().split(", ");
                    let x_goal = values.next().unwrap().split("=").last().unwrap();
                    let y_goal = values.next().unwrap().split("=").last().unwrap();

                    equations.push([x_goal.parse().unwrap(), y_goal.parse().unwrap()]);

                    // TODO: Do Stuff with Equations, then empty them out

                    equations.clear();
                    state.next();
                }
            }
            result.push(l.to_string());
        }
    });

    LinearSystem { foo: result }
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day13(parse_input("./tests/day13/example.txt")),
        "actual" => day13(parse_input("./tests/day13/actual.txt")),
        "example_v2" => day13_v2(parse_input("./tests/day13/example.txt")),
        "actual_v2" => day13_v2(parse_input("./tests/day13/actual.txt")),
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
