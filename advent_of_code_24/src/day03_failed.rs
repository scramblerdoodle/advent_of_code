use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum Token {
    LPar,
    RPar,
    Mul,
    Comma,
    Number(u32),
}

#[derive(Debug)]
struct TokenBuffer {
    tokens: Vec<Token>,
}

impl TokenBuffer {
    fn add_if_valid_or_empty(&mut self, t: Token) -> () {
        let last_t = self.last();

        match last_t {
            None => {
                if t == Token::Mul {
                    self.tokens.push(t);
                }
            }
            Some(Token::Mul) => {
                if t == Token::LPar {
                    self.tokens.push(t);
                } else {
                    self.tokens.clear();
                }
            }
            Some(Token::LPar) => {
                if matches!(t, Token::Number { .. }) {
                    self.tokens.push(t);
                } else {
                    self.tokens.clear();
                }
            }
            Some(Token::Number(_n)) => {
                if t == Token::RPar || matches!(t, Token::Number { .. }) || t == Token::Comma {
                    self.tokens.push(t);
                } else {
                    self.tokens.clear();
                }
            }
            Some(Token::Comma) => {
                if matches!(t, Token::Number { .. }) {
                    self.tokens.push(t);
                } else {
                    self.tokens.clear();
                }
            }
            Some(Token::RPar) => {
                self.tokens.push(t);
                if !self.is_valid() {
                    self.tokens.clear();
                }
            }
        }
    }

    fn is_valid(&self) -> bool {
        let mut tokens = self.tokens.iter();

        let t = tokens.next();
        if t.is_none_or(|t| t != &Token::Mul) {
            return false;
        }

        let t = tokens.next();
        if t.is_none_or(|t| t != &Token::LPar) {
            return false;
        }

        loop {
            let t = tokens.next();
            if t.is_some_and(|t| matches!(t, &Token::Number { .. })) {
                continue;
            } else if t.is_some_and(|t| t == &Token::Comma) {
                break;
            } else {
                return false;
            }
        }

        loop {
            let t = tokens.next();
            if t.is_some_and(|t| matches!(t, &Token::Number { .. })) {
                continue;
            } else if t.is_some_and(|t| t == &Token::RPar) {
                return true;
            } else {
                return false;
            }
        }
    }

    fn compute(&mut self) -> u32 {
        println!("{:?}", self.tokens);
        let mut tokens = self.tokens.iter();

        let mut n1: String = String::new();
        while let Some(t) = tokens.next() {
            match t {
                Token::Number(n) => {
                    n1.push(char::from_digit(*n, 10).unwrap());
                }
                Token::Comma => break,
                _ => continue,
            }
        }

        let mut n2: String = String::new();
        while let Some(t) = tokens.next() {
            match t {
                Token::Number(n) => {
                    n2.push(char::from_digit(*n, 10).unwrap());
                }
                Token::RPar => break,
                _ => continue,
            }
        }

        let n1 = match n1.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                println!("n1: {}", n1);
                panic!("{e}");
            }
        };
        let n2 = match n2.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                println!("n1: {}", n1);
                panic!("{e}");
            }
        };

        self.tokens.clear();
        n1 * n2
    }

    fn last(&self) -> Option<&Token> {
        match self.tokens.last() {
            Some(t) => Some(t),
            None => None,
        }
    }
}

fn compute_tokens(tokens: Vec<Token>) -> u32 {
    let mut token_buffer: TokenBuffer = TokenBuffer { tokens: vec![] };
    let mut result: u32 = 0;

    for t in tokens {
        token_buffer.add_if_valid_or_empty(t);
        if token_buffer.is_valid() {
            result += token_buffer.compute();
        }
    }

    result
}

fn parse_input(filepath: &str) -> Vec<Token> {
    let input = &read_to_string(filepath).unwrap()[..];

    let mut tokens: Vec<Token> = vec![];
    let mut i: usize = 0;

    let cs: Vec<char> = input.chars().collect();

    while i < input.len() {
        let c = cs.get(i).unwrap();

        match c {
            '(' => tokens.push(Token::LPar),
            ')' => tokens.push(Token::RPar),
            'm' => {
                let next_c = cs.get(i + 1);
                let next_next_c = cs.get(i + 2);

                if next_c.is_none() || next_next_c.is_none() {
                    break;
                }

                if *next_c.unwrap() != 'u' || *next_next_c.unwrap() != 'l' {
                    ()
                }

                i += 2;

                tokens.push(Token::Mul)
            }
            '0'..='9' => tokens.push(Token::Number(c.to_digit(10).unwrap())),
            ',' => tokens.push(Token::Comma),
            _ => (),
        };

        i += 1;
    }

    // println!("{:?}", tokens);

    tokens
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => {
            let tokens = parse_input("./tests/day03/example.txt");
            compute_tokens(tokens)
        }
        "actual" => {
            let tokens = parse_input("./tests/day03/actual.txt");
            compute_tokens(tokens)
        }

        "example_v2" => {
            // let input = parse_input("./tests/day03/example.txt");
            // lexer_v2(&input[..])
            0
        }
        "actual_v2" => {
            // let input = parse_input("./tests/day03/actual.txt");
            // lexer_v2(&input[..])
            0
        }
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
        assert_eq!(main("example_v2"), 0);
    }
    //
    // #[test]
    // fn test_actual() {
    //     assert!(main("actual") > 157725712);
    // }
}
