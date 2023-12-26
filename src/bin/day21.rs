use aoc2023::advent_point::{Dir, Point};
use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read,
};

#[derive(Parser, Debug)]
#[command(author="Vampire Exec", version="0.0", about=format!("solution for {}", file!()), long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,
    #[arg(long)]
    part: u8,
    #[arg(long)]
    steps: i128,
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
    let map = re.find_iter(&IN).map(|m| m.as_bytes()).collect::<Vec<_>>();

    let mut start = None;
    let mut rocks = HashMap::new();
    for j in 0..(map.len() as i128) {
        for i in 0..(map[j as usize].len() as i128) {
            match map[j as usize][i as usize] {
                b'S' => {
                    start = Some(Point::new(i, j));
                }
                b'#' => {
                    rocks.insert(Point::new(i, j), i128::MIN);
                }
                _ => (),
            }
        }
    }
    let start = start.unwrap();
    let mut dest = HashSet::new();

    let mut stack = vec![(start, 0)];
    let mut cache = HashSet::new();

    while let Some((curr, steps)) = stack.pop() {
        if steps == ARGS.steps {
            dest.insert(curr);
            continue;
        }

        for adj in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let next = curr + adj.as_point();
            if !(0..map.len() as i128).contains(&next.j)
                || !(0..map[0].len() as i128).contains(&next.i)
                || rocks.contains_key(&next)
                || cache.contains(&(next, steps + 1))
            {
                continue;
            }
            cache.insert((next, steps + 1));
            stack.push((next, steps + 1));
        }
    }

    for j in 0..(map.len() as i128) {
        for i in 0..(map[j as usize].len() as i128) {
            let p = Point::new(i, j);
            if dest.contains(&p) {
                print!("O");
            } else {
                print!("{}", map[j as usize][i as usize] as char);
            }
        }
        println!("");
    }

    let count = dest.len();
    println!("count={count}");
}

fn part2() {
    let re = Regex::new("[^\n]+").unwrap();
    let map = re.find_iter(&IN).map(|m| m.as_bytes()).collect::<Vec<_>>();

    let mut start = None;
    let mut rocks = HashMap::new();
    for j in 0..(map.len() as i128) {
        for i in 0..(map[j as usize].len() as i128) {
            match map[j as usize][i as usize] {
                b'S' => {
                    start = Some(Point::new(i, j));
                }
                b'#' => {
                    rocks.insert(Point::new(i, j), i128::MIN);
                }
                _ => (),
            }
        }
    }
    let start = start.unwrap();
    let mut dest = HashSet::new();

    let mut stack = vec![(vec![start], HashMap::new(), 0)];
    let mut cache = HashSet::new();

    while let Some((path, mod_visited, steps)) = stack.pop() {
        let curr = *path.last().unwrap();
        if steps == ARGS.steps {
            dest.insert(curr);
            continue;
        }

        for adj in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let next = curr + adj.as_point();
            let next_steps = steps + 1;

            let mod_next = Point::new(
                next.i.rem_euclid(map[0].len() as i128),
                next.j.rem_euclid(map.len() as i128),
            );

            if rocks.contains_key(&mod_next) || cache.contains(&(next, next_steps)) {
                continue;
            }

            if mod_visited.contains_key(&mod_next) && ARGS.steps % next_steps == 0 {}

            let mut path = path.clone();
            let mut mod_visited = mod_visited.clone();

            path.push(next);
            cache.insert((next, next_steps));
            mod_visited.insert(mod_next, next_steps);
            stack.push((path, mod_visited, next_steps));
        }
    }

    for j in 0..(map.len() as i128) {
        for i in 0..(map[j as usize].len() as i128) {
            let p = Point::new(i, j);
            if dest.contains(&p) {
                print!("O");
            } else {
                print!("{}", map[j as usize][i as usize] as char);
            }
        }
        println!("");
    }

    let count = dest.len();
    println!("count={count}");
}
