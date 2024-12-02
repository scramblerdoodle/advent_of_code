pub mod day1;
// pub mod day2;
// pub mod day3;
// pub mod day4;

use std::io::stdin;
fn main() {
    loop {
        let mut input = String::new();
        println!(
            "Choose a Day from {} to {}; 0 exits and default is 0.",
            1, 3
        );
        let input = match stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => panic!("Unexpected input."),
        }
        .chars()
        .nth(0)
        .unwrap_or('0')
        .to_digit(10)
        .unwrap_or(0);

        match input {
            0 => break,
            1 => {
                println!("Example: {}", day1::main("example"));
                println!("Example v2: {}", day1::main("example_v2"));
                println!("Actual: {}", day1::main("actual"));
                println!("Actual v2: {}", day1::main("actual_v2"));
            }
            // 2 => {
            //     println!("{}", day2::main("actual"));
            //     println!("{}", day2::main("actual_v2"));
            // }
            // 3 => {
            //     println!("{}", day3::main("actual"));
            //     println!("{}", day3::main("actual_v2"));
            // }
            // 4 => {
            //     println!("{}", day4::main("actual"));
            //     println!("{}", day4::main("actual_v2"));
            // }
            // 5 => {
            //     println!("{}", day5::main("actual"));
            //     println!("{}", day5::main("actual_v2"));
            // }
            _ => todo!(),
        }
        println!();
    }
}
