use crate::utils::{lines1, whitespace_surrounded, SolverResult};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::character::complete::usize;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

struct Database {
    ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

fn range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (input, (start, end)) = separated_pair(usize, tag("-"), usize).parse(input)?;

    Ok((input, start..=end))
}

fn ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<usize>>> {
    lines1(range).parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<usize>> {
    lines1(usize).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, Database> {
    let (input, (ranges, ingredients)) =
        whitespace_surrounded(separated_pair(ranges, multispace1, ingredients)).parse(input)?;

    Ok((
        input,
        Database {
            ranges,
            ingredients,
        },
    ))
}

fn part_1(db: &Database) -> usize {
    db.ingredients
        .iter()
        .filter(|ingredient| db.ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

fn collapse_ranges(ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    ranges
        .iter()
        .sorted_by_key(|range| range.start())
        .fold(vec![], |mut acc, range| {
            if let Some(last) = acc.last_mut() {
                if range.start() <= last.end() {
                    *last = *last.start()..=*range.end().max(last.end());
                } else {
                    acc.push(range.clone());
                }
            } else {
                acc.push(range.clone());
            }
            acc
        })
}

fn part_2(db: &Database) -> usize {
    collapse_ranges(db.ranges.clone())
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

pub fn solve() -> SolverResult {
    let input = read_to_string("inputs/day_05.txt")?;
    let (_, db) = parse_input(&input).map_err(|e| e.to_string())?;

    println!("Part 1: {}", part_1(&db));
    println!("Part 2: {}", part_2(&db));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_parse_example() {
        let (_, db) = parse_input(EXAMPLE).unwrap();

        assert_eq!(db.ranges, vec![3..=5, 10..=14, 16..=20, 12..=18]);
        assert_eq!(db.ingredients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_part_1_example() {
        let (_, db) = parse_input(EXAMPLE).unwrap();

        assert_eq!(part_1(&db), 3)
    }

    #[test]
    fn test_part_2_example() {
        let (_, db) = parse_input(EXAMPLE).unwrap();

        assert_eq!(part_2(&db), 14)
    }
}
