use clap::Parser;
use lazy_static::lazy_static;
use num::Integer;
use regex::bytes::Regex;
use std::{collections::HashMap, fs::read, iter::repeat, str::from_utf8};

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
    let re = Regex::new(r"^[RL]+\n|[A-Z0-9]{3}|\n").unwrap();
    let tok = &mut re.find_iter(&IN).map(|b| from_utf8(b.as_bytes()).unwrap());
    let mut dirs = tok.next().unwrap().chars().collect::<Vec<_>>();
    dirs.truncate(dirs.len() - 1); // chomp newline

    let mut map = HashMap::new();
    while let Some(_newline) = tok.next() {
        let start = tok.next().unwrap();
        map.insert(start, (tok.next().unwrap(), tok.next().unwrap()));
    }

    let mut pos = "AAA";
    let mut step = 0;
    let mut ins = repeat(0..dirs.len()).flatten();
    while pos != "ZZZ" {
        let (l, r) = map[pos];
        let i = ins.next().unwrap();
        pos = dirs[i].eq(&'L').then(|| l).or_else(|| Some(r)).unwrap();
        step += 1;
    }
    println!("{}", step);
}

fn part2() {
    let re = Regex::new(r"^[RL]+\n|[A-Z0-9]{3}|\n").unwrap();
    let tok = &mut re.find_iter(&IN).map(|b| from_utf8(b.as_bytes()).unwrap());
    let mut dirs = tok.next().unwrap().chars().collect::<Vec<_>>();
    dirs.truncate(dirs.len() - 1); // chomp newline

    let mut map = HashMap::new();
    let mut pos = vec![];
    while let Some(_newline) = tok.next() {
        let start = tok.next().unwrap();
        start.ends_with("A").then(|| pos.push(start));
        map.insert(start, (tok.next().unwrap(), tok.next().unwrap()));
    }

    let mut zees = vec![0u128; pos.len()];
    for idx in 0..pos.len() {
        let mut pos = pos[idx];
        let mut ins = repeat(0..dirs.len()).flatten();
        while !pos.ends_with("Z") {
            let (l, r) = map[pos];
            let i = ins.next().unwrap();
            pos = dirs[i].eq(&'L').then(|| l).or_else(|| Some(r)).unwrap();
            zees[idx] += 1;
        }
    }

    println!("{:?}", zees.iter().fold(1, |acc, z| acc.lcm(z)));
}
