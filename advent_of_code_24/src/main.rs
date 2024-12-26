pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
mod utils;

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
            1, 13
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
            1 => run_inputs(day01::main),
            2 => run_inputs(day02::main),
            3 => run_inputs(day03::main),
            4 => run_inputs(day04::main),
            5 => run_inputs(day05::main),
            6 => run_inputs(day06::main),
            7 => {
                println!("WARNING! This one takes a while. Not proud of this.");
                run_inputs(day07::main)
            }
            8 => run_inputs(day08::main),
            9 => run_inputs(day09::main),
            10 => run_inputs(day10::main),
            11 => run_inputs(day11::main),
            12 => run_inputs(day12::main),
            13 => run_inputs(day13::main),
            _ => todo!(),
        }
    }
    println!("Bye!");
}
