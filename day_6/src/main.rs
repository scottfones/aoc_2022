use std::collections::VecDeque;
use std::str;
use std::time::Instant;

use aoc_cache::get;

const MY_COOKIE: &str = include_str!("../../my.cookie");

/// Bit offset approach from Amos (fasterthanlime)
/// https://fasterthanli.me/series/advent-of-code-2022/part-6
trait GetBitOffset {
    fn get_bit_offset(&self) -> u32;
}

impl GetBitOffset for u8 {
    fn get_bit_offset(&self) -> u32 {
        1 << (*self as u32 - 'a' as u32)
    }
}

fn main() {
    let t_start = Instant::now();

    let input = get("https://adventofcode.com/2022/day/6/input", MY_COOKIE).unwrap();

    let p1_output = part_one(&input).unwrap();
    println!("Part one: {p1_output}");

    let p2_output = part_two(&input).unwrap();
    println!("Part two: {p2_output}");

    let t_total = t_start.elapsed().as_micros();
    println!("Day Six Time: {t_total} microseconds");
}

fn part_one(input: &str) -> Option<u32> {
    let mut input_iter = input.as_bytes().iter();
    let mut a = input_iter.next().unwrap().get_bit_offset();
    let mut b = input_iter.next().unwrap().get_bit_offset();
    let mut c = input_iter.next().unwrap().get_bit_offset();

    let mut index = 4;
    for val in input_iter {
        let d = val.get_bit_offset();
        let comp = a | b | c | d;
        if comp.count_ones() == 4 {
            return Some(index);
        }

        a = b;
        b = c;
        c = d;
        index += 1;
    }
    None
}

fn part_two(input: &str) -> Option<u32> {
    let mut input_iter = input.as_bytes().iter();

    let mut priors: VecDeque<u32> = (0..13)
        .map(|_| input_iter.next().unwrap().get_bit_offset())
        .collect();

    let mut index = 14;
    for val in input_iter {
        let new_bits = val.get_bit_offset();
        priors.push_front(new_bits);
        let comp = priors.iter().fold(0, |acc, b| acc | b);
        if comp.count_ones() == 14 {
            return Some(index);
        }

        priors.pop_back();
        index += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const MY_COOKIE: &str = include_str!("../../my.cookie");
    const SAMPLE_A: (&str, u32, u32) = ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23);
    const SAMPLE_B: (&str, u32, u32) = ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23);
    const SAMPLE_C: (&str, u32, u32) = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29);
    const SAMPLE_D: (&str, u32, u32) = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26);
    const SAMPLE_E: (&str, u32, u32) = ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19);

    #[test]
    fn test_day6_part1_sample() {
        let sample_res = part_one(SAMPLE_A.0).unwrap();
        assert_eq!(sample_res, SAMPLE_A.1,);

        let sample_res = part_one(SAMPLE_B.0).unwrap();
        assert_eq!(sample_res, SAMPLE_B.1,);

        let sample_res = part_one(SAMPLE_C.0).unwrap();
        assert_eq!(sample_res, SAMPLE_C.1,);

        let sample_res = part_one(SAMPLE_D.0).unwrap();
        assert_eq!(sample_res, SAMPLE_D.1,);

        let sample_res = part_one(SAMPLE_E.0).unwrap();
        assert_eq!(sample_res, SAMPLE_E.1,);
    }

    #[test]
    fn test_day6_part2_sample() {
        let sample_res = part_two(SAMPLE_A.0).unwrap();
        assert_eq!(sample_res, SAMPLE_A.2,);

        let sample_res = part_two(SAMPLE_B.0).unwrap();
        assert_eq!(sample_res, SAMPLE_B.2,);

        let sample_res = part_two(SAMPLE_C.0).unwrap();
        assert_eq!(sample_res, SAMPLE_C.2,);

        let sample_res = part_two(SAMPLE_D.0).unwrap();
        assert_eq!(sample_res, SAMPLE_D.2,);

        let sample_res = part_two(SAMPLE_E.0).unwrap();
        assert_eq!(sample_res, SAMPLE_E.2,);
    }

    #[test]
    fn test_day6_part1_actual() {
        let input = get("https://adventofcode.com/2022/day/6/input", MY_COOKIE).unwrap();
        let actual_res = part_one(&input).unwrap();
        assert_eq!(actual_res, 1361);
    }

    #[test]
    fn test_day6_part2_actual() {
        let input = get("https://adventofcode.com/2022/day/6/input", MY_COOKIE).unwrap();
        let actual_res = part_two(&input).unwrap();
        assert_eq!(actual_res, 3263);
    }
}
