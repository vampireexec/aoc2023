use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{collections::HashMap, fs::read};

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
fn part1() {
    let re = Regex::new("[^\n]+").unwrap();
    let mut map = re
        .find_iter(&IN)
        .map(|t| Vec::from(t.as_bytes()))
        .collect::<Vec<_>>();
    let w = map[0].len();
    let h = map.len();

    let mut sum = 0;
    for j in 0..h {
        for i in 0..w {
            if map[j][i] == b'O' {
                map[j][i] = b'.';
                let mut dest = (i, j);
                for dj in (0..=j).rev() {
                    if dj == 0 || map[dj - 1][i] != b'.' {
                        dest = (i, dj);
                        break;
                    }
                }
                map[dest.1][dest.0] = b'O';
                sum += h - dest.1;
            }
        }
    }
    println!("{sum}");
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

fn part2() {
    let re = Regex::new("[^\n]+").unwrap();
    let mut map = re
        .find_iter(&IN)
        .map(|t| Vec::from(t.as_bytes()))
        .collect::<Vec<_>>();
    let w = map[0].len();
    let h = map.len();

    let mut cache: HashMap<(Vec<Vec<u8>>, Dir), Vec<Vec<u8>>> = HashMap::new();
    let mut remaining = 1000000000;
    while remaining > 0 {
        remaining -= 1;
        if spin(&mut cache, &mut map, h, w) {
            break;
        }
    }

    let start = map.clone();
    let mut steps = 1;
    remaining -= 1;
    spin(&mut cache, &mut map, h, w);
    while remaining > 0 && start != map {
        steps += 1;
        remaining -= 1;
        spin(&mut cache, &mut map, h, w);
    }

    remaining = remaining % steps;

    while remaining > 0 {
        remaining -= 1;
        spin(&mut cache, &mut map, h, w);
    }

    for j in 0..h {
        for i in 0..w {
            print!("{}", map[j][i] as char);
        }
        println!("");
    }

    let mut sum = 0;
    for j in 0..h {
        for i in 0..w {
            if map[j][i] == b'O' {
                sum += h - j;
            }
        }
    }

    println!("{sum}");
}

fn spin(
    cache: &mut HashMap<(Vec<Vec<u8>>, Dir), Vec<Vec<u8>>>,
    map: &mut Vec<Vec<u8>>,
    h: usize,
    w: usize,
) -> bool {
    let mut cached = false;

    // north
    if let Some(next) = cache.get(&(map.clone(), Dir::North)) {
        cached = true;
        *map = next.clone();
    } else {
        let before = map.clone();
        for j in 0..h {
            for i in 0..w {
                if map[j][i] == b'O' {
                    map[j][i] = b'.';
                    let mut dest = (i, j);
                    for dj in (0..=j).rev() {
                        if dj == 0 || map[dj - 1][i] != b'.' {
                            dest = (i, dj);
                            break;
                        }
                    }
                    map[dest.1][dest.0] = b'O';
                }
            }
        }
        // for j in 0..h {
        //     for i in 0..w {
        //         print!("{}", map[j][i] as char);
        //     }
        //     println!("");
        // }
        cache.insert((before, Dir::North), map.clone());
    }

    // west
    if let Some(next) = cache.get(&(map.clone(), Dir::North)) {
        cached = true;
        *map = next.clone();
    } else {
        let before = map.clone();
        for j in 0..h {
            for i in 0..w {
                if map[j][i] == b'O' {
                    map[j][i] = b'.';
                    let mut dest = (i, j);
                    for di in (0..=i).rev() {
                        if di == 0 || map[j][di - 1] != b'.' {
                            dest = (di, j);
                            break;
                        }
                    }
                    map[dest.1][dest.0] = b'O';
                }
            }
        }
        cache.insert((before, Dir::West), map.clone());
    }

    // south
    if let Some(next) = cache.get(&(map.clone(), Dir::South)) {
        cached = true;
        *map = next.clone();
    } else {
        let before = map.clone();
        for j in (0..h).rev() {
            for i in 0..w {
                if map[j][i] == b'O' {
                    map[j][i] = b'.';
                    let mut dest = (i, j);
                    for dj in j..h {
                        if dj == (h - 1) || map[dj + 1][i] != b'.' {
                            dest = (i, dj);
                            break;
                        }
                    }
                    map[dest.1][dest.0] = b'O';
                }
            }
        }
        cache.insert((before, Dir::South), map.clone());
    }

    // east
    if let Some(next) = cache.get(&(map.clone(), Dir::North)) {
        cached = true;
        *map = next.clone();
    } else {
        let before = map.clone();
        for j in 0..h {
            for i in (0..w).rev() {
                if map[j][i] == b'O' {
                    map[j][i] = b'.';
                    let mut dest = (i, j);
                    for di in i..w {
                        if di == (w - 1) || map[j][di + 1] != b'.' {
                            dest = (di, j);
                            break;
                        }
                    }
                    map[dest.1][dest.0] = b'O';
                }
            }
        }
        cache.insert((before, Dir::East), map.clone());
    }

    return cached;
}
