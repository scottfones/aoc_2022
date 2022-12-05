use aoc_cache::get_input_from_web_or_cache;
use std::time::Instant;

const MY_COOKIE: &str = include_str!("my.cookie");

fn main() {
    let t_start = Instant::now();

    let input = // Grabs from web if
        get_input_from_web_or_cache(              // it's the first run
            "https://adventofcode.com/2022/day/1/input", 
            MY_COOKIE
    ).unwrap();

    let max_value = input
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .fold(0, |acc, y| acc + y.parse::<u32>().unwrap())
        })
        .max()
        .unwrap();

    println!("Part One: {max_value}");

    let mut sorted: Vec<u32> = input 
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .fold(0, |acc, y| acc + y.parse::<u32>().unwrap())
        })
        .collect::<Vec<u32>>();
    sorted.sort();
    sorted.reverse();

    let top_three_total: u32 = sorted.iter().take(3).sum();

    println!("Part Two: {top_three_total}");

    let t_total = t_start.elapsed().as_micros();
    println!("Day One Time: {t_total} microseconds");
}
