use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{collections::HashMap, fmt::Debug, fs::read, str::from_utf8};

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Op<'a> {
    GeBr(PartField, i64, &'a [u8]),
    LeBr(PartField, i64, &'a [u8]),
    GeAccept(PartField, i64),
    LeAccept(PartField, i64),
    GeReject(PartField, i64),
    LeReject(PartField, i64),
    Br(&'a [u8]),
    Reject,
    Accept,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum PartField {
    X,
    M,
    A,
    S,
}

impl Part {
    fn get(&self, f: PartField) -> i64 {
        match f {
            PartField::X => self.x,
            PartField::M => self.m,
            PartField::A => self.a,
            PartField::S => self.s,
        }
    }

    fn _set(&mut self, v: i64, f: PartField) {
        match f {
            PartField::X => self.x = v,
            PartField::M => self.m = v,
            PartField::A => self.a = v,
            PartField::S => self.s = v,
        }
    }
}

impl From<u8> for PartField {
    fn from(value: u8) -> Self {
        match value {
            b'x' => PartField::X,
            b'm' => PartField::M,
            b'a' => PartField::A,
            b's' => PartField::S,
            _ => panic!("bad field"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Instr<'a> {
    name: &'a [u8],
    rules: Vec<Op<'a>>,
}

fn get_stabs<'a>(input: &'a [u8]) -> HashMap<&'a [u8], Instr<'a>> {
    let rules_re =
        Regex::new(r"([xmas])([<>])(\d+):(?:([AR])|([a-z]+))|([AR])|([a-z]+)|(\n\n)").unwrap();
    let mut stab = HashMap::new();
    let mut toks = rules_re.captures_iter(&input);
    loop {
        let name = toks.next().unwrap().iter().next().unwrap().unwrap();
        let name = name.as_bytes();
        if name == b"\n\n" {
            break;
        }

        let mut rules = vec![];
        loop {
            let captures = toks.next().unwrap();
            let groups = captures
                .iter()
                .map(|m| m.map_or_else(|| b"" as &[u8], |m| m.as_bytes()))
                .collect::<Vec<_>>();
            match &groups[1..=7] {
                [f, b">", n, b"A", b"", b"", b""] => {
                    rules.push(Op::GeAccept(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                    ));
                }
                [f, b"<", n, b"A", b"", b"", b""] => {
                    rules.push(Op::LeAccept(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                    ));
                }
                [f, b">", n, b"R", b"", b"", b""] => {
                    rules.push(Op::GeReject(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                    ));
                }
                [f, b"<", n, b"R", b"", b"", b""] => {
                    rules.push(Op::LeReject(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                    ));
                }
                [f, b">", n, b"", br, b"", b""] => {
                    rules.push(Op::GeBr(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                        br,
                    ));
                }
                [f, b"<", n, b"", br, b"", b""] => {
                    rules.push(Op::LeBr(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                        br,
                    ));
                }
                [b"", b"", b"", b"", b"", b"A", b""] => {
                    rules.push(Op::Accept);
                    break;
                }
                [b"", b"", b"", b"", b"", b"R", b""] => {
                    rules.push(Op::Reject);
                    break;
                }
                [b"", b"", b"", b"", b"", b"", br] => {
                    rules.push(Op::Br(br));
                    break;
                }
                _ => panic!(
                    "unkown: {:?}",
                    groups
                        .iter()
                        .map(|b| from_utf8(b).unwrap())
                        .collect::<Vec<_>>()
                ),
            }
        }

        stab.insert(name, Instr { name, rules });
    }
    stab
}

fn get_parts<'a>(input: &'a [u8]) -> Vec<Part> {
    Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}")
        .unwrap()
        .captures_iter(&input)
        .map(|m| {
            let (_, fields) = m.extract::<4>();
            Part {
                x: from_utf8(fields[0]).unwrap().parse().unwrap(),
                m: from_utf8(fields[1]).unwrap().parse().unwrap(),
                a: from_utf8(fields[2]).unwrap().parse().unwrap(),
                s: from_utf8(fields[3]).unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn eval_part(p: &Part, stab: &HashMap<&[u8], Instr>) -> bool {
    let mut curr = &stab[b"in" as &[u8]];
    loop {
        'rules: for rule in &curr.rules {
            match rule {
                Op::GeBr(f, n, br) => {
                    if p.get(f.clone()) > *n {
                        curr = &stab[br];
                        break 'rules;
                    }
                }
                Op::LeBr(f, n, br) => {
                    if p.get(f.clone()) < *n {
                        curr = &stab[br];
                        break 'rules;
                    }
                }
                Op::Br(br) => {
                    curr = &stab[br];
                    break 'rules;
                }
                Op::GeAccept(f, n) => {
                    if p.get(f.clone()) > *n {
                        return true;
                    }
                }
                Op::LeAccept(f, n) => {
                    if p.get(f.clone()) < *n {
                        return true;
                    }
                }
                Op::Accept => return true,
                Op::GeReject(f, n) => {
                    if p.get(f.clone()) > *n {
                        return false;
                    }
                }

                Op::LeReject(f, n) => {
                    if p.get(f.clone()) < *n {
                        return false;
                    }
                }
                Op::Reject => return false,
            }
        }
    }
}

fn part1() {
    let mut sum = 0;
    let stab = get_stabs(&IN);
    let parts = get_parts(&IN);
    for p in parts {
        if eval_part(&p, &stab) {
            sum += p.x + p.m + p.a + p.s;
        }
    }

    println!("sum {sum}");
}

fn part2() {
    let stab = get_stabs(&IN);
    let mut stack = stab[b"in" as &[u8]]
        .rules
        .iter()
        .map(|r| vec![r.clone()])
        .collect::<Vec<_>>();
    let mut accept_paths = vec![];
    while let Some(path) = stack.pop() {
        let curr = path.last().unwrap();
        match curr {
            Op::GeAccept(_, _) | Op::LeAccept(_, _) | Op::Accept => {
                accept_paths.push(path);
            }
            Op::GeBr(_, _, label) | Op::LeBr(_, _, label) | Op::Br(label) => {
                for rule in stab[&label as &[u8]].rules.iter() {
                    let mut next_path = path.clone();
                    next_path.push(rule.clone());
                    stack.push(next_path);
                }
            }
            Op::GeReject(_, _) | Op::LeReject(_, _) | Op::Reject => (),
        }
    }

    let mut acceptable_union = HashMap::from([
        (PartField::X, (4000, 1)),
        (PartField::M, (4000, 1)),
        (PartField::A, (4000, 1)),
        (PartField::S, (4000, 1)),
    ]);
    let mut total = 0;
    for path in accept_paths.iter() {
        let mut acceptable = HashMap::from([
            (PartField::X, (1, 4000)),
            (PartField::M, (1, 4000)),
            (PartField::A, (1, 4000)),
            (PartField::S, (1, 4000)),
        ]);
        for rule in path.iter() {
            match rule {
                Op::GeBr(field, n, _) | Op::GeAccept(field, n) => {
                    let ent = acceptable.get_mut(&field).unwrap();
                    if ent.0 < *n {
                        ent.0 = *n;
                    }
                }
                Op::LeBr(field, n, _) | Op::LeAccept(field, n) => {
                    let ent = acceptable.get_mut(&field).unwrap();
                    if ent.1 > *n {
                        ent.1 = *n
                    }
                }
                Op::Br(_) | Op::Accept => (),
                Op::GeReject(_, _) | Op::LeReject(_, _) | Op::Reject => panic!("should not reject"),
            }
        }

        let mut count = 1;
        for field in [PartField::X, PartField::M, PartField::A, PartField::S] {
            assert!(acceptable[&field].0 < acceptable[&field].1);
            count *= acceptable[&field].1 - acceptable[&field].0;
        }
        total += count;
        println!("{path:?}\n{acceptable:?}\ncount={count}\ntotal={total}");

        for field in [PartField::X, PartField::M, PartField::A, PartField::S] {
            if acceptable_union[&field].0 > acceptable[&field].0 {
                let au = acceptable_union.get_mut(&field).unwrap();
                au.0 = acceptable[&field].0;
            }
            if acceptable_union[&field].1 < acceptable[&field].1 {
                let au = acceptable_union.get_mut(&field).unwrap();
                au.1 = acceptable[&field].1;
            }
        }
    }

    let mut count = 1;
    for field in [PartField::X, PartField::M, PartField::A, PartField::S] {
        assert!(acceptable_union[&field].0 < acceptable_union[&field].1);
        count *= acceptable_union[&field].1 - acceptable_union[&field].0;
    }
    println!("{acceptable_union:?}");
    println!("other={}", count);
    println!("real ={}", 167409079868000)
}
