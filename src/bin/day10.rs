use clap::Parser;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet, LinkedList},
    fs::read,
    ops::Add,
    rc::Rc,
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

#[derive(Debug, Clone)]
struct Node {
    pipe: char,
    pt: Point,
    ch: Vec<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    i: i64,
    j: i64,
}

impl Point {
    fn rel(&self, delta: Point) -> Point {
        Point {
            i: self.i + delta.i,
            j: self.j + delta.j,
        }
    }

    fn new(i: i64, j: i64) -> Self {
        Self { i, j }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        self.rel(rhs)
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self {
            i: value.0,
            j: value.1,
        }
    }
}

const UP: Point = Point { i: 0, j: -1 };
const DOWN: Point = Point { i: 0, j: 1 };
const LEFT: Point = Point { i: -1, j: 0 };
const RIGHT: Point = Point { i: 1, j: 0 };

fn part1() {
    let mut map = HashMap::new();
    let mut i = 0;
    let mut j = 0;
    let mut start = Point::new(0, 0);
    let mut w = 0;
    let mut h = 0;
    for (_, b) in IN.iter().enumerate() {
        if b == &('S' as u8) {
            start = Point::new(i, j);
        }
        if b != &('\n' as u8) {
            let pt = Point::new(i, j);
            let children = match *b {
                b'|' => vec![pt + UP, pt + DOWN],
                b'-' => vec![pt + LEFT, pt + RIGHT],
                b'7' => vec![pt + LEFT, pt + DOWN],
                b'F' => vec![pt + RIGHT, pt + DOWN],
                b'L' => vec![pt + RIGHT, pt + UP],
                b'J' => vec![pt + LEFT, pt + UP],
                b'S' => vec![],
                b'.' => vec![],
                _ => panic!("Unexpected token"),
            };
            map.insert(
                pt,
                Rc::new(Node {
                    pipe: *b as char,
                    pt,
                    ch: children,
                }),
            );
            i += 1;
        } else {
            if i > w {
                w = i;
            }
            if j > h {
                h = j;
            }
            i = 0;
            j += 1;
        }
    }
    h += 1; // since the boundry doesn't exist vs. newlines
    println!("{}x{}", w, h);
    let start = map[&start].clone();
    let mut paths = start
        .ch
        .iter()
        .filter_map(|p| {
            map.get(p).map(|n| {
                n.ch.contains(&start.pt)
                    .then_some((start.clone(), map[p].clone(), 0i64))
            })
        })
        .flatten()
        .collect::<Vec<_>>();
    println!("{:?}", paths);

    let mut visited = HashMap::from([(start.pt, 0)]);
    while let Some((prev, curr, step)) = paths.pop() {
        assert_eq!(curr.ch.len(), 2);
        let next = if curr.ch[0] == prev.pt {
            map[&curr.ch[1]].clone()
        } else {
            map[&curr.ch[0]].clone()
        };
        if next.pt == start.pt {
            continue;
        }
        if !visited.contains_key(&curr.pt) || step + 1 < visited[&curr.pt] {
            visited.insert(curr.pt, step + 1);
            paths.push((curr, next, step + 1))
        }
    }
    println!("{}", visited.values().max().unwrap());
    //_dump(h, w, visited, map);
}

fn _dump(h: i64, w: i64, visited: HashMap<(i64, i64), i64>, map: HashMap<(i64, i64), Rc<Node>>) {
    for j in 0..h {
        print!("{} ", j);
        for i in 0..w {
            if let Some(step) = visited.get(&(i, j)) {
                print!("{}", step);
            } else {
                print!("{}", map[&(i, j)].pipe)
            }
        }
        println!("");
    }
}

