use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{fs::read, iter::repeat_with, str::from_utf8};

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

fn hash(n: &[u8]) -> usize {
    n.iter().fold(0, |a, n| ((a + *n as usize) * 17) % 256)
}

fn part1() {
    let re = Regex::new(r"[^,]+").unwrap();
    let toks = re.find_iter(&IN).map(|t| t.as_bytes());
    println!("{}", toks.fold(0, |acc, t| acc + hash(t)));
}

fn part2() {
    let re = Regex::new(r"\w+|\d+|=|-").unwrap();
    let mut toks = re.find_iter(&IN).map(|t| t.as_bytes());
    let mut boxes: Vec<Vec<(&[u8], usize)>> = repeat_with(|| vec![]).take(256).collect::<Vec<_>>();
    while let Some(label) = toks.next() {
        let n = hash(label);
        match toks.next() {
            Some(b"=") => {
                let v: usize = from_utf8(toks.next().unwrap()).unwrap().parse().unwrap();
                if let Some(i) = (0..boxes[n].len()).find(|i| boxes[n][*i].0 == label) {
                    boxes[n][i] = (label, v);
                } else {
                    boxes[n].push((label, v));
                }
            }
            Some(b"-") => {
                if let Some(i) = (0..boxes[n].len()).find(|i| boxes[n][*i].0 == label) {
                    boxes[n].remove(i);
                }
            }
            _ => (),
        }
    }

    println!(
        "{}",
        boxes.iter().enumerate().fold(0, |sum, (n, b)| {
            sum + b
                .iter()
                .enumerate()
                .fold(0, |sum, (i, l)| sum + (1 + n) * (1 + i) * l.1)
        })
    );
}
