use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{fs::read, str::from_utf8};

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
fn part1() {
    let re = Regex::new(r"[^\n]+|\n\n").unwrap();
    let toks = &mut re.find_iter(&IN).peekable();

    let mut sum = 0usize;
    while toks.peek().is_some() {
        let rows = toks
            .map_while(|t| t.as_bytes().ne(b"\n\n").then(|| t.as_bytes()))
            .collect::<Vec<&[u8]>>();

        let mut mirrored;
        for j in 0..(rows.len() - 1) {
            let count = if j < rows.len() / 2 {
                j + 1
            } else {
                rows.len() - j
            };
            mirrored = true;
            for dj in 0..count {
                let a = rows[j - dj];
                let b = rows[j + 1 + dj];

                println!("{} {} {} {:?} vs {:?}", j, dj, count, a, b);
                if a != b {
                    mirrored = false;
                    break;
                }
            }
            println!("mirrored {}", mirrored);
            if mirrored {
                sum += 100 * j;
                break;
            }
        }
        println!("{}", sum);
        let mut cols = vec![];
        for i in 0..rows[0].len() {
            let mut col = vec![];
            for j in 0..rows.len() {
                col.push(rows[j][i]);
            }
            cols.push(col);
        }
    }
}

fn part2() {}
