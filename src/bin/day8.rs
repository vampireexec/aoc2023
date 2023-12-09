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
    let dirs = tok
        .next()
        .unwrap()
        .chars()
        .map_while(|t| t.ne(&'\n').then(|| t))
        .collect::<Vec<_>>();

    println!("{:?}", dirs);
    let mut map = HashMap::new();
    while let Some(_newline) = tok.next() {
        map.insert(
            tok.next().unwrap(),
            (tok.next().unwrap(), tok.next().unwrap()),
        );
    }
    let mut pos = "AAA";
    let mut count = 0;
    for i in repeat(0..dirs.len()).flatten() {
        let node = map[pos];
        println!("{} -> {:?}, {}:{}", pos, node, i, dirs[i]);
        pos = dirs[i]
            .eq(&'R')
            .then(|| node.1)
            .or_else(|| Some(node.0))
            .unwrap();
        count += 1;
        if pos == "ZZZ" {
            break;
        }
    }
    println!("{}", count);
}

fn part2() {
    let re = Regex::new(r"^[RL]+\n|[A-Z0-9]{3}|\n").unwrap();
    let tok = &mut re.find_iter(&IN).map(|b| from_utf8(b.as_bytes()).unwrap());
    let dirs = tok
        .next()
        .unwrap()
        .chars()
        .map_while(|t| t.ne(&'\n').then(|| t))
        .collect::<Vec<_>>();

    let mut map = HashMap::new();
    let mut pos = vec![];
    while let Some(_newline) = tok.next() {
        let start = tok.next().unwrap();
        start.ends_with("A").then(|| pos.push(start));
        map.insert(start, (tok.next().unwrap(), tok.next().unwrap()));
    }

    let mut paths = vec![];
    for idx in 0..pos.len() {
        let mut cache = HashMap::new();
        let mut offset = 0i128;
        let mut pos = pos[idx];
        let mut zee = 0i128;
        let mut cycle_start = 0i128;
        for i in repeat(0..dirs.len()).flatten() {
            if let Some(start_offset) = cache.get(&(pos, i)) {
                cycle_start = *start_offset;
                break;
            }

            if pos.ends_with("Z") {
                zee = offset;
            }

            cache.insert((pos, i), offset);
            let c = map[pos];
            pos = dirs[i].eq(&'R').then(|| c.1).or_else(|| Some(c.0)).unwrap();
            offset += 1;
        }
        let cycle_len = zee - cycle_start + 1;
        paths.push((zee, cycle_len));
    }

    let mut lcm = paths[0];
    let mut limit = 0;
    for idx in 1..paths.len() {
        let mut sync = [lcm, paths[idx]];
        loop {
            let next = sync
                .iter()
                .map(|(off, cycle)| (off + cycle))
                .enumerate()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap()
                .0;
            sync[next].0 += sync[next].1;
            if sync[0].0 == sync[1].0 {
                lcm = sync[0];
                println!("{}", sync[0].0);
                break;
            }
            limit += 1;
            if limit > 50000000 {
                println!("{:?}", sync);
                break;
            }
        }
    }

    for path in &paths {
        println!("{:?}", path);
    }
}
