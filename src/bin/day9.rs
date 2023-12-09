use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{fs::read, str::from_utf8};

#[derive(Parser, Debug)]
#[command(author="Vampire Exec", version="0.0", about=format!("solution for {}", file!()), long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
    static ref IN: Vec<u8> = ARGS.input.as_ref().map_or(vec![], |p| read(p).unwrap());
}

fn main() {
    let re = Regex::new(r"-?\d+|\n").unwrap();
    let toks = &mut re
        .find_iter(&IN)
        .map(|t| from_utf8(t.as_bytes()).unwrap())
        .peekable();
    let mut sum1 = 0;
    let mut sum2 = 0;
    while toks.peek().is_some() {
        let s = toks
            .map_while(|t| t.ne("\n").then(|| t.parse().unwrap()))
            .collect();
        sum1 += last_in_seq(&s);
        sum2 += first_in_seq(&s);
    }
    println!("1: {}", sum1);
    println!("2: {}", sum2);
}

fn first_in_seq(s: &Vec<i128>) -> i128 {
    if s.iter().all(|n| *n == 0) {
        return 0;
    }
    let d = first_in_seq(
        &s.iter()
            .zip(s.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>(),
    );
    s.first().unwrap() - d
}

fn last_in_seq(seq: &Vec<i128>) -> i128 {
    if seq.iter().all(|n| *n == 0) {
        return 0;
    }
    let d = last_in_seq(
        &seq.iter()
            .zip(seq.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>(),
    );
    seq.last().unwrap() + d
}
