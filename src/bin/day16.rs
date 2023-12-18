use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{
    collections::HashSet,
    fs::read,
    ops::{Add, AddAssign},
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
    } else {
        part2();
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    i: i64,
    j: i64,
}

impl Point {
    fn new(i: i64, j: i64) -> Self {
        Point { i, j }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.i + rhs.i, self.j + rhs.j)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.i = self.i + rhs.i;
        self.j = self.j + rhs.j;
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

fn part1() {
    let re = Regex::new(r"\S+").unwrap();
    let map = re.find_iter(&IN).map(|r| r.as_bytes()).collect::<Vec<_>>();

    let mut beams = vec![(Point::new(0, 0), Dir::Right)];
    let mut visited = HashSet::new();

    println!("{}x{}", map.len(), map[0].len());
    while let Some((p, dir)) = beams.pop() {
        if p.j < 0 || p.j >= map.len() as i64 || p.i < 0 as i64 || p.i >= map[0].len() as i64 {
            continue;
        }
        if visited.contains(&(p, dir)) {
            continue;
        }
        visited.insert((p, dir));
        match (map[p.j as usize][p.i as usize], dir) {
            (b'/', Dir::Up) => beams.push((p + Dir::Right.as_point(), Dir::Right)),
            (b'/', Dir::Down) => beams.push((p + Dir::Left.as_point(), Dir::Left)),
            (b'/', Dir::Left) => beams.push((p + Dir::Down.as_point(), Dir::Down)),
            (b'/', Dir::Right) => beams.push((p + Dir::Up.as_point(), Dir::Up)),

            (b'\\', Dir::Up) => beams.push((p + Dir::Left.as_point(), Dir::Left)),
            (b'\\', Dir::Down) => beams.push((p + Dir::Right.as_point(), Dir::Right)),
            (b'\\', Dir::Left) => beams.push((p + Dir::Up.as_point(), Dir::Up)),
            (b'\\', Dir::Right) => beams.push((p + Dir::Down.as_point(), Dir::Down)),

            (b'-', Dir::Up) | (b'-', Dir::Down) => {
                beams.push((p + Dir::Left.as_point(), Dir::Left));
                beams.push((p + Dir::Right.as_point(), Dir::Right));
            }
            (b'|', Dir::Left) | (b'|', Dir::Right) => {
                beams.push((p + Dir::Up.as_point(), Dir::Up));
                beams.push((p + Dir::Down.as_point(), Dir::Down));
            }
            _ => beams.push((p + dir.as_point(), dir)),
        }
    }

    let unique_visited = visited.iter().map(|(p, _)| p).collect::<HashSet<_>>();
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if unique_visited.contains(&Point::new(i as i64, j as i64)) {
                print!("#");
            } else {
                print!("{}", map[j][i] as char);
            }
        }
        println!("")
    }
    println!("{}", unique_visited.len());
}

fn part2() {
    let re = Regex::new(r"\S+").unwrap();
    let map = re.find_iter(&IN).map(|r| r.as_bytes()).collect::<Vec<_>>();

    let mut entries = vec![];
    for j in 0..map.len() {
        entries.push((Point::new(0, j as i64), Dir::Right));
        entries.push((Point::new(map[0].len() as i64 - 1, j as i64), Dir::Left));
    }
    for i in 0..map.len() {
        entries.push((Point::new(i as i64, 0), Dir::Down));
        entries.push((Point::new(i as i64, map.len() as i64), Dir::Up));
    }

    let mut energy = vec![];
    for start in entries {
        let mut beams = vec![start];
        let mut visited = HashSet::new();
        while let Some((p, dir)) = beams.pop() {
            if p.j < 0 || p.j >= map.len() as i64 || p.i < 0 as i64 || p.i >= map[0].len() as i64 {
                continue;
            }
            if visited.contains(&(p, dir)) {
                continue;
            }
            visited.insert((p, dir));
            match (map[p.j as usize][p.i as usize], dir) {
                (b'/', Dir::Up) => beams.push((p + Dir::Right.as_point(), Dir::Right)),
                (b'/', Dir::Down) => beams.push((p + Dir::Left.as_point(), Dir::Left)),
                (b'/', Dir::Left) => beams.push((p + Dir::Down.as_point(), Dir::Down)),
                (b'/', Dir::Right) => beams.push((p + Dir::Up.as_point(), Dir::Up)),

                (b'\\', Dir::Up) => beams.push((p + Dir::Left.as_point(), Dir::Left)),
                (b'\\', Dir::Down) => beams.push((p + Dir::Right.as_point(), Dir::Right)),
                (b'\\', Dir::Left) => beams.push((p + Dir::Up.as_point(), Dir::Up)),
                (b'\\', Dir::Right) => beams.push((p + Dir::Down.as_point(), Dir::Down)),

                (b'-', Dir::Up) | (b'-', Dir::Down) => {
                    beams.push((p + Dir::Left.as_point(), Dir::Left));
                    beams.push((p + Dir::Right.as_point(), Dir::Right));
                }
                (b'|', Dir::Left) | (b'|', Dir::Right) => {
                    beams.push((p + Dir::Up.as_point(), Dir::Up));
                    beams.push((p + Dir::Down.as_point(), Dir::Down));
                }
                _ => beams.push((p + dir.as_point(), dir)),
            }
        }

        let unique_visited = visited.iter().map(|(p, _)| p).collect::<HashSet<_>>();
        println!("{}", unique_visited.len());
        energy.push(unique_visited.len());
    }

    energy.sort();
    println!("{:?}", energy);
    println!("{}", energy.last().unwrap());
}
