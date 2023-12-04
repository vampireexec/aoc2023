use clap::{self, Parser};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
};

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
    static ref INPUT: String = ARGS
        .input
        .as_ref()
        .map(|p| read_to_string(p).unwrap())
        .unwrap_or("".into());
}

fn main() {
    if ARGS.part == 1 {
        part1().ok();
    } else {
        part2().ok();
    }
}
fn part1() -> Result<(), Box<dyn Error>> {
    let tokens_re = Regex::new(r"\d+|\||\n").unwrap();
    let mut tokens = tokens_re.find_iter(&INPUT);
    let mut sum = 0i64;
    'parse: loop {
        let mut winners = HashSet::new();

        let Some(_game) = tokens.next() else {
            break 'parse;
        };

        loop {
            if let Some(token) = tokens.next() {
                if token.as_str() == "|" || token.as_str() == "\n" {
                    break;
                } else {
                    winners.insert(token.as_str().parse::<i64>().unwrap());
                }
            } else {
                break 'parse;
            }
        }
        let mut match_count = 0;
        loop {
            if let Some(token) = tokens.next() {
                if token.as_str() == "\n" {
                    break;
                } else {
                    let n = &token.as_str().parse::<i64>().unwrap();
                    if winners.contains(n) {
                        match_count += 1
                    }
                }
            } else {
                break 'parse;
            }
        }

        if match_count > 0 {
            sum += 2i64.pow(match_count - 1);
        }
    }

    println!("{}", sum);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let tokens_re = Regex::new(r"\d+|\||\n").unwrap();
    let mut tokens = tokens_re.find_iter(&INPUT);
    let mut copy_map = HashMap::new();
    let mut count = 0;
    'parse: loop {
        let mut winners = HashSet::new();

        let Some(game) = tokens.next() else {
            break 'parse;
        };
        let game = game.as_str().parse::<i64>()?;
        let copies = *copy_map.entry(game).or_insert(1i64);
        count += copies;
        loop {
            if let Some(token) = tokens.next() {
                if token.as_str() == "|" || token.as_str() == "\n" {
                    break;
                } else {
                    winners.insert(token.as_str().parse::<i64>()?);
                }
            } else {
                break 'parse;
            }
        }
        let mut match_count = 0;
        loop {
            if let Some(token) = tokens.next() {
                if token.as_str() == "\n" {
                    break;
                } else {
                    let n = &token.as_str().parse::<i64>()?;
                    if winners.contains(n) {
                        match_count += 1
                    }
                }
            } else {
                break 'parse;
            }
        }

        for i in 1..=match_count {
            *copy_map.entry(game + i).or_insert(1) += copies;
        }
    }

    println!("{}", count);
    Ok(())
}
