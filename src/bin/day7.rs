use clap::Parser;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{collections::HashMap, fs::read, str::from_utf8};

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(u8)]
enum Card {
    Wild = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(u8)]
enum Ranks {
    High = 0,
    TwoOf,
    TwoPair,
    ThreeOf,
    FullHouse,
    FourOf,
    FiveOf,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        match value {
            "A" => Card::A,
            "K" => Card::K,
            "Q" => Card::Q,
            "J" => Card::J,
            "T" => Card::T,
            "9" => Card::Nine,
            "8" => Card::Eight,
            "7" => Card::Seven,
            "6" => Card::Six,
            "5" => Card::Five,
            "4" => Card::Four,
            "3" => Card::Three,
            "2" => Card::Two,
            _ => panic!("invalid input"),
        }
    }
}

impl Card {
    fn as_wild(&self) -> Card {
        match self {
            Card::J => Card::Wild,
            _ => *self,
        }
    }
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    pub cards: Vec<Card>,
    pub bid: i64,
}
impl Hand {
    fn rank(&self) -> Ranks {
        let counts = self.cards.iter().fold(HashMap::new(), |mut m, c| {
            *m.entry(*c).or_insert(0) += 1;
            m
        });

        let mut freq = counts.iter().map(|(k, v)| (*v, *k)).collect::<Vec<_>>();
        freq.sort();
        freq.reverse();

        if freq.len() > 1 {
            let wilds = freq
                .iter()
                .cloned()
                .enumerate()
                .find(|(_, (_, k))| *k == Card::Wild);
            if let Some((i, (v, _))) = wilds {
                freq.remove(i);
                freq[0].0 += v;
            }
        }

        if freq[0].0 == 5 {
            Ranks::FiveOf
        } else if freq[0].0 == 4 {
            Ranks::FourOf
        } else if freq[0].0 == 3 && freq[1].0 == 2 {
            Ranks::FullHouse
        } else if freq[0].0 == 3 {
            Ranks::ThreeOf
        } else if freq[0].0 == 2 && freq[1].0 == 2 {
            Ranks::TwoPair
        } else if freq[0].0 == 2 {
            Ranks::TwoOf
        } else {
            Ranks::High
        }
    }

    fn as_wilds(&self) -> Self {
        Self {
            cards: self.cards.iter().map(Card::as_wild).collect(),
            bid: self.bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank() != other.rank() {
            return self.rank().partial_cmp(&other.rank());
        }
        let Some((a, b)) = self.cards.iter().zip(&other.cards).find(|(a, b)| a != b) else {
            return Some(std::cmp::Ordering::Equal);
        };
        a.partial_cmp(&b)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.cards.clone();
        a.sort();
        let mut b = other.cards.clone();
        b.sort();
        a.eq(&b)
    }
}

fn part1() {
    let re = Regex::new(r"[AKQJT0-9]| |\n").unwrap();
    let toks = re.find_iter(&IN).map(|t| from_utf8(t.as_bytes()).unwrap());
    let toks = &mut toks.peekable();

    let mut hands = vec![];
    while toks.peek().is_some() {
        let cards = toks
            .map_while(|t| t.ne(" ").then(|| Card::from(t)))
            .collect::<Vec<_>>();
        let bid = toks
            .map_while(|t| t.ne("\n").then_some(t))
            .collect::<String>();
        let bid: i64 = bid.parse().unwrap();
        hands.push(Hand { cards, bid });
    }
    hands.sort();
    println!(
        "{}",
        hands.iter().enumerate().fold(0, |mut sum, (rank, hand)| {
            sum += (rank + 1) as i64 * hand.bid;
            sum
        })
    );
}

fn part2() {
    let re = Regex::new(r"[AKQJT0-9]| |\n").unwrap();
    let toks = re.find_iter(&IN).map(|t| from_utf8(t.as_bytes()).unwrap());
    let toks = &mut toks.peekable();

    let mut hands = vec![];
    while toks.peek().is_some() {
        let cards = toks
            .map_while(|t| t.ne(" ").then(|| Card::from(t)))
            .collect::<Vec<_>>();
        let bid = toks
            .map_while(|t| t.ne("\n").then_some(t))
            .collect::<String>();
        let bid: i64 = bid.parse().unwrap();
        hands.push(Hand { cards, bid });
    }
    let mut hands = hands.iter().map(Hand::as_wilds).collect::<Vec<_>>();
    hands.sort();
    let mut s = 0;
    hands
        .iter()
        .enumerate()
        .for_each(|(r, h)| s += (r + 1) as i64 * h.bid);
    println!("{}", s);
}
