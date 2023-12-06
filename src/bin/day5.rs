use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{collections::BTreeMap, fs::read, iter::once, rc::Rc, str::from_utf8, usize};

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
        part1();
    } else {
        part2();
    }
}

type LookUp = BTreeMap<usize, (usize, usize)>;
fn get_maps(tokens: &mut dyn Iterator<Item = &str>) -> Vec<Rc<LookUp>> {
    let mut lookups = vec![];
    let tokens = &mut tokens.peekable();
    while tokens.peek().is_some() {
        let mut map = LookUp::new();
        tokens.next(); //label
        tokens.next(); //newline
        while let Some(dest) = tokens.next() {
            if dest == "\n" {
                break;
            }
            let dest: usize = dest.parse().unwrap();
            let source: usize = tokens.next().unwrap().parse().unwrap();
            let len: usize = tokens.next().unwrap().parse().unwrap();
            map.insert(source, (dest, len));
            tokens.next(); //newline
        }
        lookups.push(Rc::new(map));
    }
    lookups
}

fn part1() {
    let tokens_re = Regex::new(r"\w+:|\d+|\n").unwrap();
    let tokens = &mut tokens_re
        .find_iter(&INPUT)
        .map(|t| from_utf8(t.as_bytes()).unwrap());

    tokens.next().unwrap(); // seeds
    let seeds: Vec<usize> = tokens
        .map_while(|t| if t != "\n" { t.parse().ok() } else { None })
        .collect();

    tokens.next(); //blank
    let pipeline = get_maps(tokens);

    let mut stream: Box<dyn Iterator<Item = usize>> = Box::new(seeds.into_iter());
    for mapping in pipeline.iter().cloned() {
        stream = Box::new(stream.map(move |in_start| {
            let (window_start, (out_start, len)) = mapping
                .range(..=in_start)
                .last()
                .map(|(k, v)| (*k, *v))
                .unwrap_or_else(|| (in_start + 1, (in_start + 1, 1)));
            if (window_start..(window_start + len)).contains(&in_start) {
                (out_start + (in_start - window_start)) as usize
            } else {
                in_start
            }
        }))
    }
    let location = stream.min().unwrap();
    println!("{}", location);
}

fn part2() {
    let tokens_re = Regex::new(r"\w+:|\d+|\n").unwrap();
    let tokens = &mut tokens_re
        .find_iter(&INPUT)
        .map(|t| from_utf8(t.as_bytes()).unwrap());
    tokens.next().unwrap(); // seeds
    let mut seeds = vec![];
    while let Some(start) = tokens.next() {
        if start.as_bytes() == b"\n" {
            break;
        }
        let start = start.parse().unwrap();
        let len = tokens.next().unwrap().parse().unwrap();
        seeds.push((start, len));
    }

    tokens.next(); //blank
    let pipeline = get_maps(tokens);

    let mut stream: Box<dyn Iterator<Item = (usize, usize)>> = Box::new(seeds.into_iter());
    for mapping in pipeline.iter().cloned() {
        stream = Box::new(stream.flat_map(move |(mut src_start, mut src_len)| {
            let src_end = src_start + src_len;
            let before = mapping.range(..src_start).last().into_iter();
            let overlap = mapping.range(src_start..src_end);
            let after = once((&usize::MAX, &(usize::MAX, 0)));
            let ranges = before.chain(overlap).chain(after);
            let mut ret = vec![];
            for (tx_start, (dst_start, dst_len)) in ranges {
                if src_len <= 0 {
                    break;
                }

                let tx_len = *dst_len;
                let tx_end = tx_start + tx_len;
                // before range, no tx
                if src_start < *tx_start {
                    let len = (tx_start - src_start).min(src_len);
                    ret.push((src_start, len));
                    src_start += len;
                    src_len -= len;
                }

                if src_len <= 0 {
                    break;
                }

                // in range, tx
                if src_start >= *tx_start && src_start < tx_end {
                    let len = (tx_len - (src_start - *tx_start)).min(src_len);
                    ret.push((*dst_start + (src_start - *tx_start), len));
                    src_start += len;
                    src_len -= len;
                }
            }
            ret
        }))
    }
    let location = stream.min().unwrap().0;
    println!("{}", location);
}
