use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    num::ParseIntError,
};

use combinatorial::Combinations;

#[derive(Debug)]
struct Input {
    size: (usize, usize),
    antenna_map: HashMap<char, Vec<(isize, isize)>>,
}

impl Input {
    fn new() -> Self {
        Self {
            size: (0, 0),
            antenna_map: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct Antinodes {
    map_size: (usize, usize),
    antinode_locations: HashSet<(usize, usize)>,
}

impl Antinodes {
    fn new(input: &Input) -> Self {
        Self {
            map_size: input.size.clone(),
            antinode_locations: HashSet::new(),
        }
    }

    fn is_within_map(self: &Self, pos: (isize, isize)) -> bool {
        if pos.0 < 0 || pos.0 as usize >= self.map_size.0 {
            return false;
        } else if pos.1 < 0 || pos.1 as usize >= self.map_size.1 {
            return false;
        } else {
            return true;
        }
    }

    fn add_location_if_within(&mut self, pos: (isize, isize)) -> bool {
        if self.is_within_map(pos) {
            self.antinode_locations
                .insert((pos.0 as usize, pos.1 as usize));
            return true;
        } else {
            return false;
        }
    }

    fn count(self) -> u32 {
        self.antinode_locations.len() as u32
    }

    #[allow(dead_code)]
    fn print_map(&self, input: &Input) -> () {
        let mut map: Vec<Vec<char>> = vec![vec!['.'; self.map_size.1]; self.map_size.0];
        for (k, v) in input.antenna_map.iter() {
            for (i, j) in v {
                map[*i as usize][*j as usize] = *k;
            }
        }
        for (i, j) in self.antinode_locations.iter() {
            map[*i][*j] = '#';
        }

        for l in map {
            println!("{}", l.into_iter().collect::<String>());
        }
    }
}

trait IsAntenna {
    fn is_antenna(self) -> bool;
}

impl IsAntenna for char {
    // Char is an antenna if lowercase, uppercase or digit
    fn is_antenna(self) -> bool {
        self.is_ascii_alphanumeric()
    }
}

fn day8(inputs: Input) -> Result<u32, ParseIntError> {
    let mut antinodes = Antinodes::new(&inputs);
    inputs.antenna_map.values().for_each(|v| {
        let combs = Combinations::of_size(v, 2);
        // Antenna positions for each antenna type
        // Need a two-by-two combination to calculate the diametrical distances
        combs.for_each(|v| {
            let v0 = v.get(0).unwrap();
            let v1 = v.get(1).unwrap();

            let dist0 = v1.0 - v0.0;
            let dist1 = v1.1 - v0.1;

            let pos1 = (v0.0 - dist0, v0.1 - dist1);
            let pos2 = (v1.0 + dist0, v1.1 + dist1);

            antinodes.add_location_if_within(pos1);
            antinodes.add_location_if_within(pos2);
        });
    });
    // println!("{:?}", antinodes);
    // println!("{:?}", inputs);
    Ok(antinodes.count())
}

fn day8_v2(inputs: Input) -> Result<u32, ParseIntError> {
    let mut antinodes = Antinodes::new(&inputs);
    inputs.antenna_map.values().for_each(|v| {
        let combs = Combinations::of_size(v, 2);
        // Antenna positions for each antenna type
        // Need a two-by-two combination to calculate the diametrical distances
        combs.for_each(|v| {
            let v0 = v.get(0).unwrap();
            let v1 = v.get(1).unwrap();

            let dist0 = v1.0 - v0.0;
            let dist1 = v1.1 - v0.1;

            let mut pos1 = (v0.0, v0.1);

            while antinodes.is_within_map(pos1) {
                antinodes
                    .antinode_locations
                    .insert((pos1.0 as usize, pos1.1 as usize));
                pos1.0 -= dist0;
                pos1.1 -= dist1;
            }

            let mut pos2 = (v1.0, v1.1);
            while antinodes.is_within_map(pos2) {
                antinodes
                    .antinode_locations
                    .insert((pos2.0 as usize, pos2.1 as usize));
                pos2.0 += dist0;
                pos2.1 += dist1;
            }
        });
    });
    // println!("{:?}", antinodes);
    // println!("{:?}", inputs);

    // antinodes.print_map(&inputs);
    Ok(antinodes.count())
}

fn parse_input(filepath: &str) -> Input {
    let mut result = Input::new();
    let input = read_to_string(filepath).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let n = lines.len();
    let m = lines[0].len();
    result.size = (n, m);
    for i in 0..n {
        let chars: Vec<char> = lines[i].chars().collect();

        for j in 0..m {
            let c = chars[j];
            if c.is_antenna() {
                result
                    .antenna_map
                    .entry(c)
                    .and_modify(|v| v.push((i as isize, j as isize)))
                    .or_insert(vec![(i as isize, j as isize)]);
            }
        }
    }

    result
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day8(parse_input("./tests/day08/example.txt")).unwrap(),
        "actual" => day8(parse_input("./tests/day08/actual.txt")).unwrap(),
        "example_v2" => day8_v2(parse_input("./tests/day08/example.txt")).unwrap(),
        "actual_v2" => day8_v2(parse_input("./tests/day08/actual.txt")).unwrap(),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 14);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 34);
    }
}
