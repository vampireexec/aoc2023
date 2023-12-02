use clap::{self, Parser};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs};

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
}

fn main() {
    if ARGS.part == 1 {
        part1();
    } else {
        part2();
    }
}

lazy_static! {
    static ref TOKENS_RE: Regex = Regex::new(r"Game \d+:|\d+|blue|green|red").unwrap();
}

fn get_counts(line: &str) -> HashMap<&str, i32> {
    let mut tokens = TOKENS_RE.find_iter(line);
    let mut counts = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    loop {
        let Some(token) = tokens.next() else {
            break;
        };

        if token.as_str().starts_with("Game") {
            continue;
        }

        let n = i32::from_str_radix(token.as_str(), 10).unwrap();
        let color = tokens.next().unwrap().as_str();
        if n > counts[color] {
            *counts.get_mut(color).unwrap() = n;
        }
    }
    counts
}

fn part1() {
    let input = fs::read_to_string(ARGS.input.as_ref().unwrap()).unwrap();
    let game_re = Regex::new(r"Game (\d+):").unwrap();
    let mut sum = 0;
    for line in input.split("\n") {
        let game =
            i32::from_str_radix(game_re.captures(line).unwrap().get(1).unwrap().as_str(), 10)
                .unwrap();
        let counts = get_counts(line);
        if counts["red"] <= 12 && counts["green"] <= 13 && counts["blue"] <= 14 {
            sum += game;
        }
    }
    println!("{}", sum);
}

fn part2() {
    let input = fs::read_to_string(ARGS.input.as_ref().unwrap()).unwrap();
    let mut sum = 0;
    for line in input.split("\n") {
        let counts = get_counts(line);
        sum += counts["red"] * counts["green"] * counts["blue"];
    }
    println!("{}", sum);
}
