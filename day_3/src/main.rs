use std::time::Instant;

use aoc_cache::get_input_from_web_or_cache;

const MY_COOKIE: &str = include_str!("my.cookie");

fn main() {
    let t_start = Instant::now();

    let input = get_input_from_web_or_cache("https://adventofcode.com/2022/day/3/input", MY_COOKIE)
        .unwrap();

    let t_total = t_start.elapsed().as_micros();
    println!("Day Three Time: {t_total} microseconds");
}
