use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::fs::read;

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
    let re = Regex::new("[^\n]+").unwrap();
    let mut map = re
        .find_iter(&IN)
        .map(|t| Vec::from(t.as_bytes()))
        .collect::<Vec<_>>();
    let w = map[0].len();
    let h = map.len();

    let mut sum = 0;
    for j in 0..h {
        for i in 0..w {
            if map[j][i] == b'O' {
                map[j][i] = b'.';
                let mut dest = (i, j);
                for dj in (0..=j).rev() {
                    if dj == 0 || map[dj - 1][i] != b'.' {
                        dest = (i, dj);
                        break;
                    }
                }
                map[dest.1][dest.0] = b'O';
                sum += h - dest.1;
            }
        }
    }
    // for j in 0..h {
    //     for i in 0..w {
    //         print!("{}", map[j][i] as char);
    //     }
    //     println!("");
    // }

    // for j in 0..h {
    //     for i in 0..w {
    //         if map[j][i] == b'O' {
    //             print!("{:x}", h - j);
    //         } else {
    //             print!("{}", map[j][i] as char);
    //         }
    //     }
    //     println!("");
    // }
    println!("{sum}");
}

fn part2() {}
