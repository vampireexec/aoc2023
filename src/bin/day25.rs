use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read,
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
struct Node<'a> {
    label: &'a [u8],
    wires: Vec<&'a [u8]>,
}

fn part1() {
    let re = Regex::new("\\w+|\n").unwrap();
    let mut toks = re.find_iter(&IN).map(|m| m.as_bytes());
    let mut connections = vec![];
    let mut wire_table = HashMap::new();
    while let Some(from) = toks.next() {
        while let Some(to) = toks.next() {
            if to == b"\n" {
                break;
            }
            let node = wire_table.entry(from).or_insert_with(|| Node {
                label: from,
                wires: vec![],
            });
            node.wires.push(to);

            let node = wire_table.entry(to).or_insert_with(|| Node {
                label: to,
                wires: vec![],
            });
            node.wires.push(from);
            connections.push(HashSet::from([from, to]));
        }
    }

    let components = wire_table.keys().cloned().collect::<HashSet<_>>();
    let mut result = vec![];

    for i in 0..(connections.len() - 2) {
        for j in (i + 1)..(connections.len() - 1) {
            for k in (j + 1)..connections.len() {
                let exclude = [&connections[i], &connections[j], &connections[k]];
                println!("{i} {j} {k}");

                let mut remaining = components.clone();
                let mut parts = vec![];

                while let Some(start) = remaining.iter().cloned().next() {
                    let mut part = HashSet::new();
                    let mut stack = vec![start];
                    while let Some(curr) = stack.pop() {
                        remaining.remove(curr);
                        part.insert(curr);
                        let node = &wire_table[curr];
                        for wire in node.wires.iter().cloned() {
                            if exclude.contains(&&HashSet::from([curr, wire])) {
                                continue;
                            }

                            if !part.contains(wire) {
                                stack.push(wire);
                            }
                        }
                    }

                    parts.push(part);
                }

                if parts.len() != 1 {
                    result.push(format!(
                        "{i} {j} {k} - {}",
                        parts
                            .iter()
                            .map(|s| s.len().to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    ));
                }
            }
        }
    }

    for r in result {
        println!("{}", r);
    }
}

fn part2() {}
