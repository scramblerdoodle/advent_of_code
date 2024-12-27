use std::{fs::read_to_string, str::Split};

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

struct Input {
    input: Vec<LinearSystem>,
}

#[derive(Copy, Clone, Debug)]
struct LinearSystem {
    button_a: Equation,
    button_b: Equation,
    prize: Equation,
}

impl LinearSystem {
    fn new() -> Self {
        LinearSystem {
            button_a: Equation::new(),
            button_b: Equation::new(),
            prize: Equation::new(),
        }
    }

    fn assign_equation(&mut self, state: &State, values: Split<'_, &str>) {
        match state {
            State::A => {
                self.button_a = Equation::from_str(values, "+");
            }
            State::B => {
                self.button_b = Equation::from_str(values, "+");
            }
            State::Prize => {
                self.prize = Equation::from_str(values, "=");
            }
        }
    }

    fn solve_or_default(self, default: i64) -> (i64, i64) {
        let m_numerator = self.button_a.x * self.prize.y - self.button_a.y * self.prize.x;
        let m_denominator = self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x;

        if m_numerator % m_denominator != 0 || m_numerator * m_denominator <= 0 {
            return (default, default);
        }
        let m = m_numerator / m_denominator;

        let n_numerator = self.prize.x - self.button_b.x * m;
        let n_denominator = self.button_a.x;

        if n_numerator % n_denominator != 0 || n_numerator * n_denominator <= 0 {
            return (default, default);
        }
        let n = n_numerator / n_denominator;
        (n, m)
    }
}

#[derive(Copy, Clone, Debug)]
struct Equation {
    x: i64,
    y: i64,
}

impl Equation {
    fn new() -> Self {
        Equation { x: 0, y: 0 }
    }
    fn from_str(values: Split<'_, &str>, sep: &str) -> Self {
        let equation = values
            .map(|s| s.split(sep).last().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let x = equation.get(0).expect("Index is out of bounds");
        let y = equation.get(1).expect("Index is out of bounds");

        Equation { x: *x, y: *y }
    }

    fn from_slice(equation: [i64; 2]) -> Self {
        let x = equation.get(0).expect("Index is out of bounds");
        let y = equation.get(1).expect("Index is out of bounds");

        Equation { x: *x, y: *y }
    }
}

fn day13(input: Input) -> i64 {
    let mut result: i64 = 0;
    for system in input.input {
        let (x, y) = system.solve_or_default(0);
        result += 3 * x + y;
    }
    result
}

fn day13_v2(input: Input) -> i64 {
    let mut result: i64 = 0;
    for mut system in input.input {
        system.prize = Equation::from_slice([
            system.prize.x + 10000000000000,
            system.prize.y + 10000000000000,
        ]);
        let (x, y) = system.solve_or_default(0);
        result += 3 * x + y;
    }
    result
}

fn parse_input(filepath: &str) -> Input {
    let mut result: Vec<LinearSystem> = vec![];
    let mut state = State::A;
    let mut linear_system = LinearSystem::new();
    for l in read_to_string(filepath).unwrap().lines() {
        if !l.eq("") {
            let values = l.split(": ").last().unwrap().split(", ");
            linear_system.assign_equation(&state, values);
            state.next();
        } else {
            result.push(linear_system);
        }
    }

    Input { input: result }
}

pub fn main(s: &str) -> i64 {
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
        assert_eq!(main("example"), 480);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 875318608908);
    }
}
