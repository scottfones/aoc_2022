use std::collections::HashMap;
use std::time::Instant;

use aoc_cache::get;

const MY_COOKIE: &str = include_str!("../../my.cookie");

fn create_value_map() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect()
}

fn main() {
    let t_start = Instant::now();

    let value_map = create_value_map();

    let input = get("https://adventofcode.com/2022/day/3/input", MY_COOKIE)
        .unwrap();

    let p1_total: usize = input
        .lines()
        .map(|sack| {
            let half = sack.len() / 2;

            let half_left: Vec<char> = sack.chars().take(half).collect();
            let half_right: Vec<char> = sack.chars().skip(half).collect();
            let shared_item: Vec<&char> = half_left
                .iter()
                .filter(|&c| half_right.contains(c))
                .collect();
            *value_map.get(shared_item[0]).unwrap()
        })
        .sum();
    println!("Part One: {p1_total}");

    let mut p2_total = 0;
    let mut sacks = input.lines();
    while let (Some(sack_a), Some(sack_b), Some(sack_c)) =
        (sacks.next(), sacks.next(), sacks.next())
    {
        let shared_item: Vec<char> = sack_a
            .chars()
            .filter(|&c| sack_b.contains(c) && sack_c.contains(c))
            .collect();
        p2_total += *value_map.get(&shared_item[0]).unwrap()
    }
    println!("Part Two: {p2_total}");

    let t_total = t_start.elapsed().as_micros();
    println!("Day Three Time: {t_total} microseconds");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_map() {
        let value_map = create_value_map();

        assert_eq!(value_map[&'a'], 1);
        assert_eq!(value_map[&'z'], 26);
        assert_eq!(value_map[&'A'], 27);
        assert_eq!(value_map[&'Z'], 52);
    }
}
