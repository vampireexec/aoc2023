use clap::{self, Parser};
use lazy_static::lazy_static;
use std::{error::Error, fs::read};

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
        part1().ok();
    } else {
        part2().ok();
    }
}
fn part1() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    Ok(())
}
