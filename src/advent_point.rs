use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Point {
    pub i: i128,
    pub j: i128,
}

impl Point {
    pub fn new(i: i128, j: i128) -> Self {
        Point { i, j }
    }

    pub fn _dir_from(self, other: &Self) -> Dir {
        let ds = self - *other;
        if ds.i == 0 {
            if ds.j < 0 {
                Dir::Up
            } else if ds.j > 0 {
                Dir::Down
            } else {
                panic!("points are equal")
            }
        } else if ds.j == 0 {
            if ds.i < 0 {
                Dir::Left
            } else if ds.i > 0 {
                Dir::Right
            } else {
                panic!("points are equal")
            }
        } else {
            panic!("only cardnial directions supported");
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.i + rhs.i, self.j + rhs.j)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.i - rhs.i, self.j - rhs.j)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.i = self.i + rhs.i;
        self.j = self.j + rhs.j;
    }
}

impl Mul<i128> for Point {
    type Output = Point;

    fn mul(self, rhs: i128) -> Self::Output {
        Point {
            i: self.i * rhs,
            j: self.j * rhs,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn as_point(&self) -> Point {
        match self {
            Dir::Up => Point::new(0, -1),
            Dir::Down => Point::new(0, 1),
            Dir::Left => Point::new(-1, 0),
            Dir::Right => Point::new(1, 0),
        }
    }
}

impl From<&[u8]> for Dir {
    fn from(value: &[u8]) -> Self {
        match value {
            b"U" => Dir::Up,
            b"D" => Dir::Down,
            b"L" => Dir::Left,
            b"R" => Dir::Right,
            _ => panic!("bad direction"),
        }
    }
}
