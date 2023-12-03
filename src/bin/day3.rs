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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Part {
    num: i64,
    locs: Vec<(i64, i64)>,
}

lazy_static! {
    static ref ADJ: Vec<(i64, i64)> = (-1..=1)
        .flat_map(|i| (-1..=1).map(move |j| (i, j)))
        .collect();
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref SYM_RE: Regex = Regex::new(r"[^.\d\n]").unwrap();
    static ref GEAR_RE: Regex = Regex::new(r"\*").unwrap();
}

trait Grided {
    fn as_ij(&self, width: i64) -> (i64, i64);
}

impl Grided for usize {
    fn as_ij(&self, width: i64) -> (i64, i64) {
        (*self as i64 % width, *self as i64 / width)
    }
}

fn part1() -> Result<(), Box<dyn Error>> {
    let width = INPUT.find("\n").unwrap() as i64 + 1;
    let mut parts = NUM_RE
        .find_iter(&INPUT)
        .map(|m| Part {
            num: m.as_str().parse::<i64>().unwrap(),
            locs: m.range().map(|s| s.as_ij(width)).collect(),
        })
        .collect::<Vec<Part>>();
    let syms: Vec<(i64, i64)> = SYM_RE
        .find_iter(&INPUT)
        .map(|m| m.start().as_ij(width))
        .collect();
    let mut sum = 0;
    for part in &mut parts {
        if part.locs.iter().any(|(i, j)| {
            let syms = &syms;
            ADJ.iter()
                .any(move |(di, dj)| syms.contains(&(i + di, j + dj)))
        }) {
            sum += part.num;
        }
    }
    println!("{}", sum);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let width = INPUT.find("\n").unwrap() as i64 + 1;
    let parts = NUM_RE
        .find_iter(&INPUT)
        .map(|m| Part {
            num: m.as_str().parse::<i64>().unwrap(),
            locs: m.range().map(|s| s.as_ij(width)).collect(),
        })
        .collect::<Vec<Part>>();
    let parts_lut: HashMap<(i64, i64), &Part> = parts
        .iter()
        .flat_map(|p| p.locs.iter().map(move |l| (*l, p)))
        .collect();
    let gears: Vec<(i64, i64)> = GEAR_RE
        .find_iter(&INPUT)
        .map(|m| m.start().as_ij(width))
        .collect();
    let mut sum = 0;
    for (i, j) in &gears {
        let adj: HashSet<&Part> = ADJ
            .iter()
            .filter_map(|(di, dj)| parts_lut.get(&(i + *di, j + *dj)))
            .cloned()
            .collect();
        if adj.len() == 2 {
            let mut pair = adj.iter();
            sum += pair.next().unwrap().num * pair.next().unwrap().num;
        }
    }
    println!("{}", sum);
    Ok(())
}
