use clap::{self, Parser};
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read,
    str::from_utf8,
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
    static ref INPUT: Vec<u8> = ARGS.input.as_ref().map_or(vec![], |p| read(p).unwrap());
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
        let Some(_game) = tokens.next() else {
            break 'parse;
        };
        let winners = (&mut tokens)
            .map_while(|t| {
                if t.as_bytes() != b"|" {
                    Some(from_utf8(t.into()).unwrap().parse().unwrap())
                } else {
                    None
                }
            })
            .collect::<HashSet<i64>>();
        let count = (&mut tokens)
            .map_while(|t| {
                if t.as_bytes() != b"\n" {
                    Some(from_utf8(t.into()).unwrap().parse().unwrap())
                } else {
                    None
                }
            })
            .filter(|t| winners.contains(t))
            .count();
        if count > 0 {
            sum += 1 << (count - 1);
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
        let Some(game) = tokens.next() else {
            break 'parse;
        };
        let game = from_utf8(game.as_bytes())?.parse::<i64>()?;
        let copies = copy_map.remove(&game).unwrap_or(1i64);
        count += copies;
        let winners = (&mut tokens)
            .map_while(|t| {
                if t.as_bytes() != b"|" {
                    Some(from_utf8(t.into()).unwrap().parse().unwrap())
                } else {
                    None
                }
            })
            .collect::<HashSet<i64>>();
        let mut wins = 0;
        (&mut tokens)
            .map_while(|t| {
                if t.as_bytes() != b"\n" {
                    if winners.contains(&from_utf8(t.into()).unwrap().parse().unwrap()) {
                        wins += 1;
                        *((&mut copy_map).entry(game + wins).or_insert(1)) += copies;
                    }
                    Some(())
                } else {
                    None
                }
            })
            .count();
    }
    println!("{}", count);
    Ok(())
}
