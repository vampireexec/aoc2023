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
        let (start_a, end_a, start_b, end_b) = if j < rows.len() / 2 {
            (0, j, j + 1, 2 * j + 1)
        } else {
            (j - ((rows.len() - 1) - j - 1), j, j + 1, rows.len() - 1)
        };

        let mut a = rows[start_a..=end_a].to_vec();
        a.reverse();
        let b = rows[start_b..=end_b].to_vec();
        assert_eq!(a.len(), b.len());

        if let Some(incl_row) = incl_row {
            if !(start_a..end_b).contains(&incl_row) {
                continue;
            }
        }

        if a == b {
            return (Some(j), None, 100 * (j + 1));
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
        let (start_a, end_a, start_b, end_b) = if i < cols.len() / 2 {
            (0, i, i + 1, 2 * i + 1)
        } else {
            (i - ((cols.len() - 1) - i - 1), i, i + 1, cols.len() - 1)
        };

        let mut a = cols[start_a..=end_a].to_vec();
        a.reverse();
        let b = cols[start_b..=end_b].to_vec();
        assert_eq!(a.len(), b.len());

        if let Some(incl_col) = incl_col {
            if !(start_a..end_b).contains(&incl_col) {
                continue;
            }
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
                let (found_row, found_col, value) = check_mirror(&mut rows, Some(j), Some(i));
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
