use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{
    collections::{BinaryHeap, HashMap, LinkedList},
    fs::read,
    iter::once,
    ops::{Add, AddAssign, Sub},
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

    fn dir_from(self, other: &Self) -> Dir {
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord)]
struct Path {
    path: LinkedList<Point>,
    heat: i64,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.heat.partial_cmp(&self.heat) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.path.partial_cmp(&self.path)
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
    let re = Regex::new(r"\d+").unwrap();
    let toks = &mut re.find_iter(&IN);
    let map = toks
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|d| (d - b'0') as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut queue = BinaryHeap::from([
        Path {
            path: LinkedList::from([Point::new(1, 0), Point::new(0, 0)]),
            heat: map[0][1],
        },
        Path {
            path: LinkedList::from([Point::new(0, 1), Point::new(0, 0)]),
            heat: map[1][0],
        },
    ]);

    let end = Point::new(map[0].len() as i64 - 1, map.len() as i64 - 1);
    let mut best = Path {
        path: LinkedList::new(),
        heat: i64::MAX,
    };
    let mut count = 0;
    let mut visited = HashMap::new();

    while let Some(path) = queue.pop() {
        // print!("{} <<< ", path.heat);
        // println!("{:?}", queue.iter().map(|p| p.heat).collect::<Vec<_>>());
        let recents = path.path.iter().take(4).cloned().collect::<Vec<_>>();

        if path.heat >= best.heat {
            continue;
        }

        if recents[0] == end {
            println!("new best {}", count);
            count += 1;
            best = path;
            continue;
        }

        let curr = path.path;
        let heat = path.heat;

        let arrived_by = recents[0].dir_from(&recents[1]);
        let mut adj = vec![];
        if recents.len() < 4
            || recents[1].dir_from(&recents[2]) != arrived_by
            || recents[2].dir_from(&recents[3]) != arrived_by
        {
            adj.push(arrived_by);
        }
        match arrived_by {
            Dir::Up | Dir::Down => {
                adj.push(Dir::Left);
                adj.push(Dir::Right);
            }
            Dir::Right | Dir::Left => {
                adj.push(Dir::Up);
                adj.push(Dir::Down);
            }
        }

        adj.sort();
        let cache_key = (recents[0], adj.clone());
        if let Some(other) = visited.get(&cache_key) {
            if *other < heat {
                continue;
            }
        }

        visited.insert(cache_key, heat);

        for dir in adj {
            let p = recents[0] + dir.as_point();
            if p.i < 0 || p.i as usize >= map[0].len() || p.j < 0 || p.j as usize >= map.len() {
                continue;
            }
            if curr.contains(&p) {
                continue;
            }
            let heat = heat + map[p.j as usize][p.i as usize];
            let next = once(p)
                .chain(curr.iter().cloned())
                .collect::<LinkedList<_>>();
            queue.push(Path { path: next, heat });
        }
    }

    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if best.path.contains(&Point::new(i as i64, j as i64)) {
                print!("*");
            } else {
                print!("{}", map[j][i])
            }
        }
        println!("")
    }
    println!("{:?}", best.heat);
}

fn part2() {}
