# Advent of Code Solutions
This started as a local git repo just so I could keep track of some experiments while learning Rust, then I started AOC24 and now it's separated into its own AOC solutions repository.

This is mostly a learning experience with Rust.

## Usage
The folder `advent_of_code_24/` contains all solutions I've made for Adv of Code 24.
To run the code, just `cargo run` inside the `advent_of_code_24/` folder. It'll compile and execute the program and prompt you for the number of the Day you'd like to execute.
Choose a number between 1 and 25 (some are not yet implemented) and the appropriate day's code will run, returning the time elapsed for the execution (Parts 1 and 2).
For a full run with actual input, create a `.tests/` folder and add files with the input data for each day (e.g. the input data for AOC24 - Day 1 is expected to be `advent_of_code_24/.tests/day01.txt` -- note the 0-padding to a 2-digit number). You can change this path in the `utils.rs` file if you'd like.
If the input file for a certain day is missing, it'll just be skipped

## Checklist
- [ ] Improve `AOC24/day07.rs` -- it's got awful performance in Part 2
- [ ] Improve file structure -- keep `src/utils.rs` in a common folder across all AOC days maybe?
- [ ] Finish AOC24 (as of today, stopped on Day 16)
