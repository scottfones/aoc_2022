use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::time::Instant;

use aoc_cache::get;

const MY_COOKIE: &str = include_str!("../../my.cookie");

fn range_contains(a: &RangeInclusive<u16>, b: &RangeInclusive<u16>) -> bool {
    match a.start().cmp(b.start()) {
        Ordering::Less => a.end() >= b.end(),
        Ordering::Equal => true,
        Ordering::Greater => b.end() >= a.end(),
    }
}

fn range_intersects(a: &RangeInclusive<u16>, b: &RangeInclusive<u16>) -> bool {
    match a.start().cmp(b.start()) {
        Ordering::Less => a.end() >= b.start(),
        Ordering::Equal => true,
        Ordering::Greater => b.end() >= a.start(),
    }
}

fn main() {
    let t_start = Instant::now();

    let input = get("https://adventofcode.com/2022/day/4/input", MY_COOKIE).unwrap();

    let p1_total = part_one(&input);
    dbg!(p1_total);

    let p2_total = part_two(&input);
    dbg!(p2_total);

    let t_total = t_start.elapsed().as_micros();
    println!("Day Four Time: {t_total} microseconds");
}

fn part_one(input: &str) -> usize {
    input.lines().filter(|&line| {
        let ranges: Vec<RangeInclusive<u16>> = line.split(',').collect::<Vec<&str>>().iter().map(|&assn| {
            let nums: Vec<&str> = assn.split('-').collect();
            let start = nums[0].parse::<u16>().unwrap();
            let end = nums[1].parse::<u16>().unwrap();
            start..=end
        }).collect();
        range_contains(&ranges[0], &ranges[1])
    }).count()
}

fn part_two(input: &str) -> usize {
    input.lines().filter(|&line| {
        let ranges: Vec<RangeInclusive<u16>> = line.split(',').collect::<Vec<&str>>().iter().map(|&assn| {
            let nums: Vec<&str> = assn.split('-').collect();
            let start = nums[0].parse::<u16>().unwrap();
            let end = nums[1].parse::<u16>().unwrap();
            start..=end
        }).collect();
        range_intersects(&ranges[0], &ranges[1])
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_day4_range_contains() {
        // true
        assert!(range_contains(&(1..=4), &(2..=3)));
        assert!(range_contains(&(1..=8), &(3..=6)));
        assert!(range_contains(&(5..=8), &(2..=9)));
        assert!(range_contains(&(1..=4), &(1..=4)));

        // false
        assert!(!range_contains(&(1..=4), &(2..=5)));
        assert!(!range_contains(&(1..=4), &(4..=5)));
        assert!(!range_contains(&(4..=10), &(1..=4)));
    }

    #[test]
    fn test_day4_part1_sample() {
        let sample_total = part_one(SAMPLE_INPUT);
        assert_eq!(sample_total, 2);
    }

    #[test]
    fn test_day4_part2_sample() {
        let sample_total = part_two(SAMPLE_INPUT);
        assert_eq!(sample_total, 4);
    }
}
