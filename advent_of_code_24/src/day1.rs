use std::{collections::HashMap, fs::read_to_string};

fn parse_input(filepath: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];

    for line in read_to_string(filepath).unwrap().lines() {
        let mut nums = line.split("   ").map(|c| c.parse::<i32>().unwrap());
        list_one.push(nums.next().unwrap());
        list_two.push(nums.next().unwrap());
    }
    (list_one, list_two)
}

fn match_list(filepath: &str) -> i32 {
    let (mut list_one, mut list_two) = parse_input(filepath);
    list_one.sort();
    list_two.sort();

    // For each element in list one and two, sum absolute differences
    (0..list_one.len())
        .map(|i| (list_one[i] - list_two[i]).abs())
        .sum()
}

fn match_list_v2(filepath: &str) -> i32 {
    let (list_one, list_two) = parse_input(filepath);

    // For each element in list one, add to list two counter map
    let mut map_two: HashMap<i32, i32> = HashMap::new();
    list_two
        .iter()
        .for_each(|k| *map_two.entry(*k).or_default() += 1);

    // Return sum number * count
    list_one
        .iter()
        .map(|n| *n * *map_two.entry(*n).or_default())
        .sum()
}

pub fn main(s: &str) -> i32 {
    match s {
        "example" => match_list("./tests/day1/example.txt"),
        "actual" => match_list("./tests/day1/actual.txt"),
        "actual_v2" => match_list_v2("./tests/day1/actual.txt"),
        "example_v2" => match_list_v2("./tests/day1/example.txt"),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 11);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 31);
    }
}
