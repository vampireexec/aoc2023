use clap::Parser;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    fs::read,
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
    _pipe: char,
    pt: (i64, i64),
    ch: Vec<(i64, i64)>,
}

fn part1() {
    let mut map = HashMap::new();
    let mut i = 0;
    let mut j = 0;
    let mut start = (0, 0);
    let mut w = 0;
    let mut h = 0;
    for (_, b) in IN.iter().enumerate() {
        if b == &('S' as u8) {
            start = (i, j);
        }
        if b != &('\n' as u8) {
            let children = match *b {
                b'|' => vec![(i, j - 1), (i, j + 1)],
                b'-' => vec![(i - 1, j), (i + 1, j)],
                b'7' => vec![(i - 1, j), (i, j + 1)],
                b'F' => vec![(i + 1, j), (i, j + 1)],
                b'L' => vec![(i + 1, j), (i, j - 1)],
                b'J' => vec![(i - 1, j), (i, j - 1)],
                b'S' => vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)],
                b'.' => vec![],
                _ => panic!("Unexpected token"),
            };
            map.insert(
                (i, j),
                Rc::new(Node {
                    _pipe: *b as char,
                    pt: (i, j),
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
                print!("{}", map[&(i, j)]._pipe)
            }
        }
        println!("");
    }
}

fn part2() {
    let mut map = HashMap::new();
    let mut i = 0;
    let mut j = 0;
    let mut start = (0, 0);
    let mut w = 0;
    let mut h = 0;
    for (_, b) in IN.iter().enumerate() {
        if b == &('S' as u8) {
            start = (i, j);
        }
        if b != &('\n' as u8) {
            let children = match *b {
                b'|' => vec![(i, j - 1), (i, j + 1)],
                b'-' => vec![(i - 1, j), (i + 1, j)],
                b'7' => vec![(i - 1, j), (i, j + 1)],
                b'F' => vec![(i + 1, j), (i, j + 1)],
                b'L' => vec![(i + 1, j), (i, j - 1)],
                b'J' => vec![(i - 1, j), (i, j - 1)],
                b'S' => vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)],
                b'.' => vec![],
                _ => panic!("Unexpected token"),
            };
            map.insert(
                (i, j),
                Rc::new(Node {
                    _pipe: *b as char,
                    pt: (i, j),
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

    let mut paths = (-1..=w)
        .flat_map(|i| [(i, -1), (i, h)])
        .chain((-1..=h).flat_map(|j| [(-1, j), (w, j)]))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    visited.extend(paths.iter().cloned());
    let mut inner = map.keys().collect::<HashSet<_>>();
    while let Some(from) = paths.pop() {
        inner.remove(&from);
        let curr = map.get(&from).cloned();
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let to = (from.0 + di, from.1 + dj);
            if !(-1..=w).contains(&to.0) || !(-1..=h).contains(&to.1) || visited.contains(&to) {
                continue;
            }
            if let Some(curr) = curr.as_ref() {
                if pipe_loop.contains_key(&curr.pt)
                    && (curr._pipe == '|' || curr._pipe == '-')
                    && !curr.ch.contains(&to)
                {
                    continue;
                }
            }
            if let Some(next) = map.get(&to) {
                if pipe_loop.contains_key(&next.pt)
                    && (next._pipe == '|' || next._pipe == '-')
                    && !next.ch.contains(&from)
                {
                    continue;
                }
            }
            // if [(6, 6), (7, 6), (3, 2), (3, 3)].contains(&to) {
            //     dump2(
            //         h,
            //         w,
            //         pipe_loop.clone(),
            //         &map,
            //         visited.clone(),
            //         paths.clone(),
            //         &inner,
            //     );
            //     panic!("{:?}", (curr, map.get(&to), (i, j)))
            // }
            if !visited.contains(&to) {
                visited.insert(to);
                paths.push(to);
                println!("{:?} -> {:?}", from, to);
                println!("{:?} | {:?}", map.get(&to), curr);
                dump2(
                    h,
                    w,
                    pipe_loop.clone(),
                    &map,
                    visited.clone(),
                    paths.clone(),
                    &inner,
                );
            }
        }
    }

    dump2(h, w, pipe_loop, &map, visited, paths, &inner);

    println!("{}", inner.len());
}

fn dump2(
    h: i64,
    w: i64,
    pipe_loop: HashMap<(i64, i64), i64>,
    map: &HashMap<(i64, i64), Rc<Node>>,
    visited: HashSet<(i64, i64)>,
    paths: Vec<(i64, i64)>,
    inner: &HashSet<&(i64, i64)>,
) {
    println!("   -01234567890");
    for j in -1..=h {
        print!("{:3} ", j);
        for i in -1..=w {
            if let Some(_) = visited.get(&(i, j)) {
                if pipe_loop.contains_key(&(i, j)) {
                    print!(
                        "{}",
                        match map[&(i, j)]._pipe {
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
            } else if pipe_loop.contains_key(&(i, j)) {
                print!("{}", map[&(i, j)]._pipe)
                //print!(".");
            } else if paths.contains(&(i, j)) {
                print!("#");
            } else if inner.contains(&(i, j)) {
                print!("*")
            } else {
                print!("{}", map[&(i, j)]._pipe)
            }
        }
        println!("");
    }
}
