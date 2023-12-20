use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{
    collections::{HashMap, LinkedList},
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
    #[arg(long, default_value_t = false)]
    debug: bool,
    #[arg(long)]
    num: u128,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum FFState {
    On,
    Off,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Op<'a> {
    Bare,
    FF(FFState),
    Cn(Vec<(&'a str, Pulse)>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Wire<'a> {
    label: &'a str,
    op: Op<'a>,
    conns: Vec<&'a str>,
}

fn get_wires<'a>() -> HashMap<&'a str, Wire<'a>> {
    let re = Regex::new(r"(?m)(?:^(\w+)|^%(\w+)|^&(\w+)|(\w+)|(\n))").unwrap();
    let mut toks = re.captures_iter(&IN);
    let mut wires = HashMap::new();
    while let Some(m) = toks.next() {
        let groups: Vec<_> = m.iter().map(|g| g.map(|c| c.as_bytes())).collect();
        let (label, op) = match &groups[1..=3] {
            [Some(label), None, None] => (from_utf8(*label).unwrap(), Op::Bare),
            [None, Some(label), None] => (from_utf8(*label).unwrap(), Op::FF(FFState::Off)),
            [None, None, Some(label)] => (from_utf8(*label).unwrap(), Op::Cn(vec![])),
            _ => panic!("Bad input! {:?}", &groups[1..=3]),
        };
        let mut wire = Wire {
            label,
            op,
            conns: vec![],
        };
        while let Some(conn) = toks.next() {
            let conn = conn.get(0).unwrap().as_bytes();
            if conn == b"\n" {
                break;
            }
            wire.conns.push(from_utf8(conn).unwrap());
        }
        wires.insert(label, wire);
    }

    // link conjs

    let keys = wires.keys().cloned().collect::<Vec<_>>();
    for label in keys.into_iter() {
        let mut wire = wires.remove(label).unwrap();
        if let Op::Cn(conj) = &mut wire.op {
            for (other_label, other) in wires.iter() {
                for conn in other.conns.iter() {
                    if *conn == label {
                        conj.push((other_label, Pulse::Low))
                    }
                }
            }
        }
        wires.insert(label, wire);
    }

    wires
}

fn push_button<'a>(wires: &mut HashMap<&'a str, Wire<'a>>, n: u128) -> (u128, u128) {
    let mut hi_count = 0u128;
    let mut lo_count = 0u128;

    let mut queue = LinkedList::from([("button", "broadcaster", Pulse::Low)]);
    lo_count += 1;
    while let Some((prev, curr, pulse)) = queue.pop_front() {
        if let Some(curr_nc) = wires.get("nc") {
            match &curr_nc.op {
                Op::Cn(c) => {
                    let cc = c.iter().filter(|(_, p)| *p == Pulse::High).count();
                    if cc >= 1 {
                        println!("{c:?} @ {n}");
                    }
                }
                _ => panic!("no!!"),
            }
        }

        if ARGS.debug {
            println!("{prev} -{pulse:?} {curr}");
        }

        if !wires.contains_key(&curr) {
            continue;
        }
        let wire = wires.get_mut(curr).unwrap();
        match (wire, pulse) {
            (wire @ Wire { op: Op::Bare, .. }, _) => {
                for next in wire.conns.iter() {
                    if pulse == Pulse::High {
                        hi_count += 1;
                    } else {
                        lo_count += 1;
                    }
                    queue.push_back((curr, next, pulse));
                }
            }
            (Wire { op: Op::FF(_), .. }, Pulse::High) => (),
            (wire @ Wire { op: Op::FF(_), .. }, Pulse::Low) => {
                if let Op::FF(s) = wire.op {
                    let resp = if s == FFState::Off {
                        wire.op = Op::FF(FFState::On);
                        Pulse::High
                    } else {
                        wire.op = Op::FF(FFState::Off);
                        Pulse::Low
                    };
                    for next in wire.conns.iter() {
                        if resp == Pulse::High {
                            hi_count += 1;
                        } else {
                            lo_count += 1;
                        }
                        queue.push_back((curr, next, resp));
                    }
                }
            }
            (wire @ Wire { op: Op::Cn(_), .. }, pulse) => {
                if let Op::Cn(mem) = &mut wire.op {
                    for i in 0..mem.len() {
                        if mem[i].0 == prev {
                            mem[i].1 = pulse;
                        }
                    }
                    let resp = if mem.iter().all(|(_, p)| *p == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for next in wire.conns.iter() {
                        if resp == Pulse::High {
                            hi_count += 1;
                        } else {
                            lo_count += 1;
                        }
                        queue.push_back((curr, next, resp));
                    }
                }
            }
        }
    }

    (hi_count, lo_count)
}
fn part1() {
    let mut wires = get_wires();
    if ARGS.debug {
        for (label, wire) in wires.iter() {
            println!("{label} {wire:?}");
        }
    }

    let mut hi_count = 0u128;
    let mut lo_count = 0u128;
    for n in 0..ARGS.num {
        if ARGS.debug {
            println!("{n}");
        }

        let (hi, lo) = push_button(&mut wires, n);
        hi_count += hi;
        lo_count += lo;
        if ARGS.debug {
            println!("[{hi_count}, {lo_count}]");
            println!("");
        }
    }
    println!("hi {hi_count} x lo {lo_count} = {}", hi_count * lo_count);
}

fn part2() {}
