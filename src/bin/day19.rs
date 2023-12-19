use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{collections::HashMap, fs::read, iter::repeat, str::from_utf8};

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

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
            PartField::S => self.m,
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instr<'a> {
    name: &'a [u8],
    rules: Vec<Op<'a>>,
}

fn part1() {
    let rules_re =
        Regex::new(r"([xmas])([<>])(\d+):(?:([AR])|([a-z]+))|([AR])|([a-z]+)|(\n\n)").unwrap();
    let parts_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let mut stab = HashMap::new();
    let mut toks = rules_re.captures_iter(&IN);
    loop {
        let name = toks.next().unwrap().iter().next().unwrap().unwrap();
        let name = name.as_bytes();
        if name == b"\n\n" {
            break;
        }

        let mut rules = vec![];
        loop {
            let mut groups: [&[u8]; 9] = [b""; 9];
            let captures = toks.next().unwrap();
            captures
                .iter()
                .enumerate()
                .filter(|(_, m)| m.is_some())
                .for_each(|(i, m)| groups[i] = m.unwrap().as_bytes());

            match groups {
                [_, f, b">", n, b"A", b"", b"", b"", b""] => {
                    rules.push(Op::GeAccept(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                    ));
                }
                [_, f, b"<", n, b"A", b"", b"", b"", b""] => {
                    rules.push(Op::LeAccept(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                    ));
                }
                [_, f, b">", n, b"R", b"", b"", b"", b""] => {
                    rules.push(Op::GeReject(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                    ));
                }
                [_, f, b"<", n, b"R", b"", b"", b"", b""] => {
                    rules.push(Op::LeReject(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                    ));
                }
                [_, f, b">", n, b"", br, b"", b"", b""] => {
                    rules.push(Op::GeBr(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                        br,
                    ));
                }
                [_, f, b"<", n, b"", br, b"", b"", b""] => {
                    rules.push(Op::LeBr(
                        f[0].into(),
                        from_utf8(n).unwrap().parse().unwrap(),
                        br,
                    ));
                }
                [_, b"", b"", b"", b"", b"", b"A", b"", b""] => {
                    rules.push(Op::Accept);
                    break;
                }
                [_, b"", b"", b"", b"", b"", b"R", b"", b""] => {
                    rules.push(Op::Reject);
                    break;
                }
                [_, b"", b"", b"", b"", b"", b"", br, b""] => {
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

    let parts = parts_re
        .captures_iter(&IN)
        .map(|m| {
            let (_, fields) = m.extract::<4>();
            Part {
                x: from_utf8(fields[0]).unwrap().parse().unwrap(),
                m: from_utf8(fields[1]).unwrap().parse().unwrap(),
                a: from_utf8(fields[2]).unwrap().parse().unwrap(),
                s: from_utf8(fields[3]).unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for p in parts {
        let mut curr = &stab[b"in" as &[u8]];
        let mut accepted = false;
        'workflow: loop {
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
                    Op::GeAccept(f, n) => {
                        if p.get(f.clone()) > *n {
                            accepted = true;
                            break 'workflow;
                        }
                    }
                    Op::LeAccept(f, n) => {
                        if p.get(f.clone()) < *n {
                            accepted = true;
                            break 'workflow;
                        }
                    }
                    Op::GeReject(f, n) => {
                        if p.get(f.clone()) > *n {
                            break 'workflow;
                        }
                    }
                    Op::LeReject(f, n) => {
                        if p.get(f.clone()) < *n {
                            break 'workflow;
                        }
                    }
                    Op::Br(br) => {
                        curr = &stab[br];
                        break 'rules;
                    }
                    Op::Reject => {
                        break 'workflow;
                    }
                    Op::Accept => {
                        accepted = true;
                        break 'workflow;
                    }
                }
            }
        }
        if accepted {
            println!("accept {p:?}");
            sum += p.x + p.m + p.a + p.s;
        } else {
            println!("reject {p:?}");
        }
    }

    println!("sum {sum}");
}

fn part2() {}
