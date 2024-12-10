use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Orders {
    rules: Vec<(String, String)>,
    production: Vec<Vec<String>>,
}

impl Orders {
    fn filter_rules<'a>(&'a self, order_line: HashSet<String>) -> Vec<&'a (String, String)> {
        self.rules
            .iter()
            .filter(|(p1, p2)| order_line.contains(p1) && order_line.contains(p2))
            .collect()
    }

    fn sort_rules(a: &String, b: &String, rules: &Vec<&(String, String)>) -> Ordering {
        for (p1, p2) in rules {
            if a == p1 && b == p2 {
                return Ordering::Less;
            }
            if b == p1 && a == p2 {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

fn day5(orders: Orders) -> u32 {
    let mut result: u32 = 0;

    for prod_line in &orders.production {
        let mut seen: HashSet<String> = HashSet::new();
        let order_line: HashSet<String> = HashSet::from_iter(prod_line.clone());
        let mut unseen = order_line.clone();

        let mut valid = true;

        let filtered_rules = orders.filter_rules(order_line);

        for p in prod_line {
            for (p1, p2) in &filtered_rules {
                // Check rules
                if p == p2 {
                    if unseen.contains(p1) {
                        valid = false;
                    }
                }
            }
            seen.insert(p.clone());
            unseen.remove(p);
        }
        if !valid {
            continue;
        }

        let middle_page_index = (prod_line.len() - 1) / 2;
        result += prod_line[middle_page_index].parse::<u32>().unwrap();
    }

    result
}

fn day5_v2(orders: Orders) -> u32 {
    let mut result: u32 = 0;

    for prod_line in &orders.production {
        let mut seen: HashSet<String> = HashSet::new();
        let mut unseen: HashSet<String> = HashSet::from_iter(prod_line.clone());
        let filtered_rules = orders.filter_rules(unseen.clone());

        let mut valid = true;
        for p in prod_line {
            for (p1, p2) in &filtered_rules {
                // Check rules
                if p == p2 {
                    if unseen.contains(p1) {
                        valid = false;
                        break;
                    }
                }
            }
            if !valid {
                break;
            }
            seen.insert(p.clone());
            unseen.remove(p);
        }

        if !valid {
            // println!("prod_line        : {:?}", prod_line);
            // println!("order_line       : {:?}", order_line);
            // println!("filtered_rules   : {:?}", filtered_rules);

            let mut ordered_prod_line = prod_line.clone();
            ordered_prod_line.sort_by(|a, b| Orders::sort_rules(a, b, &filtered_rules));
            // println!("ordered_prod_line: {:?}", ordered_prod_line);
            // println!();

            let middle_page_index = (ordered_prod_line.len() - 1) / 2;
            result += ordered_prod_line[middle_page_index].parse::<u32>().unwrap();
        }
    }

    result
}

fn parse_text(file_path: &str) -> Orders {
    let content = read_to_string(file_path).unwrap();
    let mut lines = content.lines();

    let mut order_rules: Vec<(String, String)> = vec![];
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        let mut l = line.split("|");

        let s1 = l.next().unwrap().to_string();
        let s2 = l.next().unwrap().to_string();

        order_rules.push((s1, s2));
    }

    let mut production_order: Vec<Vec<String>> = vec![];
    while let Some(line) = lines.next() {
        production_order.push(line.split(",").map(|s| s.to_string()).collect());
    }

    Orders {
        rules: order_rules,
        production: production_order,
    }
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day5(parse_text("./tests/day5/example.txt")),
        "actual" => day5(parse_text("./tests/day5/actual.txt")),
        "example_v2" => day5_v2(parse_text("./tests/day5/example.txt")),
        "actual_v2" => day5_v2(parse_text("./tests/day5/actual.txt")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 143);
    }
    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 123);
    }
}
