use std::{fmt, fs::read_to_string};

enum Quadrant {
    UL,
    UR,
    DL,
    DR,
    Middle,
}

impl Quadrant {
    fn get_quadrant(pos: (usize, usize), board_size: (usize, usize)) -> Self {
        let half_board = (board_size.0 / 2, board_size.1 / 2);

        if pos.0 < half_board.0 && pos.1 < half_board.1 {
            Quadrant::UL
        } else if pos.0 > half_board.0 && pos.1 < half_board.1 {
            Quadrant::UR
        } else if pos.0 < half_board.0 && pos.1 > half_board.1 {
            Quadrant::DL
        } else if pos.0 > half_board.0 && pos.1 > half_board.1 {
            Quadrant::DR
        } else {
            Quadrant::Middle
        }
    }
}

#[derive(Clone)]
struct RobotBoard {
    robots: Vec<Robot>,
    board_size: (usize, usize),
}

impl RobotBoard {
    fn move_robots(&mut self, n: isize) {
        for robot in self.robots.iter_mut() {
            robot.move_n_times(&self.board_size, n);
        }
    }
    fn compute_danger_level(&self) -> u32 {
        let mut quadrant_count: [usize; 4] = [0; 4];
        for robot in self.robots.iter() {
            let quadrant = Quadrant::get_quadrant(robot.pos, self.board_size);
            match quadrant {
                Quadrant::Middle => continue,
                _ => quadrant_count[quadrant as usize] += 1,
            }
        }

        quadrant_count.iter().fold(1, |acc, e| acc * e) as u32
    }
}
impl fmt::Display for RobotBoard {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = vec![vec!['.'; self.board_size.0]; self.board_size.1];

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

        writeln!(
            f,
            "{}\n",
            board
                .iter()
                .map(|v| v.iter().collect())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
#[derive(Clone, Debug)]
struct Robot {
    pos: (usize, usize),
    vel: (isize, isize),
}

impl Robot {
    fn new(pos: (usize, usize), vel: (isize, isize)) -> Self {
        Self { pos, vel }
    }

    fn move_n_times(&mut self, board_size: &(usize, usize), n: isize) {
        let next_pos_x = self.pos.0 as isize + self.vel.0 * n;
        let next_pos_y = self.pos.1 as isize + self.vel.1 * n;

        self.pos = (
            next_pos_x.rem_euclid(board_size.0 as isize) as usize,
            next_pos_y.rem_euclid(board_size.1 as isize) as usize,
        )
    }
}

fn day14(mut robot_board: RobotBoard) -> u32 {
    robot_board.move_robots(100);
    // println!("{}", robot_board);
    robot_board.compute_danger_level()
}

fn day14_v2(mut robot_board: RobotBoard) -> u32 {
    let mut danger_levels: Vec<u32> = vec![];
    let mut all_states: Vec<RobotBoard> = vec![];
    // 0 steps
    danger_levels.push(robot_board.compute_danger_level());
    all_states.push(robot_board.clone());

    // At most x*y states because modulo
    for _t in 0..robot_board.board_size.0 * robot_board.board_size.1 {
        robot_board.move_robots(1);
        danger_levels.push(robot_board.compute_danger_level());
        all_states.push(robot_board.clone());
    }

    // Response is magically min danger
    let min_danger = danger_levels.iter().min().unwrap();
    let min_danger_index = danger_levels.iter().position(|x| x == min_danger).unwrap();

    // println!("{}:\n{}", min_danger_index, all_states[min_danger_index]);
    min_danger_index as u32
}

fn parse_input(filepath: &str, board_size: (usize, usize)) -> RobotBoard {
    fn strip_robot_info(values: &str, prefix: &str) -> (isize, isize) {
        let info = values
            .strip_prefix(prefix)
            .unwrap()
            .split(",")
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        (
            *info.get(0).expect("Unexpected parsing"),
            *info.get(1).expect("Unexpected parsing"),
        )
    }

    let mut robots: Vec<Robot> = vec![];
    read_to_string(filepath).unwrap().lines().for_each(|l| {
        // values = [ "p=0,4", "v=3,-3" ]
        let mut values = l.split(" ");
        let pos = strip_robot_info(values.next().unwrap(), "p=");
        let vel = strip_robot_info(values.next().unwrap(), "v=");

        robots.push(Robot::new((pos.0 as usize, pos.1 as usize), (vel.0, vel.1)));
    });

    RobotBoard { robots, board_size }
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
        assert_eq!(main("example_v2"), 0);
    }

    #[test]
    fn test_actual_v2() {
        assert_eq!(main("actual_v2"), 7051);
    }
}
