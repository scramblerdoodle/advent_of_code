use crate::utils::{Board, Direction};
use std::{fmt, fs::read_to_string};

struct TrailMap {
    trail_map: Board<u8>,
    visited: Board<bool>,
    trailhead_count: u32,
}

impl TrailMap {
    fn new(trail_map: Vec<Vec<u8>>) -> Self {
        TrailMap {
            visited: Board::new(vec![vec![false; trail_map[0].len()]; trail_map.len()]),
            trail_map: Board::new(trail_map),
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

    fn is_valid_path(&self, curr_level: u8, next_pos: (usize, usize)) -> bool {
        self.trail_map[next_pos.0][next_pos.1] == curr_level + 1
            && !self.visited[next_pos.0][next_pos.1]
    }

    fn walk_trail(&mut self, pos: (usize, usize)) {
        self.visited[pos.0][pos.1] = true;

        if self.trail_map[pos.0][pos.1] == 9 {
            self.trailhead_count += 1;
            return;
        }

        for dir in Direction::ORTHOGONALS {
            if let Some(next_step) = self.trail_map.add_direction(&dir, pos) {
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

    fn is_valid_path_v2(&self, curr_level: u8, next_pos: (usize, usize)) -> bool {
        self.trail_map[next_pos.0][next_pos.1] == curr_level + 1
    }

    fn walk_trail_v2(&mut self, pos: (usize, usize)) {
        if self.trail_map[pos.0][pos.1] == 9 {
            self.trailhead_count += 1;
            return;
        }

        for dir in Direction::ORTHOGONALS {
            if let Some(next_step) = self.trail_map.add_direction(&dir, pos) {
                let curr_level = self
                    .trail_map
                    .get(pos.0)
                    .expect("Out of bounds")
                    .get(pos.1)
                    .expect("Out of bounds");
                if self.is_valid_path_v2(*curr_level, next_step) {
                    self.walk_trail_v2(next_step);
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
            self.trailhead_count, self.trail_map, self.visited
        )
    }
}

fn day10(mut trail: TrailMap) -> u32 {
    // println!("Starting trail: \n{}\n", trail);
    for i in 0..trail.trail_map.len() {
        for j in 0..trail.trail_map[i].len() {
            if trail.trail_map[i][j] == 0 {
                trail.reset_visited();
                // Find trailhead
                trail.walk_trail((i, j));
            }
        }
    }
    // println!("Final trail: \n{}\n", trail);
    trail.trailhead_count
}

fn day10_v2(mut trail: TrailMap) -> u32 {
    // println!("Starting trail: \n{}\n", trail);
    for i in 0..trail.trail_map.len() {
        for j in 0..trail.trail_map[i].len() {
            if trail.trail_map[i][j] == 0 {
                // Find trailhead
                trail.walk_trail_v2((i, j));
            }
        }
    }
    // println!("Final trail: \n{}\n", trail);
    trail.trailhead_count
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
        "example" => day10(parse_input("./tests/day10/example.txt")),
        "actual" => day10(parse_input("./tests/day10/actual.txt")),
        "example_v2" => day10_v2(parse_input("./tests/day10/example.txt")),
        "actual_v2" => day10_v2(parse_input("./tests/day10/actual.txt")),
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

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 81);
    }
}
