use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::{Match, Regex};
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
fn mkmap(tokens: &mut dyn Iterator<Item = Match>) -> LookUp {
    let mut map = LookUp::new();
    let mut tokens = tokens.peekable();
    tokens.next(); //label
    tokens.next(); //newline
    while let Some(dest) = tokens.next() {
        if dest.as_bytes() == b"\n" {
            break;
        }
        let dest = from_utf8(dest.into()).unwrap().parse::<usize>().unwrap();
        let source = from_utf8(tokens.next().unwrap().into())
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let len = from_utf8(tokens.next().unwrap().into())
            .unwrap()
            .parse::<usize>()
            .unwrap();
        map.insert(source, (dest, len));
        tokens.next(); //newline
    }
    map
}

fn translate<'a>(
    input: Box<dyn Iterator<Item = usize>>,
    mapping: Rc<LookUp>,
) -> Box<dyn Iterator<Item = usize>> {
    Box::new(input.map(move |in_start| {
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

fn translate2<'a>(
    input: Box<dyn Iterator<Item = (usize, usize)>>,
    mapping: Rc<LookUp>,
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    Box::new(input.flat_map(move |(mut in_start, mut in_len)| {
        let in_end = in_start + in_len;
        let before = mapping.range(..in_start).last().into_iter();
        let overlap = mapping.range(in_start..in_end);
        let after = once((&usize::MAX, &(usize::MAX, 0)));
        let ranges = before.chain(overlap).chain(after);

        let ranges = ranges.collect::<Vec<(&usize, &(usize, usize))>>();
        let ranges = ranges.into_iter();

        let mut ret = vec![];
        for (window_start, (out_start, out_len)) in ranges {
            if in_len <= 0 {
                break;
            }
            let window_len = *out_len;
            let window_end = window_start + window_len;
            // before range, no xlate
            if in_start < *window_start {
                let len = (window_start - in_start).min(in_len);
                ret.push((in_start, len));
                in_start += len;
                in_len -= len;
            }

            if in_len <= 0 {
                break;
            }

            // in range, xlate
            if in_start >= *window_start && in_start < window_end {
                let len = (window_len - (in_start - *window_start)).min(in_len);
                ret.push((*out_start + (in_start - *window_start), len));
                in_start += len;
                in_len -= len;
            }
        }
        ret
    }))
}

fn part1() {
    let tokens_re = Regex::new(r"\w+:|\d+|\n").unwrap();
    let tokens = &mut tokens_re.find_iter(&INPUT);
    tokens.next().unwrap(); // seeds
    let seeds: Vec<usize> = tokens
        .map_while(|t| {
            if t.as_bytes() != b"\n" {
                Some(from_utf8(t.into()).unwrap().parse().unwrap())
            } else {
                None
            }
        })
        .collect();
    tokens.next(); //blank

    let to_soil = Rc::new(mkmap(tokens));
    let to_fertilizer = Rc::new(mkmap(tokens));
    let to_water = Rc::new(mkmap(tokens));
    let to_light = Rc::new(mkmap(tokens));
    let to_temp = Rc::new(mkmap(tokens));
    let to_humidity = Rc::new(mkmap(tokens));
    let to_location = Rc::new(mkmap(tokens));

    let pipeline = [
        to_soil,
        to_fertilizer,
        to_water,
        to_light,
        to_temp,
        to_humidity,
        to_location,
    ];
    type Stream = Box<dyn Iterator<Item = usize>>;
    type Mapper = fn(Stream, Rc<LookUp>) -> Stream;
    let location = pipeline
        .iter()
        .cloned()
        .fold::<Stream, Mapper>(Box::new(seeds.into_iter()), |it, map| translate(it, map))
        .min()
        .unwrap();
    println!("{:?}", location);
}

fn part2() {
    let tokens_re = Regex::new(r"\w+:|\d+|\n").unwrap();
    let tokens = &mut tokens_re.find_iter(&INPUT);
    tokens.next().unwrap(); // seeds
    let mut seeds = vec![];
    while let Some(start) = tokens.next() {
        if start.as_bytes() == b"\n" {
            break;
        }
        let start = from_utf8(start.into()).unwrap().parse().unwrap();
        let len = from_utf8(tokens.next().unwrap().into())
            .unwrap()
            .parse()
            .unwrap();
        seeds.push((start, len));
    }

    tokens.next(); //blank
    let tokens = &mut tokens.peekable();
    let mut pipeline = vec![];
    while tokens.peek().is_some() {
        let map = Rc::new(mkmap(tokens));
        if map.is_empty() {
            break;
        }
        pipeline.push(map)
    }

    type Stream = Box<dyn Iterator<Item = (usize, usize)>>;
    type Mapper = fn(Stream, Rc<LookUp>) -> Stream;
    let location = pipeline
        .iter()
        .cloned()
        .fold::<Stream, Mapper>(Box::new(seeds.into_iter()), |it, map| translate2(it, map))
        .min()
        .unwrap()
        .0;
    println!("{:?}", location);
}
