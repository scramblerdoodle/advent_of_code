use std::{fmt, fs::read_to_string};

use crate::utils::Board;

struct Input {
    robots: Vec<Robot>,
    board_size: (usize, usize),
}

impl fmt::Display for Input {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = Board::from_size((self.board_size.1, self.board_size.0), '.');

        for r in self.robots.iter() {
            let x = r.pos.0 as usize;
            let y = r.pos.1 as usize;
            match board[y][x] {
                '.' => board[y][x] = '1',
                _ => {
                    let d = board[y][x].to_digit(10).unwrap();
                    board[y][x] = char::from_digit(d + 1, 10).unwrap();
                }
            };
        }
        writeln!(f, "{}", board)
    }
}
#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn new(pos: (i32, i32), vel: (i32, i32)) -> Self {
        Self { pos, vel }
    }

    fn pos_in_100s(&self, board_size: &(usize, usize)) -> (i32, i32) {
        let future_pos_x = self.pos.0 + self.vel.0 * 100;
        let future_pos_y = self.pos.1 + self.vel.1 * 100;

        (
            future_pos_x.rem_euclid(board_size.0 as i32),
            future_pos_y.rem_euclid(board_size.1 as i32),
        )
    }
}

#[derive(PartialEq, Debug)]
enum Quadrant {
    UL,
    UR,
    DL,
    DR,
    Middle,
}

impl Quadrant {
    fn get_quadrant(pos: (i32, i32), board_size: (usize, usize)) -> Self {
        let x = pos.0 as usize;
        let y = pos.1 as usize;

        let half_board_x = board_size.0 / 2;
        let half_board_y = board_size.1 / 2;

        // println!("Half: ({}, {})", half_board_x, half_board_y);

        if x == half_board_x {
            return Quadrant::Middle;
        } else if y == half_board_y {
            return Quadrant::Middle;
        } else if x < half_board_x && y < half_board_y {
            Quadrant::UL
        } else if x > half_board_x && y < half_board_y {
            Quadrant::UR
        } else if x < half_board_x && y > half_board_y {
            Quadrant::DL
        } else if x > half_board_x && y > half_board_y {
            Quadrant::DR
        } else {
            Quadrant::Middle
        }
    }
}

fn day14(input: Input) -> u32 {
    let mut quadrant_count: [usize; 4] = [0; 4];
    let mut future: Vec<Robot> = vec![];
    for robot in input.robots {
        let future_pos = robot.pos_in_100s(&input.board_size);

        // FIXME: Debugging
        future.push(Robot {
            pos: future_pos,
            vel: robot.vel,
        });

        let future_quadrant = Quadrant::get_quadrant(future_pos, input.board_size);
        if future_quadrant == Quadrant::Middle {
            continue;
        }
        quadrant_count[future_quadrant as usize] += 1;
    }
    // FIXME: Debugging
    println!(
        "{}",
        Input {
            robots: future,
            board_size: input.board_size,
        }
    );
    // .print();
    quadrant_count.iter().fold(1, |acc, e| acc * e) as u32
}

fn day14_v2(input: Input) -> u32 {
    let mut result: u32 = 0;
    result
}

fn parse_input(filepath: &str, board_size: (usize, usize)) -> Input {
    let mut robots: Vec<Robot> = vec![];
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        // values = [pos, vel]
        let mut values = l.split(" ");
        let pos = values
            .next()
            .unwrap()
            .strip_prefix("p=")
            .unwrap()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let vel = values
            .next()
            .unwrap()
            .strip_prefix("v=")
            .unwrap()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        robots.push(Robot::new((pos[0], pos[1]), (vel[0], vel[1])));
    });

    // FIXME: Debugging purposes
    // let input = Input { robots, board_size };
    // println!("{}", input);
    Input { robots, board_size }
}

pub fn main(s: &str) -> u32 {
    match s {
        "example" => day14(parse_input("./tests/day14/example.txt", (11, 7))),
        "actual" => day14(parse_input("./tests/day14/actual.txt", (101, 103))),
        "example_v2" => day14_v2(parse_input("./tests/day14/example.txt", (11, 7))),
        "actual_v2" => day14_v2(parse_input("./tests/day14/actual.txt", (101, 103))),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example"), 12);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2"), 10);
    }
}
