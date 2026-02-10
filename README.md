# Advent of Code Solutions
This started as a local git repo just so I could keep track of some experiments while learning Rust, then I started AoC24 and now it's its own AoC solutions repository.

This is mostly a learning experience with Rust and Go.

## Usage AoC24
The folder `advent_of_code_24/` contains all solutions I've made for Adv of Code 24.
To run the code, just `cargo run` inside the `advent_of_code_24/` folder. It'll compile and execute the program and prompt you for the number of the Day you'd like to execute.
Choose a number between 1 and 25 (some are not yet implemented) and the appropriate day's code will run, returning the time elapsed for the execution (Parts 1 and 2).
For a full run with actual input, create a `.tests/` folder and add files with the input data for each day (e.g. the input data for AOC24 - Day 1 is expected to be `advent_of_code_24/.tests/day01.txt` -- note the 0-padding to a 2-digit number). You can change this path in the `utils.rs` file if you'd like.
If the input file for a certain day is missing, it'll just be skipped

## Usage AoC25
The folder `advent_of_code_25/` contains all solutions for Advent of Code 25, written in Go.
To run the code, just run `go run aoc25.go` in the `advent_of_code_25/` folder. You can also choose a specific day among the ones that have been implemented so far, e.g. `go run aoc25.go 3` to only run Day 3. You can also pass the flag `-d` for the debug print, e.g. `go run aoc25.go -d 7`
All test files are under `tests/` with the appropriate days' number (e.g. `tests/day03.txt` for Day 3). Add your own input to your heart's content!


## Checklist AoC25
- [ ] Improve `AOC24/day07.rs` -- it's got awful performance in Part 2
- [ ] Improve file structure -- keep `src/utils.rs` in a common folder across all AOC days maybe?
- [ ] Finish AOC24 (as of today, stopped on Day 16)

## Checklist AoC25
- [X] Better project structure in the Go portion (main function that calls specific days etc)
- [ ] Finish part 2 for Day 1 (such a simple problem, no idea what edge case I'm missing)
- [ ] Finish part 2 for Day 9 (taking forever, there's definitely some obvious math optimisation that gets me to figure out if it's inside/outside the allowed area)
- [ ] Finish part 2 for Day 10 (just need to write a diophantine equation solver)
- [ ] Finish Day 12 (got freaked out just from reading it, it looks NP-hard)
- [ ] Write test suite for everything using default input
- [ ] Explore go routines
