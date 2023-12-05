use clap::Parser;
use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read,
    str::from_utf8,
};

#[derive(Parser, Debug)]
#[command(author="Vampire Exec", version="0.0", about=format!("solution for {}", file!()), long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = args.input.as_ref().map_or(vec![], |p| read(p).unwrap());
    let tokens_re = Regex::new(r"\d+|\||\n").unwrap();
    let tokens = &mut tokens_re.find_iter(&input);
    let mut sum1 = 0i64;
    let mut sum2 = 0i64;
    let mut copy_map = HashMap::new();
    while let Some(game) = tokens.next() {
        let game = from_utf8(game.as_bytes()).unwrap().parse::<i64>().unwrap();
        let copies = copy_map.remove(&game).unwrap_or(1i64);
        sum2 += copies;
        let winners = tokens
            .map_while(|t| {
                if t.as_bytes() != b"|" {
                    Some(from_utf8(t.into()).unwrap().parse().unwrap())
                } else {
                    None
                }
            })
            .collect::<HashSet<i64>>();
        let mut wins = 0;
        let count = tokens
            .map_while(|t| {
                if t.as_bytes() != b"\n" {
                    if winners.contains(&from_utf8(t.into()).unwrap().parse().unwrap()) {
                        wins += 1;
                        *copy_map.entry(game + wins).or_insert(1) += copies;
                    }
                    Some(from_utf8(t.into()).unwrap().parse().unwrap())
                } else {
                    None
                }
            })
            .filter(|t| winners.contains(t))
            .count();
        if count > 0 {
            sum1 += 1 << (count - 1);
        }
    }
    println!("1) {} 2) {}", sum1, sum2);
}
