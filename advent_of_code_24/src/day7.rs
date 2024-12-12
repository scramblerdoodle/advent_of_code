use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Operation {
    SUM,
    MULT,
    CONCAT,
}

#[derive(Clone, Debug)]
struct Operators {
    val: u32,
    cap: u32,
}

trait Binary {
    fn as_binary(&self) -> String
    where
        Self: std::fmt::Binary,
    {
        format!("{self:b}")
    }
}

impl Binary for u32 {}

trait Trinary {
    fn as_trinary(self) -> String;
}

impl Trinary for u32 {
    fn as_trinary(self) -> String {
        let mut result = vec![];
        let radix = 3;
        let mut x = self;

        loop {
            let m = x % radix;
            x = x / radix;

            result.push(std::char::from_digit(m, radix).unwrap());
            if x == 0 {
                break;
            }
        }
        result.into_iter().rev().collect()
    }
}

trait Concat {
    fn concat(self, a: Self) -> Self;
}

impl Concat for u64 {
    fn concat(self, a: u64) -> Self {
        let mut s = self.to_string();
        let s2 = a.to_string();
        s.push_str(&s2);
        s.parse().unwrap()
    }
}

impl Operators {
    fn as_operations(self: &Self) -> Vec<Operation> {
        let padding = self.cap.as_binary().len() - self.val.as_binary().len();
        let mut v: Vec<Operation> = vec![Operation::SUM; padding];

        for op in self.val.as_binary().chars() {
            match op {
                '0' => v.push(Operation::SUM),
                '1' => v.push(Operation::MULT),
                _ => panic!("How the fuck did this happen"),
            };
        }
        v
    }

    fn as_operations_v2(self: &Self) -> Vec<Operation> {
        let padding = self.cap.as_trinary().len() - self.val.as_trinary().len();
        let mut v: Vec<Operation> = vec![Operation::SUM; padding];

        for op in self.val.as_trinary().chars() {
            match op {
                '0' => v.push(Operation::SUM),
                '1' => v.push(Operation::MULT),
                '2' => v.push(Operation::CONCAT),
                _ => panic!("How the fuck did this happen"),
            };
        }
        v
    }
}

#[derive(Debug)]
struct Equation {
    total: u64,
    values: Vec<u64>,
    operators: Operators,
}

impl Equation {
    fn evaluate(self: &Self) -> u64 {
        let mut result = self.values[0];
        let operations = self.operators.as_operations();

        for i in 0..operations.len() {
            match operations.get(i) {
                Some(Operation::SUM) => {
                    result += self.values[i + 1];
                    if result > self.total {
                        break;
                    }
                }
                Some(Operation::MULT) => {
                    result *= self.values[i + 1];
                    if result > self.total {
                        break;
                    }
                }
                _ => (),
            };
        }
        result
    }

    fn evaluate_v2(self: &Self) -> u64 {
        let mut result = self.values[0];
        let operations = self.operators.as_operations_v2();

        for i in 0..operations.len() {
            match operations.get(i) {
                None => (),
                Some(Operation::SUM) => {
                    result += self.values[i + 1];
                    if result > self.total {
                        break;
                    }
                }
                Some(Operation::MULT) => {
                    result *= self.values[i + 1];
                    if result > self.total {
                        break;
                    }
                }
                Some(Operation::CONCAT) => {
                    result = result.concat(self.values[i + 1]);
                    if result > self.total {
                        break;
                    }
                }
            };
        }
        result
    }
}

fn day7(input: Vec<Equation>) -> u64 {
    let mut result = 0;
    for mut equation in input {
        loop {
            if equation.evaluate() == equation.total {
                result += equation.total;
                break;
            } else if equation.operators.val >= equation.operators.cap {
                break;
            } else {
                equation.operators.val += 1;
            };
        }
    }
    result
}

fn day7_v2(input: Vec<Equation>) -> u64 {
    let mut result = 0;
    for mut equation in input {
        loop {
            if equation.evaluate_v2() == equation.total {
                result += equation.total;
                break;
            } else if equation.operators.val >= equation.operators.cap {
                break;
            } else {
                equation.operators.val += 1;
            };
        }
    }
    result
}

fn parse_input(file_path: &str) -> Vec<Equation> {
    let content = read_to_string(file_path).unwrap();
    let lines = content.lines();
    let mut input: Vec<Equation> = vec![];

    for l in lines {
        let mut split = l.split(": ");
        let total = split.next().unwrap().parse().unwrap();
        let mut remainder = split.next().unwrap().split(" ");
        let mut values = vec![];
        while let Some(s) = remainder.next() {
            values.push(s.parse().unwrap());
        }

        let operators = Operators {
            val: 0,
            cap: (2 as u32).pow(values.len() as u32 - 1) - 1,
        };

        input.push(Equation {
            total,
            values,
            operators,
        });
    }

    input
}

fn parse_input_v2(file_path: &str) -> Vec<Equation> {
    let content = read_to_string(file_path).unwrap();
    let lines = content.lines();
    let mut input: Vec<Equation> = vec![];

    for l in lines {
        let mut split = l.split(": ");
        let total = split.next().unwrap().parse().unwrap();
        let mut remainder = split.next().unwrap().split(" ");
        let mut values = vec![];
        while let Some(s) = remainder.next() {
            values.push(s.parse().unwrap());
        }

        let operators = Operators {
            val: 0,
            cap: (3 as u32).pow(values.len() as u32 - 1) - 1,
        };

        input.push(Equation {
            total,
            values,
            operators,
        });
    }

    input
}

pub fn main(s: &str) -> u64 {
    match s {
        "example" => day7(parse_input("./tests/day7/example.txt")),
        "actual" => day7(parse_input("./tests/day7/actual.txt")),
        "example_v2" => day7_v2(parse_input_v2("./tests/day7/example.txt")),
        "actual_v2" => day7_v2(parse_input_v2("./tests/day7/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 3749);
    }
    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 11387);
    }

    #[test]
    fn test_concat() {
        let a: u64 = 17;
        assert_eq!(a.concat(8), 178);
    }
}
