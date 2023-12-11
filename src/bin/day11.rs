use clap::Parser;
use lazy_static::lazy_static;
use std::fs::read_to_string;

#[derive(Parser, Debug)]
#[command(author="Vampire Exec", version="0.0", about=format!("solution for {}", file!()), long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,
    #[arg(long)]
    factor: u128,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
    static ref IN: String = ARGS
        .input
        .as_ref()
        .map_or(String::new(), |p| read_to_string(p).unwrap());
}

fn main() {
    let w = IN.find("\n").unwrap() as usize;
    let map = Vec::<u8>::from(IN.lines().collect::<String>());
    let h = map.len() / w;

    let mut exp_j = vec![];
    let mut exp_i = vec![];

    for j in 0..h {
        let mut has_star = false;
        for i in 0..w {
            if map[i + j * w] == b'#' {
                has_star = true;
                break;
            }
        }
        if !has_star {
            exp_j.push(j)
        }
    }

    for i in 0..w {
        let mut has_star = false;
        for j in 0..h {
            if map[i + j * w] == b'#' {
                has_star = true;
                break;
            }
        }
        if !has_star {
            exp_i.push(i);
        }
    }

    let stars = map
        .iter()
        .enumerate()
        .filter_map(|(s, b)| b.eq(&b'#').then(|| Some((s % w, s / w))))
        .flatten()
        .collect::<Vec<_>>();

    let mut sum = 0u128;
    let f = ARGS.factor - 1;
    for a in 0..(stars.len() - 1) {
        for b in (a + 1)..stars.len() {
            let a = stars[a];
            let b = stars[b];

            let di = if a.0 < b.0 { a.0..=b.0 } else { b.0..=a.0 };
            let dj = if a.1 < b.1 { a.1..=b.1 } else { b.1..=a.1 };

            let mut exp = exp_i.iter().filter(|i| di.contains(&i)).count();
            exp += exp_j.iter().filter(|j| dj.contains(&j)).count();

            sum += a.0.abs_diff(b.0) as u128 + a.1.abs_diff(b.1) as u128 + exp as u128 * f;
        }
    }
    println!("{}", sum);
}
