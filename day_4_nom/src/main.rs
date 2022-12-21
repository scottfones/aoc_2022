use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::time::Instant;

use aoc_cache::get;
use nom::bytes::complete::tag;
use nom::character::complete::{self, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

const MY_COOKIE: &str = include_str!("my.cookie");

/// Nom parsing taken from Chris Biscardi
/// https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2022/rust/day-04/src/lib.rs
type RangePair = (RangeInclusive<u16>, RangeInclusive<u16>);
type RangeList = Vec<RangePair>;

fn parse_sections(input: &str) -> IResult<&str, RangeInclusive<u16>> {
    let (input, (start, end)) = separated_pair(complete::u16, tag("-"), complete::u16)(input)?;

    Ok((input, start..=end))
}

fn parse_line(input: &str) -> IResult<&str, RangePair> {
    let (input, (start, end)) = separated_pair(parse_sections, tag(","), parse_sections)(input)?;

    Ok((input, (start, end)))
}

fn parse_line_assignments(input: &str) -> IResult<&str, RangeList> {
    let (input, ranges) = separated_list1(newline, parse_line)(input)?;

    Ok((input, ranges))
}

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
    println!("Part one: {p1_total}");

    let p2_total = part_two(&input);
    println!("Part two: {p2_total}");

    let t_total = t_start.elapsed().as_micros();
    println!("Day Four (Nom) Time: {t_total} microseconds");
}

fn part_one(input: &str) -> usize {
    let (_, assignment_pairs) = parse_line_assignments(input).unwrap();
    assignment_pairs
        .iter()
        .filter(|(range_a, range_b)| range_contains(range_a, range_b))
        .count()
}

fn part_two(input: &str) -> usize {
    let (_, assignment_pairs) = parse_line_assignments(input).unwrap();
    assignment_pairs
        .iter()
        .filter(|(range_a, range_b)| range_intersects(range_a, range_b))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MY_COOKIE: &str = include_str!("my.cookie");
    const SAMPLE_INPUT: &str = "2-4,6-8\n\
                                2-3,4-5\n\
                                5-7,7-9\n\
                                2-8,3-7\n\
                                6-6,4-6\n\
                                2-6,4-8";

    #[test]
    fn test_day4_nom_range_contains() {
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
    fn test_day4_nom_part1_sample() {
        let sample_total = part_one(SAMPLE_INPUT);
        assert_eq!(sample_total, 2);
    }

    #[test]
    fn test_day4_nom_part2_sample() {
        let sample_total = part_two(SAMPLE_INPUT);
        assert_eq!(sample_total, 4);
    }

    #[test]
    fn test_day4_nom_part1_actual() {
        let input = get("https://adventofcode.com/2022/day/4/input", MY_COOKIE).unwrap();
        let actual_total = part_one(&input);
        assert_eq!(actual_total, 444);
    }

    #[test]
    fn test_day4_nom_part2_actual() {
        let input = get("https://adventofcode.com/2022/day/4/input", MY_COOKIE).unwrap();
        let actual_total = part_two(&input);
        assert_eq!(actual_total, 801);
    }
}
