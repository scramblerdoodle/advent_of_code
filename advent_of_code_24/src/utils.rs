use std::ops::{Index, IndexMut};
use std::slice::Iter;
use std::vec::IntoIter;
use std::{fmt, slice};

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
}

impl Direction {
    pub const ORTHOGONALS: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];
    pub const DIAGONALS: [Self; 4] = [Self::UpRight, Self::DownRight, Self::DownLeft, Self::UpLeft];
    pub const DIRECTIONS: [Self; 8] = [
        Self::Up,
        Self::UpRight,
        Self::Right,
        Self::DownRight,
        Self::Down,
        Self::DownLeft,
        Self::Left,
        Self::UpLeft,
    ];

    pub fn get_direction(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::UpRight => (-1, 1),
            Self::DownRight => (1, 1),
            Self::DownLeft => (1, -1),
            Self::UpLeft => (-1, -1),
        }
    }

    pub fn next_orth(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            _ => panic!("For diagonals, use `next_diag`. For all directions, use `next_all_dir`"),
        }
    }

    pub fn next_diag(&self) -> Self {
        todo!()
    }

    pub fn next_all_dir(&self) -> Self {
        todo!()
    }

    pub fn as_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
            _ => panic!("Diagonals don't support characters yet."),
        }
    }
    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Diagonals don't support characters yet."),
        }
    }
}

#[derive(Clone)]
pub struct Board<T> {
    pub board: Vec<Vec<T>>,
}

impl<T> Board<T> {
    pub fn new(board: Vec<Vec<T>>) -> Self {
        Board { board }
    }

    pub fn from_size(size: (usize, usize), default: T) -> Self
    where
        T: Clone,
    {
        Board {
            board: vec![vec![default; size.1]; size.0],
        }
    }

    pub fn len(&self) -> usize {
        self.board.len()
    }

    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.board.iter()
    }

    pub fn get(&self, i: usize) -> Option<&Vec<T>> {
        self.board.get(i)
    }

    pub fn get_pos(&self, pos: (usize, usize)) -> Option<&T> {
        self.get(pos.0).expect("Out of bounds").get(pos.1)
    }

    pub fn update_pos(&mut self, pos: (usize, usize), value: T) {
        self.board[pos.0][pos.1] = value
    }

    pub fn add_direction(&self, dir: &Direction, pos: (usize, usize)) -> Option<(usize, usize)> {
        let step = dir.get_direction();
        let (next_i, next_j): (usize, usize);

        // Guards against negative out of bounds
        match pos.0.checked_add_signed(step.0) {
            Some(n) => next_i = n,
            None => return None,
        }

        // Guards against negative out of bounds
        match pos.1.checked_add_signed(step.1) {
            Some(n) => next_j = n,
            None => return None,
        }

        // Guards against positive out of bounds
        if let Some(next_line) = self.board.get(next_i) {
            // Guards against positive out of bounds
            if let Some(_) = next_line.get(next_j) {
                return Some((next_i, next_j));
            }
        };
        None
    }
}

impl<T> IntoIterator for Board<T> {
    type Item = Vec<T>;
    type IntoIter = IntoIter<Vec<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.board.into_iter()
    }
}

impl<T> Index<usize> for Board<T> {
    type Output = Vec<T>;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.board.index(index)
    }
}

impl<T> IndexMut<usize> for Board<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.board.index_mut(index)
    }
}

impl fmt::Display for Board<char> {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            concat!("Board:\n\t{}"),
            self.board
                .iter()
                .map(|v| v.iter().collect())
                .collect::<Vec<String>>()
                .join("\n\t"),
        )
    }
}

macro_rules! impl_board_display {
    (for $($t:ty),+) => {
        $(impl fmt::Display for Board<$t> {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            concat!("Board:\n\t{}"),
            self.board
                .iter()
                .map(|v| v
                    .iter()
                    .map(|d| char::from_digit(d.clone().try_into().unwrap(), 10).expect("Type can't be converted into char from digit"))
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n\t"),
        )
    }
        })*
    }
}
impl_board_display!(for bool, u8, u16, u32, u64);
