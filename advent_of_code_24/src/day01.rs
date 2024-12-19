use std::{collections::HashMap, fs::read_to_string};

fn parse_input(filepath: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        let mut nums = l.split("   ").map(|c| c.parse::<i32>().unwrap());
        list_one.push(nums.next().unwrap());
        list_two.push(nums.next().unwrap());
    });

    (list_one, list_two)
}

fn match_list(mut l1: Vec<i32>, mut l2: Vec<i32>) -> i32 {
    l1.sort();
    l2.sort();

    // For each element in list one and two, sum absolute differences
    (0..l1.len()).map(|i| (l1[i] - l2[i]).abs()).sum()
}

fn match_list_v2(l1: Vec<i32>, l2: Vec<i32>) -> i32 {
    let mut l2_counter: HashMap<i32, i32> = HashMap::new();

    // For each element in list one, add to list two counter map
    l2.iter()
        .for_each(|k| *l2_counter.entry(*k).or_default() += 1);

    // Return sum number * count
    l1.iter()
        .map(|n| *n * *l2_counter.entry(*n).or_default())
        .sum()
}

pub fn main(s: &str) -> i32 {
    match s {
        "example" => {
            let (l1, l2) = parse_input("./tests/day01/example.txt");
            match_list(l1, l2)
        }
        "actual" => {
            let (l1, l2) = parse_input("./tests/day01/actual.txt");
            match_list(l1, l2)
        }

        "example_v2" => {
            let (l1, l2) = parse_input("./tests/day01/example.txt");
            match_list_v2(l1, l2)
        }
        "actual_v2" => {
            let (l1, l2) = parse_input("./tests/day01/actual.txt");
            match_list_v2(l1, l2)
        }
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
