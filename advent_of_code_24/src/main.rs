pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use std::time::Instant;
use std::{fmt, io::stdin};

fn run_inputs<T>(f: fn(&str) -> T)
where
    T: fmt::Display,
{
    let inputs = vec![
        ("Example", "example"),
        ("Example v2", "example_v2"),
        ("Actual", "actual"),
        ("Actual v2", "actual_v2"),
    ];
    for (name, path) in inputs {
        let now = Instant::now();

        {
            println!("{}: {}", name, f(path));
        }

        let elapsed = now.elapsed();
        println!("\tElapsed: {:.2?}", elapsed);
    }
    println!();
}

fn main() {
    loop {
        let mut input = String::new();
        println!(
            "Choose a Day from {} to {}; 0 exits and input defaults to 0.",
            1, 10
        );
        let input = match stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => panic!("How did you even do this"),
        }
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

        match input {
            0 => break,
            1 => run_inputs(day1::main),
            2 => run_inputs(day2::main),
            3 => run_inputs(day3::main),
            4 => run_inputs(day4::main),
            5 => run_inputs(day5::main),
            6 => run_inputs(day6::main),
            7 => {
                println!("WARNING! This one takes a while. Not proud of this.");
                run_inputs(day7::main)
            }
            8 => run_inputs(day8::main),
            9 => run_inputs(day9::main),
            10 => run_inputs(day10::main),
            _ => todo!(),
        }
    }
    println!("Bye!");
}
