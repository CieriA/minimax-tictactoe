use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Index, IndexMut},
    slice::Iter,
};
use colored::{ColoredString, Colorize};
use crate::geomath::Point;

const PLAYER: char = 'X';
const BOT: char = 'O';
#[inline(always)]
pub(crate) const fn p_name(turn: bool) -> char {
    if turn { PLAYER } else { BOT }
}
#[inline(always)]
pub(crate) fn p_colored(turn: bool) -> ColoredString {
    let s = format!("{}", p_name(turn));
    if turn { s.bright_blue() } else { s.bright_red() }
}

/// - `None`: empty
/// - `Some(true)`: player
/// - `Some(false)`: bot
type Square = Option<bool>;
type Row = [Square; Board::SIZE];
type Grid = [Row; Board::SIZE];
#[derive(Default, Debug, Clone)]
pub(crate) struct Board(Grid);

impl Board {
    pub(crate) const SIZE: usize = 3;
    
    #[inline(always)]
    pub(crate) fn iter(&self) -> Iter<Row> {
        self.0.iter()
    }
    fn center(&self) -> Square {
        self[Point::new(1, 1)]
    }
    #[inline]
    fn check_rows(&self, last_coord: Point) -> bool {
        let b = self[last_coord].unwrap();
        self[last_coord.y].iter().all(|square| square.is_some_and(|square| square == b)) 
    }
    #[inline]
    fn check_cols(&self, last_coord: Point) -> bool {
        let b = self[last_coord].unwrap();
        self.iter().all(|row| row[last_coord.x].is_some_and(|square| square == b))
    }
    /// Use knowing the last_coord is in a diagonal
    fn check_diagonals(&self, last_coord: Point) -> bool {
        let b = self[last_coord].unwrap();
        let Point { x, y } = last_coord;
        if x == y {
            (0..Board::SIZE)
                .all(|i| self[Point::new(i, i)].is_some_and(|square| square == b))
        } else {
            [self[last_coord], self.center(), self[Point::new(last_coord.y, last_coord.x)]]
                .into_iter()
                .all(|square| square.is_some_and(|square| square == b))
        }
        
        
        
    }
    pub(crate) fn check_win(&self, last_coord: Point) -> bool {
        let in_diagonal = 
            last_coord.x == last_coord.y ||
            (last_coord.x + last_coord.y + 1) % Self::SIZE == 0;
        
        self.check_rows(last_coord) ||
        self.check_cols(last_coord) ||
        if in_diagonal { self.check_diagonals(last_coord) } else { false }
    }
    #[inline]
    pub(crate) fn is_full(&self) -> bool {
        self
            .iter()
            .all(|row| row.iter().all(|square| square.is_some()))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for (y, row) in self.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                match square {
                    Some(b) => write!(f, "{}", p_colored(*b))?,
                    None => write!(f, "{}", x + y * Board::SIZE + 1)?,
                }
                if x + 1 != Board::SIZE {
                    write!(f, " | ")?;
                }
            }
            if y + 1 != Board::SIZE {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
impl Index<Point> for Board {
    type Output = Square;
    #[inline(always)]
    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}
impl IndexMut<Point> for Board {
    #[inline(always)]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}
impl Index<usize> for Board {
    type Output = Row;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
