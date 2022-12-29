use std::time::Instant;

use aoc_cache::get;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, alpha1, digit1, multispace1, newline, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

const MY_COOKIE: &str = include_str!("../../my.cookie");

/// Nom parsing taken from Chris Biscardi
/// https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2022/rust/day-05/src/lib.rs
fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    match c {
        "   " => Ok((input, None)),
        value => Ok((input, Some(value))),
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, result))
}

#[derive(Debug, PartialEq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::u8(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u8(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u8(input)?;
    Ok((
        input,
        Move {
            count: count as usize,
            from: (from - 1) as usize,
            to: (to - 1) as usize,
        },
    ))
}

fn parser_main(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates_horizontal) = separated_list1(newline, parse_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, parse_move)(input)?;

    let mut crates_vertical: Vec<Vec<Option<&str>>> = vec![];
    for _ in 0..=crates_horizontal.len() {
        crates_vertical.push(vec![]);
    }

    for vec in crates_horizontal.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            crates_vertical[i].push(c.clone())
        }
    }

    let final_crates: Vec<Vec<&str>> = crates_vertical
        .iter()
        .map(|vec| vec.iter().filter_map(|v| *v).collect())
        .collect();

    Ok((input, (final_crates, moves)))
}

fn main() {
    let t_start = Instant::now();

    let input = get("https://adventofcode.com/2022/day/5/input", MY_COOKIE).unwrap();
    let (_input_rem, (mut tower, moves)) = parser_main(&input).unwrap();

    let p1_output = part_one(&mut tower.clone(), &moves);
    println!("Part one: {p1_output}");

    let p2_output = part_two(&mut tower, &moves);
    println!("Part two: {p2_output}");

    let t_total = t_start.elapsed().as_micros();
    println!("Day Five Time: {t_total} microseconds");
}

fn part_one(tower: &mut Vec<Vec<&str>>, moves: &Vec<Move>) -> String {
    for Move { count, from, to } in moves {
        let end = tower[*from].len();
        let mut tmp: Vec<&str> = tower[*from].drain((end-count)..).rev().collect();
        tower[*to].append(&mut tmp);
    }

    let output: String = tower
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();
    output
}

fn part_two(tower: &mut Vec<Vec<&str>>, moves: &Vec<Move>) -> String {
    for Move { count, from, to } in moves {
        let end = tower[*from].len();
        let mut tmp: Vec<&str> = tower[*from].drain((end-count)..).collect();
        tower[*to].append(&mut tmp);
    }

    let output: String = tower
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const MY_COOKIE: &str = include_str!("../../my.cookie");
    const SAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_day5_input_parser_sample() {
        let (input_rem, (tower, moves)) = parser_main(SAMPLE_INPUT).unwrap();

        assert_eq!(input_rem, "");

        assert_eq!(tower[0], vec!["Z", "N"]);
        assert_eq!(tower[1], vec!["M", "C", "D"]);
        assert_eq!(tower[2], vec!["P"]);

        let mv_a = Move {count: 1, from: 1, to: 0};
        assert_eq!(moves[0], mv_a, "testing mv_a");

        let mv_b = Move {count: 3, from: 0, to: 2};
        assert_eq!(moves[1], mv_b, "testing mv_b");

        let mv_c = Move {count: 2, from: 1, to: 0};
        assert_eq!(moves[2], mv_c, "testing mv_c");

        let mv_d = Move {count: 1, from: 0, to: 1};
        assert_eq!(moves[3], mv_d, "testing mv_d");
    }

    #[test]
    fn test_day5_part1_sample() {
        let (_input_rem, (mut tower, moves)) = parser_main(SAMPLE_INPUT).unwrap();
        let test_output = part_one(&mut tower, &moves);

        assert_eq!(tower[0], vec!["C"]);
        assert_eq!(tower[1], vec!["M"]);
        assert_eq!(tower[2], vec!["P", "D", "N", "Z"]);

        assert_eq!(test_output, "CMZ".to_string());
    }

    #[test]
    fn test_day5_part2_sample() {
        let (_input_rem, (mut tower, moves)) = parser_main(SAMPLE_INPUT).unwrap();
        let test_output = part_two(&mut tower, &moves);

        assert_eq!(tower[0], vec!["M"]);
        assert_eq!(tower[1], vec!["C"]);
        assert_eq!(tower[2], vec!["P", "Z", "N", "D"]);

        assert_eq!(test_output, "MCD".to_string());
    }

    #[test]
    fn test_day5_part1_actual() {
        let input = get("https://adventofcode.com/2022/day/5/input", MY_COOKIE).unwrap();
        let (_input_rem, (mut tower, moves)) = parser_main(&input).unwrap();
        let test_output = part_one(&mut tower, &moves);
        assert_eq!(test_output, "VJSFHWGFT".to_string());
    }

    #[test]
    fn test_day5_part2_actual() {
        let input = get("https://adventofcode.com/2022/day/5/input", MY_COOKIE).unwrap();
        let (_input_rem, (mut tower, moves)) = parser_main(&input).unwrap();
        let test_output = part_two(&mut tower, &moves);
        assert_eq!(test_output, "LCTQFBVZV".to_string());
    }
}
