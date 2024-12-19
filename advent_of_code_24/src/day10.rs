use std::{fmt, fs::read_to_string, num::ParseIntError};

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const DIRECTIONS: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    fn get_direction(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
        }
    }
}

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

    fn reset_visited(&mut self) {
        for i in 0..self.visited.len() {
            for j in 0..self.visited[i].len() {
                self.visited[i][j] = false;
            }
        }
    }

    fn add_direction(&self, dir: &Direction, pos: (usize, usize)) -> Option<(usize, usize)> {
        let step = dir.get_direction();
        let (next_i, next_j): (usize, usize);

        // Guards against out of bounds by going negative
        match pos.0.checked_add_signed(step.0) {
            Some(n) => next_i = n,
            None => return None,
        }

        // Guards against out of bounds by going negative
        match pos.1.checked_add_signed(step.1) {
            Some(n) => next_j = n,
            None => return None,
        }

        // Guards against going out of bounds by going positive
        if let Some(next_line) = self.trail_map.get(next_i) {
            // Guards against going out of bounds by going positive
            if let Some(_) = next_line.get(next_j) {
                return Some((next_i, next_j));
            }
        };
        None
    }

    fn is_valid_path(&self, curr_level: u8, next_pos: (usize, usize)) -> bool {
        if self.trail_map[next_pos.0][next_pos.1] == curr_level + 1
            && !self.visited[next_pos.0][next_pos.1]
        {
            true
        } else {
            false
        }
    }

    fn walk_trail(&mut self, pos: (usize, usize)) {
        self.visited[pos.0][pos.1] = true;

        if self.trail_map[pos.0][pos.1] == 9 {
            self.trailhead_count += 1;
            return;
        }

        for dir in Direction::DIRECTIONS {
            if let Some(next_step) = self.add_direction(&dir, pos) {
                let curr_level = self
                    .trail_map
                    .get(pos.0)
                    .expect("Out of bounds")
                    .get(pos.1)
                    .expect("Out of bounds");
                if self.is_valid_path(*curr_level, next_step) {
                    self.walk_trail(next_step);
                }
            };
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

fn day10(mut trail: TrailMap) -> Result<u32, ParseIntError> {
    println!("Starting trail: \n{}\n", trail);
    for i in 0..trail.trail_map.len() {
        for j in 0..trail.trail_map[i].len() {
            if trail.trail_map[i][j] == 0 {
                trail.reset_visited();
                // Find trailhead
                trail.walk_trail((i, j));
            }
        }
    }
    println!("Final trail: \n{}\n", trail);
    Ok(trail.trailhead_count as u32)
}

fn day10_v2(inputs: TrailMap) -> Result<u32, ParseIntError> {
    // println!("{}", inputs);
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
        assert_eq!(main("example"), 36);
    }

    // #[test]
    // fn test_example_v2() {
    //     assert_eq!(main("example"), 0);
    // }
}
