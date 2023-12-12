use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::read_to_string, iter::repeat};

#[derive(Parser, Debug)]
#[command(author="Vampire Exec", version="0.0", about=format!("solution for {}", file!()), long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,
    #[arg(long)]
    folds: usize,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
    static ref IN: String = ARGS
        .input
        .as_ref()
        .map_or(String::new(), |p| read_to_string(p).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Spring {
    Good,
    Bad,
    Unk,
}
impl From<&str> for Spring {
    fn from(value: &str) -> Self {
        match value {
            "." => Self::Good,
            "#" => Self::Bad,
            "?" => Self::Unk,
            _ => panic!("Bad input"),
        }
    }
}

impl ToString for Spring {
    fn to_string(&self) -> String {
        match self {
            Spring::Good => ".".into(),
            Spring::Bad => "#".into(),
            Spring::Unk => "?".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Report {
    springs: Vec<Spring>,
    index: Vec<usize>,
}

fn main() {
    let re = Regex::new(r"\n| |#|\.|\?|\d+").unwrap();
    let mut reports = vec![];
    let toks = &mut re.find_iter(&IN).map(|t| t.as_str()).peekable();
    while let Some(_) = toks.peek() {
        let springs = toks
            .map_while(|t| t.ne(" ").then(|| Spring::from(t)))
            .collect();
        let springs = repeat(springs)
            .take(ARGS.folds)
            .collect::<Vec<Vec<_>>>()
            .join(&Spring::Unk);

        let index = toks
            .map_while(|t| t.ne("\n").then(|| t.parse().unwrap()))
            .collect::<Vec<usize>>();

        let index = repeat(index)
            .take(ARGS.folds)
            .flatten()
            .collect::<Vec<usize>>();

        reports.push(Report {
            springs: springs,
            index: index,
        })
    }

    let mut sum = 0;
    let mut report_n = 0;
    for report in reports {
        let unknowns = report.springs.iter().filter(|s| s == &&Spring::Unk).count();
        let known_bad = report.springs.iter().filter(|s| s == &&Spring::Bad).count();
        let total_bad = report.index.iter().sum::<usize>();

        if total_bad == 0 || total_bad == known_bad {
            sum += 1;
            println!(
                "{:04} {:04} {}",
                report_n,
                1,
                report
                    .springs
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<String>()
            );
            report_n += 1;
            continue;
        }

        let mut constructed = report.springs.clone();
        let mut configs = 0;

        let min = (1u128 << (total_bad - known_bad)) - 1;
        let max = min << (unknowns + known_bad - total_bad);
        println!("{} {}", min, max);
        for mask in min..=max {
            if (mask.count_ones() + (known_bad as u32)) != total_bad as u32 {
                continue;
            }

            constructed.truncate(0);
            let mut j = 0;
            for i in 0..report.springs.len() {
                if report.springs[i] == Spring::Unk {
                    if (mask & (1 << j)) != 0 {
                        constructed.push(Spring::Bad);
                    } else {
                        constructed.push(Spring::Good);
                    }
                    j += 1;
                } else {
                    constructed.push(report.springs[i]);
                }
            }

            let mut idx = 0;
            let mut bad_count = 0;
            let mut mode = constructed[0];
            let mut expected_bad = if mode == Spring::Good {
                0
            } else {
                idx = 1;
                report.index[0]
            };

            let mut valid = true;
            for i in 0..constructed.len() {
                let curr = constructed[i];
                match (mode, curr) {
                    (Spring::Good, Spring::Good) => (),
                    (Spring::Good, Spring::Bad) => {
                        if idx >= report.index.len() {
                            valid = false;
                            break;
                        }
                        mode = Spring::Bad;
                        expected_bad = report.index[idx];
                        idx += 1;
                        bad_count = 1;
                    }
                    (Spring::Bad, Spring::Good) => {
                        if bad_count != expected_bad {
                            valid = false;
                            break;
                        }
                        mode = Spring::Good;
                        expected_bad = 0;
                    }
                    (Spring::Bad, Spring::Bad) => bad_count += 1,
                    _ => panic!("Failed to construct"),
                }
            }
            if valid && idx == report.index.len() {
                configs += 1;
            }
        }

        println!(
            "{:04} {:04} {}",
            report_n,
            configs,
            report
                .springs
                .iter()
                .map(|s| s.to_string())
                .collect::<String>()
        );
        report_n += 1;
        sum += configs;
    }
    println!("{}", sum);
}
