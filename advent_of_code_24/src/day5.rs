use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Orders {
    rules: Vec<(String, String)>,
    production: Vec<Vec<String>>,
}

fn day5(orders: Orders) -> u32 {
    let mut result: u32 = 0;

    for prod_line in &orders.production {
        let mut seen: HashSet<String> = HashSet::new();
        let mut unseen: HashSet<String> = HashSet::from_iter(prod_line.clone());

        let mut valid = true;
        for p in prod_line {
            for (p1, p2) in &orders.rules {
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
}
