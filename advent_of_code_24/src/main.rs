pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day3_attempt2;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use std::io::stdin;
fn main() {
    loop {
        let mut input = String::new();
        println!(
            "Choose a Day from {} to {}; 0 exits and default is 0.",
            1, 10
        );
        let input = match stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => panic!("Unexpected input."),
        }
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

        match input {
            0 => break,
            1 => {
                println!("Example: {}", day1::main("example"));
                println!("Example v2: {}", day1::main("example_v2"));
                println!("Actual: {}", day1::main("actual"));
                println!("Actual v2: {}", day1::main("actual_v2"));
            }
            2 => {
                println!("Example: {}", day2::main("example"));
                println!("Example v2: {}", day2::main("example_v2"));
                println!("Actual: {}", day2::main("actual"));
                println!("Actual v2: {}", day2::main("actual_v2"));
            }
            3 => {
                println!("Example: {}", day3_attempt2::main("example"));
                println!("Example v2: {}", day3_attempt2::main("example_v2"));
                println!("Actual: {}", day3_attempt2::main("actual"));
                println!("Actual v2: {}", day3_attempt2::main("actual_v2"));
            }
            4 => {
                println!("Example: {}", day4::main("example"));
                println!("Example v2: {}", day4::main("example_v2"));
                println!("Actual: {}", day4::main("actual"));
                println!("Actual v2: {}", day4::main("actual_v2"));
            }
            5 => {
                println!("Example: {}", day5::main("example"));
                println!("Example v2: {}", day5::main("example_v2"));
                println!("Actual: {}", day5::main("actual"));
                println!("Actual v2: {}", day5::main("actual_v2"));
            }
            6 => {
                println!("Example: {}", day6::main("example"));
                println!("Example v2: {}", day6::main("example_v2"));
                println!("Actual: {}", day6::main("actual"));
                println!("Actual v2: {}", day6::main("actual_v2"));
            }
            7 => {
                println!("Example: {}", day7::main("example"));
                println!("Example v2: {}", day7::main("example_v2"));
                println!("Actual: {}", day7::main("actual"));
                println!("Actual v2: {}", day7::main("actual_v2"));
            }
            8 => {
                println!("Example: {}", day8::main("example"));
                println!("Example v2: {}", day8::main("example_v2"));
                println!("Actual: {}", day8::main("actual"));
                println!("Actual v2: {}", day8::main("actual_v2"));
            }
            9 => {
                println!("Example: {}", day9::main("example"));
                println!("Example v2: {}", day9::main("example_v2"));
                println!("Actual: {}", day9::main("actual"));
                println!("Actual v2: {}", day9::main("actual_v2"));
            }
            10 => {
                println!("Example: {}", day10::main("example"));
                println!("Example v2: {}", day10::main("example_v2"));
                println!("Actual: {}", day10::main("actual"));
                println!("Actual v2: {}", day10::main("actual_v2"));
            }
            _ => todo!(),
        }
        println!();
    }
}