fn part2() {
    let mut map = HashMap::new();
    let mut i = 0;
    let mut j = 0;
    let mut start = Point::new(0, 0);
    let mut w = 0;
    let mut h = 0;
    for (_, b) in IN.iter().enumerate() {
        if b == &('S' as u8) {
            start = Point::new(i, j);
        }
        if b != &('\n' as u8) {
            let pt = Point::new(i, j);
            let children = match *b {
                b'|' => vec![pt + UP, pt + DOWN],
                b'-' => vec![pt + LEFT, pt + RIGHT],
                b'7' => vec![pt + LEFT, pt + DOWN],
                b'F' => vec![pt + RIGHT, pt + DOWN],
                b'L' => vec![pt + RIGHT, pt + UP],
                b'J' => vec![pt + LEFT, pt + UP],
                b'S' => vec![],
                b'.' => vec![],
                _ => panic!("Unexpected token"),
            };
            map.insert(
                pt,
                Rc::new(Node {
                    pipe: *b as char,
                    pt,
                    ch: children,
                }),
            );
            i += 1;
        } else {
            if i > w {
                w = i;
            }
            if j > h {
                h = j;
            }
            i = 0;
            j += 1;
        }
    }
    h += 1; // since the boundry doesn't exist vs. newlines
    println!("{}x{}", w, h);
    let start = map[&start].clone();
    let mut paths = start
        .ch
        .iter()
        .filter_map(|p| {
            map.get(p).map(|n| {
                if n.pipe.ne(&'.') && n.ch.contains(&start.pt) {
                    Some((start.clone(), map[p].clone(), 0i64))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<Vec<_>>();
    paths.truncate(1);

    let mut pipe_loop = HashMap::from([(start.pt, 0)]);
    while let Some((prev, curr, step)) = paths.pop() {
        if curr.pt == start.pt {
            continue;
        }
        assert_eq!(curr.ch.len(), 2);
        let next = if curr.ch[0] == prev.pt {
            map[&curr.ch[1]].clone()
        } else {
            map[&curr.ch[0]].clone()
        };

        if !pipe_loop.contains_key(&curr.pt) || step + 1 < pipe_loop[&curr.pt] {
            pipe_loop.insert(curr.pt, step + 1);
            paths.push((curr, next, step + 1))
        }
    }

    for loc in map.keys().cloned().collect::<Vec<_>>() {
        if !pipe_loop.contains_key(&loc) {
            map.insert(
                loc,
                Rc::new(Node {
                    pipe: '.',
                    pt: loc,
                    ch: vec![],
                }),
            );
        }
    }

    let mut paths = LinkedList::from([Point::new(-1, -1)]);
    let mut visited = HashSet::new();
    visited.extend(paths.iter().cloned());
    let mut inner = map.keys().collect::<HashSet<_>>();
    while let Some(from) = paths.pop_front() {
        visited.insert(from);
        inner.remove(&from);

        let curr = map
            .get(&from)
            .or(Some(&Rc::new(Node {
                pipe: '.',
                pt: from,
                ch: vec![],
            })))
            .cloned()
            .unwrap();

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let to = from + dir;
            if !(-1..=w).contains(&to.i) || !(-1..=h).contains(&to.j) || visited.contains(&to) {
                continue;
            }

            let next = map.get(&to).cloned().or_else(|| {
                Some(Rc::new(Node {
                    pipe: '.',
                    pt: to,
                    ch: vec![],
                }))
            });

            if !visited.contains(&to) {
                paths.push_back(to);
                // println!("{:?} -> {:?}", from, to);
                // println!("{:?} | {:?}", map.get(&to), curr);
                // dump2(
                //     h,
                //     w,
                //     pipe_loop.clone(),
                //     &map,
                //     visited.clone(),
                //     paths.clone(),
                //     &inner,
                // );
            }
        }
    }
    println!("   -01234567890");
    for j in -1..=h {
        print!("{:3} ", j);
        for i in -1..=w {
            let pt = Point::new(i, j);
            if let Some(_) = visited.get(&pt) {
                if pipe_loop.contains_key(&pt) {
                    print!(
                        "{}",
                        match map[&pt].pipe {
                            '7' => '\\',
                            'F' => '/',
                            'L' => '\\',
                            'J' => '/',
                            '|' => '[',
                            '-' => '_',
                            'S' => 's',
                            _ => panic!("wat"),
                        }
                    )
                } else {
                    print!(" ");
                }
            } else if pipe_loop.contains_key(&pt) {
                print!("{}", map[&pt].pipe)
                //print!(".");
            } else if paths.contains(&pt) {
                print!("#");
            } else if inner.contains(&pt) {
                print!("*")
            } else {
                print!("{}", map[&pt].pipe)
            }
        }
        println!("");
        println!("{}", inner.len());
    }
}
