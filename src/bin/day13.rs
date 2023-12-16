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
    let re = Regex::new(r"[^\n]+|\n\n").unwrap();
    let toks = &mut re.find_iter(&IN).map(|t| t.as_bytes()).peekable();
    let mut sum = 0;
    while toks.peek().is_some() {
        let rows = toks.take_while(|t| t != b"\n\n").collect::<Vec<_>>();
        for j in 0..(rows.len() - 1) {
            let mut a = rows[0..=j].to_vec();
            let mut b = rows[(j + 1)..].to_vec();
            a.reverse();
            if a.len() < b.len() {
                b.truncate(a.len());
            } else {
                a.truncate(b.len())
            }

            for (i, (a, b)) in a.iter().zip(b.iter()).enumerate() {
                println!("{}: {} {}", i, from_utf8(a).unwrap(), from_utf8(b).unwrap());
            }
            println!("");
            if a == b {
                sum += 100 * j;
            }
        }
    }

    println!("{sum}");
}

fn part2() {}
