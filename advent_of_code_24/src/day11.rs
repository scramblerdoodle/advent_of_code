use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

#[derive(Debug, Hash, PartialEq, Eq)]
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

struct StoneVec(VecDeque<Stone>);

impl StoneVec {
    fn new(stones: VecDeque<Stone>) -> Self {
        StoneVec(stones)
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

#[derive(Debug)]
struct StoneMap(HashMap<Stone, usize>);

impl StoneMap {
    fn new(stones: HashMap<Stone, usize>) -> Self {
        StoneMap(stones)
    }

    fn from_vec(stone_vec: StoneVec) -> Self {
        let mut stone_map = HashMap::new();
        for stone in stone_vec.0 {
            stone_map.insert(stone, 1);
        }
        StoneMap(stone_map)
    }

    fn insert_or_add_count(&mut self, stone: Stone, count: usize) -> &mut usize {
        self.0
            .entry(stone)
            .and_modify(|n| *n += count)
            .or_insert(count)
    }

    fn evolve_stones(self) -> Self {
        let mut next_stones = Self::new(HashMap::new());
        for (stone, count) in self.0 {
            if stone.is_engraving_even_digits() {
                let new_stones = stone.split_stone();
                next_stones.insert_or_add_count(new_stones.0, count);
                next_stones.insert_or_add_count(new_stones.1, count);
            } else {
                next_stones.insert_or_add_count(stone.change(), count);
            }
        }
        // println!(
        //     "{}, {}",
        //     next_stones.count(),
        //     next_stones
        //         .0
        //         .iter()
        //         .map(|(stone, count)| format!("{}: {count}", stone.0 as u64))
        //         .collect::<Vec<String>>()
        //         .join(", ")
        // );
        // println!();
        next_stones
    }

    fn count(&self) -> u32 {
        self.0.values().sum::<usize>() as u32
    }
}

fn day11(mut stones: StoneVec) -> u32 {
    let blinks = 25;
    for _b in 0..blinks {
        stones = stones.evolve_stones();
    }

    // println!("{:?}", stones.0);
    stones.len() as u32
}

fn day11_v2(stones: StoneVec) -> u32 {
    let mut stone_map = StoneMap::from_vec(stones);
    // println!("{:?}", stone_map);
    // println!(
    //     "{}, {}",
    //     stone_map.count(),
    //     stone_map
    //         .0
    //         .iter()
    //         .map(|(stone, count)| format!("{}: {count}", stone.0 as u64))
    //         .collect::<Vec<String>>()
    //         .join(", ")
    // );
    // println!();
    let blinks = 75;
    for _b in 0..blinks {
        stone_map = stone_map.evolve_stones();
    }

    stone_map.count()
}

fn parse_input(filepath: &str) -> StoneVec {
    let stones = VecDeque::from_iter(
        read_to_string(filepath)
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| Stone(s.parse::<u64>().unwrap())),
    );

    StoneVec::new(stones)
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
        assert_eq!(main("example_v2"), 4003138674);
    }

    // #[test]
    // fn test_actual_v2() {
    //     assert_eq!(main("actual_v2"), 547791280);
    // }
}
