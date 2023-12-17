use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{fs::read, iter::repeat_with};

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
    let toks = &mut re.find_iter(&IN).map(|t| t.as_bytes()).peekable();
    let mut sum = 0;
    while toks.peek().is_some() {
        let mut rows = toks
            .map_while(|t| t.ne(b"\n\n").then(|| Some(Vec::from(t))))
            .map(|o| o.unwrap())
            .collect::<Vec<_>>();

        let (_, _, value) = check_mirror(&mut rows, None, None);
        sum += value;
    }

    println!("{sum}");
}

fn check_mirror(
    rows: &mut Vec<Vec<u8>>,
    incl_row: Option<usize>,
    incl_col: Option<usize>,
) -> (Option<usize>, Option<usize>, usize) {
    for j in 0..(rows.len() - 1) {
        let mut a = rows[0..=j].to_vec();
        let mut b = rows[(j + 1)..].to_vec();
        a.reverse();
        if a.len() < b.len() {
            b.truncate(a.len());
        } else {
            a.truncate(b.len())
        }

        if a == b {
            return (None, Some(j), 100 * (j + 1));
        }
    }

    //transpose
    let mut cols = repeat_with(|| repeat_with(|| b'.').take(rows.len()).collect::<Vec<_>>())
        .take(rows[0].len())
        .collect::<Vec<_>>();

    for j in 0..rows.len() {
        for i in 0..rows[0].len() {
            cols[i][j] = rows[j][i];
        }
    }

    // mirrored
    for i in 0..(cols.len() - 1) {
        let mut a = cols[0..=i].to_vec();
        let mut b = cols[(i + 1)..].to_vec();
        a.reverse();
        if a.len() < b.len() {
            b.truncate(a.len());
        } else {
            a.truncate(b.len())
        }

        if a == b {
            return (None, Some(i), i + 1);
        }
    }
    (None, None, 0)
}

fn part2() {
    let re = Regex::new(r"[^\n]+|\n\n").unwrap();
    let toks = &mut re.find_iter(&IN).map(|t| t.as_bytes()).peekable();
    let mut sum = 0;
    while toks.peek().is_some() {
        let mut rows = toks
            .map_while(|t| t.ne(b"\n\n").then(|| Some(Vec::from(t))))
            .map(|o| o.unwrap())
            .collect::<Vec<_>>();

        let (orig_row, orig_col, _) = check_mirror(&mut rows, None, None);
        'outer: for j in 0..rows.len() {
            for i in 0..rows[0].len() {
                let curr = rows[j][i];
                rows[j][i] = if curr == b'.' { b'#' } else { b'.' };
                let (found_row, found_col, value) = check_mirror(&mut rows, None, None);
                rows[j][i] = curr;

                if (orig_row.and(found_row).is_some() && orig_row != found_row)
                    || (orig_col.and(found_col).is_some() && orig_col != found_col)
                {
                    println!("here {:?} {:?} {}", found_row, found_col, value);
                    sum += value;
                    break 'outer;
                }
            }
        }
    }

    println!("{sum}");
}
