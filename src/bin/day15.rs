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
    let re = Regex::new(r"[^,]+").unwrap();
    let mut toks = re.find_iter(&IN).map(|t| t.as_bytes());
    let mut sum = 0;
    while let Some(t) = toks.next() {
        sum += t.iter().fold(0u128, |a, n| ((a + *n as u128) * 17) % 256);
    }
    println!("{}", sum);
}

fn part2() {}
