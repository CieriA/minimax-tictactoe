use std::{error::Error, ops::{Add, AddAssign}};
use crate::board::Board;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Point {
    #[inline(always)]
    pub(crate) const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl TryFrom<usize> for Point {
    type Error = Box<dyn Error>;
    #[inline]
    fn try_from(i: usize) -> Result<Self, Self::Error> {
        if i >= Board::SIZE * Board::SIZE {
            Err(format!("Invalid point index: {}", i).into())
        } else {
            Ok(Point::new(
                i % Board::SIZE,
                i / Board::SIZE
            ))
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
