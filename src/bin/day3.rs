use clap::{self, Parser};
use lazy_static::lazy_static;
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
    valid: Option<bool>,
}

fn part1() -> Result<(), Box<dyn Error>> {
    let lines = INPUT.split_ascii_whitespace().collect::<Vec<&str>>();

    let mut i = 0;
    let mut j = 0;
    let mut parts = vec![];
    let mut syms = HashSet::new();

    while j < lines.len() {
        let line = lines[j].as_bytes();
        while i < line.len() {
            if !line[i].is_ascii_digit() {
                if line[i] as char != '.' {
                    syms.insert((i as i64, j as i64));
                }
                i += 1;
                continue;
            }

            let mut buf = String::new();
            let mut locs = vec![];
            while i < line.len() && line[i].is_ascii_digit() {
                buf.push(line[i] as char);
                locs.push((i as i64, j as i64));
                i += 1;
            }

            parts.push(Part {
                num: buf.parse::<i64>()?,
                locs,
                valid: None,
            });
        }
        i = 0;
        j += 1;
    }

    let mut sum = 0;
    'validate: for part in &mut parts {
        for loc in &part.locs {
            for dj in -1..=1 {
                for di in -1..=1 {
                    if syms.contains(&(loc.0 + di as i64, loc.1 + dj as i64)) {
                        part.valid.replace(true);
                        sum += part.num;
                        continue 'validate;
                    }
                }
            }
        }
        part.valid.replace(false);
    }

    println!("{}", sum);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let lines = INPUT.split_ascii_whitespace().collect::<Vec<&str>>();

    let mut i = 0;
    let mut j = 0;
    let mut parts = vec![];
    let mut part_lut = HashMap::new();
    let mut asters = HashSet::new();

    while j < lines.len() {
        let line = lines[j].as_bytes();
        while i < line.len() {
            if !line[i].is_ascii_digit() {
                if line[i] as char == '*' {
                    asters.insert((i as i64, j as i64));
                }
                i += 1;
                continue;
            }

            let mut buf = String::new();
            let mut locs = vec![];
            while i < line.len() && line[i].is_ascii_digit() {
                buf.push(line[i] as char);
                locs.push((i as i64, j as i64));
                i += 1;
            }
            let part = Part {
                num: buf.parse::<i64>()?,
                locs,
                valid: None,
            };

            for loc in &part.locs {
                part_lut.insert(loc.clone(), part.clone());
            }
            parts.push(part);
        }
        i = 0;
        j += 1;
    }

    let mut sum = 0;
    for aster in asters {
        let mut adj = HashSet::new();
        for dj in -1..=1 {
            for di in -1..=1 {
                if let Some(part) = part_lut.get(&(aster.0 + di as i64, aster.1 + dj as i64)) {
                    adj.insert(part);
                }
            }
        }

        if adj.len() == 2 {
            let mut parts = adj.iter();
            sum += parts.next().unwrap().num * parts.next().unwrap().num;
        }
    }

    println!("{}", sum);
    Ok(())
}
