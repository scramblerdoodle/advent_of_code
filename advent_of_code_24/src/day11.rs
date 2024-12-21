use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug)]
struct Stone(u64);

impl Stone {
    fn change(&self) -> Self {
        if self.0 == 0 {
            Stone(1)
        } else {
            Stone(2024 * self.0)
        }
    }

    fn is_engraving_even_digits(&self) -> bool {
        self.0.to_string().len() % 2 == 0
    }

    fn split_stone(&self) -> (Self, Self) {
        let mut engraving = self.0.to_string();
        let new_stone = engraving.split_off(engraving.len() / 2);

        let stone1 = Stone(engraving.parse::<u64>().unwrap());
        let stone2 = Stone(new_stone.parse::<u64>().unwrap());

        (stone1, stone2)
    }
}

// struct StoneMap(HashMap<Stone, usize>);

struct Stones(VecDeque<Stone>);

impl Stones {
    fn new(stones: VecDeque<Stone>) -> Self {
        Stones(stones)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn push_front(&mut self, value: Stone) {
        self.0.push_front(value)
    }

    fn evolve_stones(self) -> Self {
        let mut next_stones = Self::new(VecDeque::new());

        for stone in self.0.iter() {
            if stone.is_engraving_even_digits() {
                let new_stones = stone.split_stone();
                next_stones.push_front(new_stones.0);
                next_stones.push_front(new_stones.1);
            } else {
                next_stones.push_front(stone.change());
            }
        }

        next_stones
    }
}

fn day11(mut stones: Stones) -> u32 {
    let blinks = 25;
    for _b in 0..blinks {
        stones = stones.evolve_stones();
    }

    // println!("{:?}", stones.0);
    stones.len() as u32
}

fn day11_v2(mut stones: Stones) -> u32 {
    // let blinks = 75;
    // for _b in 0..blinks {
    //     stones = stones.evolve_stones();
    // }
    //
    // // println!("{:?}", stones.0);
    // stones.len() as u32
    0
}

fn parse_input(filepath: &str) -> Stones {
    let stones = VecDeque::from_iter(
        read_to_string(filepath)
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| Stone(s.parse::<u64>().unwrap())),
    );

    Stones::new(stones)
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day11(parse_input("./tests/day11/example.txt")),
        "actual" => day11(parse_input("./tests/day11/actual.txt")),
        "example_v2" => day11_v2(parse_input("./tests/day11/example.txt")),
        "actual_v2" => day11_v2(parse_input("./tests/day11/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 55312);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 0);
    }
}
