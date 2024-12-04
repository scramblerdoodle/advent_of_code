use std::fs::read_to_string;

enum State {
    UNSAFE,
    SAFE,
}

fn reactor_safety(report: Vec<Vec<i32>>) -> i32 {
    let mut count_safe: i32 = 0;
    let mut report = report.iter();
    // println!("{:?}", report);

    loop {
        count_safe += match report.next() {
            None => return count_safe,
            Some(r) => {
                // println!("{:?}", r);
                let mut prev = r[0];
                let mut result = State::SAFE;

                let is_increasing = r[1] > r[0];

                for n in &r[1..] {
                    if prev == *n {
                        // println!("Both values are equal!");
                        // println!("prev: {prev}, n: {n}");
                        result = State::UNSAFE;
                        break;
                    } else if *n < prev && is_increasing {
                        // println!("Decreasing but should be increasing!");
                        // println!("prev: {prev}, n: {n}");
                        result = State::UNSAFE;
                        break;
                    } else if *n > prev && !is_increasing {
                        // println!("Increasing but should be decreasing!");
                        // println!("prev: {prev}, n: {n}");
                        result = State::UNSAFE;
                        break;
                    // if | prev - n | <= 3 : it's good
                    //      i.e. -3 <= prev - n <= 3
                    //      negation: prev - n < 3 or prev - n > 3
                    //      i.e. |prev - n| > 3
                    } else if (prev - *n).abs() > 3 {
                        // println!("Diff greater than 3!");
                        // println!("prev: {prev}, n: {n}");
                        result = State::UNSAFE;
                        break;
                    }

                    prev = *n;
                }

                result as i32
            }
        };
    }
}

fn validate_state(prev: i32, n: i32, is_increasing: bool) -> State {
    if prev == n {
        // println!("Both values are equal!");
        // println!("prev: {prev}, n: {n}");
        return State::UNSAFE;
    } else if n < prev && is_increasing {
        // println!("Decreasing but should be increasing!");
        // println!("prev: {prev}, n: {n}");
        return State::UNSAFE;
    } else if n > prev && !is_increasing {
        // println!("Increasing but should be decreasing!");
        // println!("prev: {prev}, n: {n}");
        return State::UNSAFE;
    } else if (prev - n).abs() > 3 {
        // println!("Diff greater than 3!");
        // println!("prev: {prev}, n: {n}");
        return State::UNSAFE;
    }

    State::SAFE
}

fn reactor_safety_v2(report: Vec<Vec<i32>>) -> i32 {
    /*  Edge cases not considered in this approach:
     *       [10, 1, 2, 3, 4]
     *           Should be SAFE because if we skip 10, everything else is valid
     *
     *       [5, 2, 6, 7, 8]
     *           Should be SAFE because it's actually increasing when we skip 2
     *
     */
    let mut count_safe: i32 = 0;
    // println!("{:?}", report);

    let mut i: usize = 0;
    loop {
        count_safe += match report.get(i) {
            None => return count_safe,
            Some(r) => {
                // println!("{:?}", r);
                let mut r = r.iter().peekable();
                let mut prev = *r.next().unwrap();
                let mut result: State = State::UNSAFE;

                let is_increasing = **r.peek().unwrap() > prev;

                while let Some(n) = r.next() {
                    result = validate_state(prev, *n, is_increasing);

                    match validate_state(prev, *n, is_increasing) {
                        State::SAFE => prev = *n,
                        State::UNSAFE => {
                            if let Some(skip) = r.peek() {
                                match validate_state(prev, **skip, is_increasing) {
                                    State::SAFE => prev = *n,
                                    State::UNSAFE => break,
                                }
                            };
                        }
                    }
                }

                result as i32
            }
        };
        i += 1;
    }
}

fn parse_input(filepath: &str) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        let nums: Vec<i32> = l.split(" ").map(|c| c.parse::<i32>().unwrap()).collect();
        result.push(nums);
    });

    result
}

pub fn main(s: &str) -> i32 {
    match s {
        "example" => {
            let report = parse_input("./tests/day2/example.txt");
            reactor_safety(report)
        }
        "actual" => {
            let report = parse_input("./tests/day2/actual.txt");
            reactor_safety(report)
        }

        "example_v2" => {
            let report = parse_input("./tests/day2/example.txt");
            reactor_safety_v2(report)
        }
        "actual_v2" => {
            let report = parse_input("./tests/day2/actual.txt");
            reactor_safety_v2(report)
            // part_2(read_to_string("./tests/day2/actual.txt").unwrap())
            //     .try_into()
            //     .unwrap()
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 2);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 4);
    }

    // #[test]
    // fn test_actual_v2() {
    //     assert_eq!(main("actual_v2"), 601);
    // }
}
