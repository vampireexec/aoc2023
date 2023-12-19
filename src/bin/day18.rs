use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read,
    ops::{Add, AddAssign, Mul, Sub},
    str::from_utf8,
    vec,
};

#[derive(Parser, Debug)]
#[command(author="Vampire Exec", version="0.0", about=format!("solution for {}", file!()), long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,
    #[arg(long)]
    part: u8,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
    static ref IN: Vec<u8> = ARGS.input.as_ref().map_or(vec![], |p| read(p).unwrap());
}

fn main() {
    if ARGS.part == 1 {
        part1();
        // wrong 36070
    } else {
        part2();
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    i: i128,
    j: i128,
}

impl Point {
    fn new(i: i128, j: i128) -> Self {
        Point { i, j }
    }

    fn _dir_from(self, other: &Self) -> Dir {
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
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn as_point(&self) -> Point {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Instr {
    dir: Dir,
    count: i128,
    color: (u8, u8, u8),
}

fn part1() {
    let re = Regex::new(r"([UDLR]) (\d+) \(#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})\)").unwrap();
    let mut toks = re.captures_iter(&IN).map(|c| c.extract::<5>());

    let mut instr = vec![];
    while let Some((_, groups)) = toks.next() {
        let dir = Dir::from(groups[0]);
        let count = from_utf8(groups[1]).unwrap().parse().unwrap();
        let (r, g, b) = (
            u8::from_str_radix(from_utf8(groups[2]).unwrap(), 16).unwrap(),
            u8::from_str_radix(from_utf8(groups[3]).unwrap(), 16).unwrap(),
            u8::from_str_radix(from_utf8(groups[4]).unwrap(), 16).unwrap(),
        );
        instr.push(Instr {
            dir,
            count,
            color: (r, g, b),
        });
    }

    let mut curr = Point::new(0, 0);
    let mut dug = HashSet::new();
    dug.insert(curr);
    for i in instr {
        for _ in 0..i.count {
            curr += i.dir.as_point();
            dug.insert(curr);
        }
    }

    let min = Point {
        i: dug
            .iter()
            .min_by(|a, b| a.i.cmp(&b.i))
            .map(|p| p.i)
            .unwrap()
            - 1,
        j: dug
            .iter()
            .min_by(|a, b| a.j.cmp(&b.j))
            .map(|p| p.j)
            .unwrap()
            - 1,
    };
    let max = Point {
        i: dug
            .iter()
            .max_by(|a, b| a.i.cmp(&b.i))
            .map(|p| p.i)
            .unwrap()
            + 1,
        j: dug
            .iter()
            .max_by(|a, b| a.j.cmp(&b.j))
            .map(|p| p.j)
            .unwrap()
            + 1,
    };

    //flood
    let mut outside = HashSet::new();
    let mut stack = vec![min.clone()];

    while let Some(curr) = stack.pop() {
        outside.insert(curr.clone());
        for adj in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let next = curr + adj.as_point();
            if dug.contains(&next) || outside.contains(&next) {
                continue;
            }
            if next.i < min.i || next.j < min.j || next.i > max.i || next.j > max.j {
                continue;
            }
            stack.push(next);
        }
    }

    let mut start = None;
    'search: for p in &dug {
        for adj in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let p = *p + adj.as_point();
            if !outside.contains(&p) && !dug.contains(&p) {
                start.replace(p);
                break 'search;
            }
        }
    }

    if start.is_none() {
        panic!("No where to start");
    }

    let mut stack = vec![start.unwrap()];
    while let Some(curr) = stack.pop() {
        dug.insert(curr);
        for adj in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let next = curr + adj.as_point();
            if dug.contains(&next) || outside.contains(&next) {
                continue;
            }
            stack.push(next);
        }
    }

    for j in min.j..=max.j {
        for i in min.i..=max.i {
            let p = Point::new(i, j);
            if outside.contains(&p) {
                print!(".");
            } else if dug.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    println!("{min:?} {max:?} {}", dug.len());
}

fn part2() {
    let re = Regex::new(r"\(#([a-f0-9]{5})([a-f0-9])\)").unwrap();
    let mut toks = re.captures_iter(&IN).map(|c| c.extract::<2>());

    let mut instr = vec![];
    while let Some((_, groups)) = toks.next() {
        let dir = match groups[1] {
            b"0" => Dir::Right,
            b"1" => Dir::Down,
            b"2" => Dir::Left,
            b"3" => Dir::Up,
            _ => panic!("bad direction"),
        };
        let count = i128::from_str_radix(from_utf8(groups[0]).unwrap(), 16).unwrap();
        let (r, g, b) = (0, 0, 0);
        instr.push(Instr {
            dir,
            count,
            color: (r, g, b),
        });
    }

    let mut curr = Point::new(0, 0);
    let mut points = vec![];
    for i in &instr {
        let next = curr + (i.dir.as_point() * i.count);
        points.push(curr);
        curr = next;
    }
    points.push(Point::new(0, 0));

    let mut sum = 0i128;
    for i in 0..(points.len() - 1) {
        let p1 = points[i];
        let p2 = points[i + 1];
        let tri = p1.i * p2.j - p2.i * p1.j;
        sum += tri;
        println!("{:?} {:?} = {} ({})", p1, p2, tri, sum / 2,);
    }

    let mut sum = 0i128;
    for i in (0..(points.len() - 1)).rev() {
        let p1 = points[i];
        let p2 = points[i + 1];
        let tri = p1.i * p2.j - p2.i * p1.j;
        sum += tri;
        println!("{:?} {:?} = {} ({})", p1, p2, tri, sum / 2,);
    }
}
