use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{collections::HashMap, fs::read};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct _Node<'a> {
    label: &'a [u8],
    wires: Vec<&'a [u8]>,
}

fn part1() {
    let re = Regex::new("\\w+|\n").unwrap();
    let mut toks = re.find_iter(&IN).map(|m| m.as_bytes());
    let mut connections = vec![];
    let mut id = 0;
    let mut lut = HashMap::new();
    while let Some(from) = toks.next() {
        let from = if lut.contains_key(&from) {
            lut[from]
        } else {
            let v = id;
            id += 1;
            lut.insert(from.clone(), v);
            v
        };
        while let Some(to) = toks.next() {
            if to == b"\n" {
                break;
            }
            let to = if lut.contains_key(&to) {
                lut[to]
            } else {
                let v = id;
                id += 1;
                lut.insert(to.clone(), v);
                v
            };
            let mut connection = [from, to];
            connection.sort();
            connections.push(connection);
        }
    }
    let num = id;
    connections.sort();

    let parts = (0..num).map(|i| Some(vec![i])).collect::<Vec<_>>();
    let parents = (0..num).map(|i| (i, i)).collect::<HashMap<_, _>>();
    let mut cache: HashMap<usize, (Vec<_>, HashMap<_, _>)> = HashMap::new();
    for i in 0..(connections.len() - 2) {
        println!("{i:4}");
        for j in (i + 1)..(connections.len() - 1) {
            for k in (j + 1)..connections.len() {
                let exclude = [i, j, k];

                let (mut parts, mut parents) = if let Some((parts, parents)) = cache.get(&(k + 1)) {
                    (parts.clone(), parents.clone())
                } else {
                    let mut parts = parts.clone();
                    let mut parents = parents.clone();

                    for cn in (k + 1)..connections.len() {
                        let from = parents[&connections[cn][0]];
                        let to = parents[&connections[cn][1]];
                        if from == to {
                            continue;
                        }

                        let to_part = parts[to].take();
                        for child in to_part.as_ref().unwrap() {
                            parents.insert(*child, from);
                        }
                        parts[from].as_mut().unwrap().append(&mut to_part.unwrap());
                    }

                    if !cache.contains_key(&(k + 1)) {
                        cache.insert(k + 1, (parts.clone(), parents.clone()));
                    }
                    (parts, parents)
                };

                for cn in 0..k {
                    if exclude.contains(&cn) {
                        continue;
                    }

                    let from = parents[&connections[cn][0]];
                    let to = parents[&connections[cn][1]];
                    if from == to {
                        continue;
                    }

                    let to_part = parts[to].take();
                    for child in to_part.as_ref().unwrap() {
                        parents.insert(*child, from);
                    }
                    parts[from].as_mut().unwrap().append(&mut to_part.unwrap());
                }

                let parts = parts.into_iter().flatten().collect::<Vec<_>>();

                if parts.len() > 1 {
                    let mut fac = 1;
                    for part in &parts {
                        fac *= part.len();
                    }
                    println!("{fac} {parts:?}");
                    return;
                }
            }
        }
    }
}

fn part2() {}
