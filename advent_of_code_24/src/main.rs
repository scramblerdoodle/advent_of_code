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
pub mod day14;
pub mod day15;
pub mod day15_v2;
pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;
mod utils;

use std::time::Instant;
use std::{fmt, io::stdin};

fn run_inputs<T, E>(f: fn(&str) -> Result<T, E>)
where
    T: fmt::Display,
    E: fmt::Display,
{
    let inputs = vec![
        ("Example", "example"),
        ("Example v2", "example_v2"),
        ("Actual", "actual"),
        ("Actual v2", "actual_v2"),
    ];
    for (name, path) in inputs {
        let now = Instant::now();

        match f(path) {
            Ok(result) => {
                println!("{name}: {result}");

                let elapsed = now.elapsed();
                println!("\tElapsed: {:.2?}", elapsed);
            }
            Err(err) => println!("{name}: ERROR! {err}"),
        }
    }
    println!();
}

fn main() {
    loop {
        let mut input = String::new();
        println!(
            "Choose a Day from {} to {}; 0 exits and input defaults to 0.",
            1, 25
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
            14 => run_inputs(day14::main),
            15 => run_inputs(day15::main),
            16 => run_inputs(day16::main),
            17 => println!("not yet implemented"),
            18 => println!("not yet implemented"),
            19 => println!("not yet implemented"),
            20 => println!("not yet implemented"),
            21 => println!("not yet implemented"),
            22 => println!("not yet implemented"),
            23 => println!("not yet implemented"),
            24 => println!("not yet implemented"),
            25 => println!("not yet implemented"),
            _ => println!("command not found"),
        }
    }
    println!("Bye!");
}
