use std::{fmt, fs::read_to_string};

use crate::utils::{get_test_file, Board, Direction, FileNotFound, ACTUAL, EXAMPLE};

enum State {
    Start,
    End,
    Wall,
    Empty,
    Visited(Direction, u32),
}

impl State {
    fn from_char(c: char) -> State {
        match c {
            '#' => State::Wall,
            '.' => State::Empty,
            'S' => State::Start,
            'E' => State::End,
            _ => panic!("Unexpected symbol in input"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            State::Wall => '#',
            State::Empty => '.',
            State::Start => 'S',
            State::End => 'E',
            State::Visited(d, _) => d.as_char(),
        }
    }
}

impl fmt::Display for Board<State> {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            concat!("Board:\n\t{}"),
            self.board
                .iter()
                .map(|v| v.iter().map(|s| s.to_char()).collect())
                .collect::<Vec<String>>()
                .join("\n\t"),
        )
    }
}

#[derive(PartialEq)]
struct Node {
    score: u32,
    pos: (usize, usize),
    dir: Direction,
}

struct Input {
    board: Board<State>,
    start: (usize, usize),
    minimum_score: u32,
}

impl Input {
    fn new(board: Vec<Vec<State>>) -> Self {
        let board = Board::new(board);
        let mut start = (0, 0);

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                match Some(board.get_pos((i, j)).unwrap()) {
                    Some(State::Start) => start = (i, j),
                    _ => (),
                }
            }
        }

        Self {
            board,
            start,
            minimum_score: u32::MAX,
        }
    }

    fn update_min(&mut self, other: u32) {
        if self.minimum_score > other {
            self.minimum_score = other;

            // println!("Min: {}\n{}", self.minimum_score, self.board);
        }
    }
    fn walk(&mut self, start: (usize, usize)) {
        /* Approach:
         *   All visited positions in board will have a score
         *   Queue structure: Vec<(score, pos<usize, usize>)>
         *   Add start to queue with score 0
         *   Init minimum score as +INF
         *
         *   While queue is not empty
         *       pos = pop lowest scored position in queue # be careful about this, could increase
         *       complexity by a lot -- maybe use priority queue
         *       For each valid direction from pos
         *           If Empty
         *               Compute score
         *               if score < minimum score:
         *                  update as visited with score
         *                  add to queue (score, pos)
         *           If Visited
         *               Compute score
         *               If score would be lower by visiting it through new path and score <
         *               minimum score:
         *                   update position's score in board
         *                   add to queue (score, pos) # not sure, this would create some overhead I
         *                   # think? I think it's necessary for computing shortest path but there's a part of me that
         *                   # thinks just woulnd't ever happen
         *               else continue
         *           If Start
         *               Continue
         *           If Wall
         *               Continue
         *           If End
         *               Update as visited with score
         *               Set minimum score as end score
         *
         */

        // Init queue with starting position
        let mut queue: Vec<Node> = vec![Node {
            score: 0,
            pos: start,
            dir: Direction::Right,
        }];

        while !queue.is_empty() {
            // Get min score from queue
            if let Some(lowest_scored_pos) = queue.iter().min_by_key(|x| x.score) {
                let curr_score = lowest_scored_pos.score;
                let curr_pos = lowest_scored_pos.pos;
                let prev_dir = lowest_scored_pos.dir;

                // println!("{}", self.board);
                // println!("Curr pos:{:?}", curr_pos);

                queue.remove(queue.iter().position(|x| x == lowest_scored_pos).unwrap());

                for d in &Direction::ORTHOGONALS {
                    let next_pos = self.board.add_direction(d, curr_pos).unwrap();
                    let next_state = self.board.get_pos(next_pos).unwrap();
                    // println!(
                    //     "Dir: {:?}, Next pos: {:?}, Next state: {:?}",
                    //     d,
                    //     next_pos,
                    //     next_state.to_char()
                    // );

                    match next_state {
                        State::Wall | State::Start => {}
                        State::Empty => {
                            // Compute score
                            let mut new_score = curr_score;
                            if *d != prev_dir {
                                new_score += 1000;
                            }
                            new_score += 1;

                            // If score goes beyond minimum score, no use computing it
                            if new_score < self.minimum_score {
                                self.board
                                    .update_pos(next_pos, State::Visited(*d, new_score));

                                queue.push(Node {
                                    score: new_score,
                                    pos: next_pos,
                                    dir: *d,
                                });
                            }
                        }
                        State::Visited(d, s) => {
                            // Compute score
                            let mut new_score = curr_score;
                            if *d != prev_dir {
                                new_score += 1000;
                            }
                            new_score += 1;

                            // If score goes beyond minimum score or visited already has lower
                            // score, no use computing it
                            if new_score < *s && new_score < self.minimum_score {
                                // Add to queue (score, pos)
                                queue.push(Node {
                                    score: new_score,
                                    pos: next_pos,
                                    dir: d.clone(),
                                });

                                self.board
                                    .update_pos(next_pos, State::Visited(*d, new_score));
                            }
                        }
                        State::End => {
                            // Set minimum score, with sanity check
                            if curr_score < self.minimum_score {
                                self.update_min(curr_score + 1);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn day16(mut input: Input) -> u32 {
    input.walk(input.start);
    input.minimum_score
}

fn day16_v2(_input: Input) -> u32 {
    let result: u32 = 0;
    result
}

fn parse_input(filepath: &str) -> Input {
    let mut board: Vec<Vec<State>> = vec![];

    read_to_string(filepath).unwrap().lines().for_each(|l| {
        board.push(
            l.chars()
                .map(|c| State::from_char(c))
                .collect::<Vec<State>>(),
        );
    });

    Input::new(board)
}

pub fn main(s: &str) -> Result<u32, FileNotFound> {
    match s {
        "example" => match get_test_file(EXAMPLE, "16") {
            Err(err) => Err(err),
            Ok(file) => Ok(day16(parse_input(&file))),
        },
        "example2" => match get_test_file(EXAMPLE, "16_2") {
            Err(err) => Err(err),
            Ok(file) => Ok(day16(parse_input(&file))),
        },
        "actual" => match get_test_file(ACTUAL, "16") {
            Err(err) => Err(err),
            Ok(file) => Ok(day16(parse_input(&file))),
        },
        "example_v2" => match get_test_file(EXAMPLE, "16") {
            Err(err) => Err(err),
            Ok(file) => Ok(day16_v2(parse_input(&file))),
        },
        "example2_v2" => match get_test_file(EXAMPLE, "16_2") {
            Err(err) => Err(err),
            Ok(file) => Ok(day16_v2(parse_input(&file))),
        },
        "actual_v2" => match get_test_file(ACTUAL, "16") {
            Err(err) => Err(err),
            Ok(file) => Ok(day16_v2(parse_input(&file))),
        },
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(main("example").unwrap(), 7036);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(main("example2").unwrap(), 11048);
    }

    #[test]
    fn test_example_1_v2() {
        assert_eq!(main("example_v2").unwrap(), 0);
    }

    #[test]
    fn test_example_2_v2() {
        assert_eq!(main("example2_v2").unwrap(), 0);
    }
}
