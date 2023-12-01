use regex::Regex;
use std::{collections::HashMap, fs};

use clap::{self, Parser};

#[derive(Parser, Debug)]
#[command(author="Vampire Exec", version="0.0", about=format!("solution for {}", file!()), long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,
    #[arg(long)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    if args.part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let args = Args::parse();
    let input = fs::read(args.input.unwrap()).unwrap();
    let mut buf = vec![];
    let mut sum = 0u64;
    for line in input.split(|b| *b as char == '\n') {
        buf.clear();
        for b in line {
            if b.is_ascii_digit() {
                buf.push(*b);
            }
        }

        let backing = if buf.len() >= 2 {
            vec![buf[0], buf[buf.len() - 1]]
        } else if buf.len() == 1 {
            vec![buf[0], buf[0]]
        } else {
            panic!("Bad input")
        };

        let num = String::from_utf8(backing).unwrap();
        sum += u64::from_str_radix(&num, 10).unwrap();
    }

    println!("{}", sum);
}

fn part2() {
    let args = Args::parse();
    let input = String::from_utf8(fs::read(args.input.unwrap()).unwrap()).unwrap();

    let lut = HashMap::from([
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ]);

    let keys = lut.keys().cloned().collect::<Vec<&str>>().join("|");

    let pattern = format!("{}", keys);
    let re = Regex::new(&pattern).unwrap();
    let rev_re = Regex::new(&pattern.chars().rev().collect::<String>()).unwrap();
    let mut sum = 0;
    for line in input.split("\n") {
        let first = re.find(line).unwrap().as_str().to_string();
        let last = rev_re
            .find(&line.chars().rev().collect::<String>())
            .unwrap()
            .as_str()
            .chars()
            .rev()
            .collect::<String>();
        sum += *lut.get(first.as_str()).unwrap_or(&0) * 10 + *lut.get(last.as_str()).unwrap_or(&0);
    }
    println!("{}", sum);
}
