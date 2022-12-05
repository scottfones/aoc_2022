use std::cmp::Ordering;
use std::str::FromStr;
use std::time::Instant;

use aoc_cache::get_input_from_web_or_cache;

const MY_COOKIE: &str = include_str!("my.cookie");

#[derive(Clone, Copy, PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => Some(Ordering::Equal),
            (Hand::Rock, Hand::Paper) => Some(Ordering::Less),
            (Hand::Rock, Hand::Scissors) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Rock) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Paper) => Some(Ordering::Equal),
            (Hand::Paper, Hand::Scissors) => Some(Ordering::Less),
            (Hand::Scissors, Hand::Rock) => Some(Ordering::Less),
            (Hand::Scissors, Hand::Paper) => Some(Ordering::Greater),
            (Hand::Scissors, Hand::Scissors) => Some(Ordering::Equal),
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Invalid Hand type".to_string()),
        }
    }
}

fn main() {
    let t_start = Instant::now();

    let input = get_input_from_web_or_cache("https://adventofcode.com/2022/day/2/input", MY_COOKIE)
        .unwrap();

    let p1_score: u32 = input
        .lines()
        .map(|line| {
            let hands: Vec<Hand> = line
                .split(' ')
                .map(|s| s.parse::<Hand>().unwrap())
                .collect();
            match hands[0].partial_cmp(&hands[1]) {
                Some(Ordering::Less) => 6 + hands[1] as u32,
                Some(Ordering::Equal) => 3 + hands[1] as u32,
                Some(Ordering::Greater) => hands[1] as u32,
                _ => {
                    unreachable!()
                }
            }
        })
        .sum();
    println!("Part One: {p1_score}");

    let t_total = t_start.elapsed().as_micros();
    println!("Day Two Time: {t_total} microseconds");
}
