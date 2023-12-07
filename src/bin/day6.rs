use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{fs::read, str::from_utf8};

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
    let re = Regex::new(r"\w+:|\d+|\n").unwrap();
    let toks = &mut re.find_iter(&IN).map(|t| from_utf8(t.as_bytes()).unwrap());
    toks.next(); // label
    let times = toks.map_while(|t| t.ne("\n").then(|| t.parse().unwrap()));
    let times = times.collect::<Vec<_>>();
    toks.next(); // label
    let dists = toks.map_while(|t| t.ne("\n").then(|| t.parse().unwrap()));
    let dists = dists.collect::<Vec<_>>();
    let mut result = 1;
    for (t, d) in times.iter().zip(dists.iter()) {
        result *= (0..*t).filter(|c| c * (t - c) > *d).count();
    }
    println!("{}", result)
}

fn part2() {
    let re = Regex::new(r"\w+:|\d+|\n").unwrap();
    let toks = &mut re.find_iter(&IN).map(|t| from_utf8(t.as_bytes()).unwrap());
    toks.next(); // label
    let time = toks.map_while(|t| t.ne("\n").then_some(t));
    let time = time.collect::<String>().parse().unwrap();
    toks.next(); // label
    let dist = toks.map_while(|t| t.ne("\n").then_some(t));
    let dist = dist.collect::<String>().parse().unwrap();
    println!("{}", (0i64..time).filter(|c| c * (time - c) > dist).count());
}
