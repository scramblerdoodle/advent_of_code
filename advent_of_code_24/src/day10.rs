use std::{fmt, fs::read_to_string, num::ParseIntError};

struct TrailMap {
    trail_map: Vec<Vec<u8>>,
    visited: Vec<Vec<bool>>,
    trailhead_count: usize,
}

impl TrailMap {
    fn new(trail_map: Vec<Vec<u8>>) -> Self {
        TrailMap {
            visited: vec![vec![false; trail_map[0].len()]; trail_map.len()],
            trail_map,
            trailhead_count: 0,
        }
    }
}

impl fmt::Display for TrailMap {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            concat!("Count: {}\n\n", "Map:\n\t{}\n\n", "Visited:\n\t{}"),
            self.trailhead_count,
            self.trail_map
                .iter()
                .map(|v| v
                    .iter()
                    .map(|&d| char::from_digit(d as u32, 10).unwrap())
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n\t"),
            self.visited
                .iter()
                .map(|v| v
                    .iter()
                    .map(|&boolean| match boolean {
                        true => "1",
                        false => "0",
                    })
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n\t"),
        )
    }
}

fn day10(inputs: TrailMap) -> Result<u32, ParseIntError> {
    println!("{}", inputs);
    Ok(1)
}

fn day10_v2(inputs: TrailMap) -> Result<u32, ParseIntError> {
    println!("{}", inputs);
    Ok(0)
}

fn parse_input(filepath: &str) -> TrailMap {
    let mut trail_map: Vec<Vec<u8>> = vec![];

    read_to_string(filepath).unwrap().lines().for_each(|l| {
        trail_map.push(l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
    });

    TrailMap::new(trail_map)
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => {
            day10(parse_input("./tests/day10/example.txt")).expect("Function raised exception")
        }
        "actual" => {
            day10(parse_input("./tests/day10/actual.txt")).expect("Function raised exception")
        }
        "example_v2" => {
            day10_v2(parse_input("./tests/day10/example.txt")).expect("Function raised exception")
        }
        "actual_v2" => {
            day10_v2(parse_input("./tests/day10/actual.txt")).expect("Function raised exception")
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 0);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example"), 0);
    }
}
