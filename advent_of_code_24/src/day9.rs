use std::fmt;
use std::{fs::read_to_string, num::ParseIntError};

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    FILE,
    FREE,
}

impl State {
    fn change_state(&mut self) -> Self {
        match self {
            State::FILE => State::FREE,
            State::FREE => State::FILE,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Block {
    id: Option<usize>,
    state: State,
    size: usize,
}

impl Block {
    fn to_string(&self) -> String {
        match self.state {
            State::FREE => (0..self.size).map(|_| ".").collect(),
            State::FILE => (0..self.size)
                .map(|_| self.id.unwrap().to_string())
                .collect(),
        }
    }

    fn is_free(&self) -> bool {
        match self.state {
            State::FREE => true,
            State::FILE => false,
        }
    }
}

#[derive(Debug)]
struct Input {
    file_blocks: Vec<Block>,
}

impl Input {
    fn new() -> Self {
        Self {
            file_blocks: Vec::new(),
        }
    }

    fn push_block(&mut self, b: Block) {
        self.file_blocks.push(b);
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.file_blocks.to_string())
    }
}

trait SliceExt {
    fn trim_r(&self) -> &Self;
}

impl SliceExt for [Block] {
    fn trim_r(&self) -> &Self {
        if let Some(last) = self.iter().rposition(|c| c.state != State::FREE) {
            &self[0..last + 1]
        } else {
            unreachable!();
        }
    }
}

trait ToString {
    fn to_string(&self) -> String;
}
impl ToString for Vec<Block> {
    fn to_string(&self) -> String {
        let s = self.iter().map(|b| b.to_string());
        String::from_iter(s)
    }
}

fn day9(input: Input) -> Result<u64, ParseIntError> {
    // 0...11..2222
    // becomes
    //  Block{id:0, size:1},
    //  Block{id:None, size:3},
    //  Block{id:1, size:2},
    //  Block{id:None, size:2},
    //  Block{id:2, size:4}
    // which in turn will become
    //
    // 0222112.....
    //  Block{id:0, size:1},
    //  Block{id:2, size:3},
    //  Block{id:1, size:2},
    //  Block{id:2, size:1},
    //  Block{id:None, size:5}

    // 0...11..22
    // becomes
    //  Block{id:0, size:1},
    //  Block{id:None, size:3},
    //  Block{id:1, size:2},
    //  Block{id:None, size:2},
    //  Block{id:2, size:2}
    // which in turn will become
    //
    // 02211.....
    //  Block{id:0, size:1},
    //  Block{id:2, size:2},
    //  Block{id:1, size:2},
    //  Block{id:None, size:5}
    let mut blocks: Vec<Block> = input.file_blocks.trim_r().to_vec();

    let mut j: usize = blocks.len() - 1;

    for mut i in 0..blocks.len() {
        // println!("i: {i}, j: {j}");
        // println!("{:?}", blocks.to_string());

        if blocks[i].is_free() {
            if blocks[i].size < blocks[j].size {
                blocks[i].id = blocks[j].id;
                blocks[i].state = State::FILE;
                blocks[j].size -= blocks[i].size;
            } else if blocks[i].size > blocks[j].size {
                blocks[i].size -= blocks[j].size;
                let block = blocks[j];
                blocks[j].state = State::FREE;
                blocks[j].id = None;
                blocks.insert(i, block);
                i += 1;
            } else {
                let block = blocks[j];
                blocks[j] = blocks[i];
                blocks[i] = block;
            }
            while blocks[j].is_free() {
                j -= 1;
            }
        }

        if i >= j {
            break;
        }
    }

    // println!("{:?}", blocks.to_string());

    let mut result: u64 = 0;
    let mut pos: usize = 0;
    for b in blocks.trim_r() {
        for _ in 0..b.size {
            result += (pos as u64) * (b.id.unwrap() as u64);
            pos += 1;
        }
    }
    Ok(result)
}

fn day9_v2(input: Input) -> Result<u64, ParseIntError> {
    let mut blocks: Vec<Block> = input.file_blocks.trim_r().to_vec();

    let mut j = blocks.len() - 1;
    while j > 0 {
        while blocks[j].is_free() {
            j -= 1;
        }

        for i in 0..j {
            if blocks[i].is_free() {
                if blocks[i].size > blocks[j].size {
                    blocks[i].size -= blocks[j].size;
                    let block = blocks[j];
                    blocks[j].state = State::FREE;
                    blocks[j].id = None;
                    blocks.insert(i, block);
                    j += 1;
                    break;
                } else if blocks[i].size == blocks[j].size {
                    let block = blocks[j];
                    blocks[j] = blocks[i];
                    blocks[i] = block;
                    break;
                }
            }
        }

        j -= 1;
    }

    // println!("{:?}", blocks.to_string());

    let mut result: u64 = 0;
    let mut pos: usize = 0;
    for b in blocks {
        match b.state {
            State::FREE => {
                pos += b.size;
            }
            State::FILE => {
                for _ in 0..b.size {
                    result += (pos as u64) * (b.id.unwrap() as u64);
                    pos += 1;
                }
            }
        }
    }
    Ok(result)
}

fn parse_input(filepath: &str) -> Input {
    let mut state: State = State::FILE;
    let mut result: Input = Input::new();
    let mut file_id: usize = 0;
    read_to_string(filepath)
        .unwrap()
        .trim()
        .chars()
        .for_each(|c| {
            let d = c.to_digit(10).unwrap();
            match state {
                State::FILE => {
                    result.push_block(Block {
                        id: Some(file_id),
                        size: d as usize,
                        state: State::FILE,
                    });
                    file_id += 1;
                }
                State::FREE => result.push_block(Block {
                    id: None,
                    size: d as usize,
                    state: State::FREE,
                }),
            }
            state = state.change_state();
        });

    // println!("{:?}", result);
    result
}

pub fn main(s: &str) -> u64 {
    let result = match s {
        "example" => day9(parse_input("./tests/day9/example.txt")),
        "actual" => day9(parse_input("./tests/day9/actual.txt")),
        "example_v2" => day9_v2(parse_input("./tests/day9/example.txt")),
        "actual_v2" => day9_v2(parse_input("./tests/day9/actual.txt")),
        _ => todo!(),
    };
    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 1928);
    }

    #[test]
    fn test_input() {
        assert_eq!(
            parse_input("./tests/day9/example.txt")
                .file_blocks
                .to_string(),
            "00...111...2...333.44.5555.6666.777.888899".to_string()
        );
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 2858);
    }
}
