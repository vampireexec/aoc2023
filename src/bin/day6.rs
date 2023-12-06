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
    static ref INPUT: Vec<u8> = ARGS.input.as_ref().map_or(vec![], |p| read(p).unwrap());
}

fn main() {
    if ARGS.part == 1 {
        part1();
    } else {
        part2();
    }
}
fn part1() {
    let tokens_re = Regex::new(r"\w+:|\d+|\n").unwrap();
    let tokens = &mut tokens_re
        .find_iter(&INPUT)
        .map(|t| from_utf8(t.as_bytes()).unwrap());
    tokens.next(); // label
    let times: Vec<i64> = tokens
        .map_while(|t| if t != "\n" { t.parse().ok() } else { None })
        .collect();
    tokens.next(); // label
    let dists: Vec<i64> = tokens
        .map_while(|t| if t != "\n" { t.parse().ok() } else { None })
        .collect();

    assert_eq!(times.len(), dists.len());
    let mut result = 1;
    for i in 0..times.len() {
        let time = times[i];
        let dist = dists[i];
        let mut count = 0;
        for charge in 0..time {
            if charge * (time - charge) > dist {
                count += 1;
            }
        }
        result *= count
    }
    println!("{}", result)
}

fn part2() {
    let tokens_re = Regex::new(r"\w+:|\d+|\n").unwrap();
    let tokens = &mut tokens_re
        .find_iter(&INPUT)
        .map(|t| from_utf8(t.as_bytes()).unwrap());
    tokens.next(); // label
    let time: i64 = tokens
        .map_while(|t| if t != "\n" { Some(t) } else { None })
        .collect::<String>()
        .parse()
        .unwrap();
    tokens.next(); // label
    let dist: i64 = tokens
        .map_while(|t| if t != "\n" { Some(t) } else { None })
        .collect::<String>()
        .parse()
        .unwrap();

    let mut count = 0;
    for charge in 0..time {
        if charge * (time - charge) > dist {
            count += 1;
        }
    }
    println!("{}", count)
}
