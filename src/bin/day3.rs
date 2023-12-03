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
    locs: Vec<usize>,
}

lazy_static! {
    static ref ADJ: Vec<(i64, i64)> = (-1..=1)
        .flat_map(|i| (-1..=1).map(move |j| (i, j)))
        .collect();
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref SYM_RE: Regex = Regex::new(r"[^.\d\n]").unwrap();
    static ref GEAR_RE: Regex = Regex::new(r"\*").unwrap();
}

fn part1() -> Result<(), Box<dyn Error>> {
    let width = INPUT.find("\n").unwrap() as i64 + 1;
    let parts = NUM_RE
        .find_iter(&INPUT)
        .map(|m| Part {
            num: m.as_str().parse::<i64>().unwrap(),
            locs: m.range().collect(),
        })
        .collect::<Vec<Part>>();
    let syms: Vec<usize> = SYM_RE.find_iter(&INPUT).map(|m| m.start()).collect();
    let mut sum = 0;
    'search: for part in parts {
        for s in part.locs {
            for (di, dj) in ADJ.iter() {
                if syms.contains(&((s as i64 + di + dj * width) as usize)) {
                    sum += part.num;
                    continue 'search;
                }
            }
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
            locs: m.range().collect(),
        })
        .collect::<Vec<Part>>();
    let parts_lut: HashMap<usize, &Part> = parts
        .iter()
        .flat_map(|p| p.locs.iter().map(move |s| (*s, p)))
        .collect();
    let gears: Vec<usize> = GEAR_RE.find_iter(&INPUT).map(|m| m.start()).collect();
    let mut sum = 0;
    for s in &gears {
        let adj: HashSet<&Part> = ADJ
            .iter()
            .filter_map(|(di, dj)| parts_lut.get(&((*s as i64 + *di + *dj * width) as usize)))
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
