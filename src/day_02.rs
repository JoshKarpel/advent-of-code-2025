use crate::utils::{whitespace_surrounded, SolverResult};
use nom::bytes::complete::tag;
use nom::character::complete::usize;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (input, (start, end)) = separated_pair(usize, tag("-"), usize).parse(input)?;

    Ok((input, start..=end))
}

fn ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<usize>>> {
    whitespace_surrounded(separated_list0(tag(","), range)).parse(input)
}

fn is_invalid_part_1(num: usize) -> bool {
    let chars = num.to_string().chars().collect::<Vec<char>>();

    let len = chars.len();
    if len % 2 != 0 {
        false
    } else {
        let midpoint = len / 2;
        chars[..midpoint] == chars[midpoint..]
    }
}

fn all_equal<T: PartialEq, I: IntoIterator<Item = T>>(iter: I) -> bool {
    let mut iter = iter.into_iter();
    if let Some(first) = iter.next() {
        iter.all(|x| x == first)
    } else {
        true
    }
}

fn is_invalid_part_2(num: usize) -> bool {
    let chars = num.to_string().chars().collect::<Vec<char>>();

    (1..=chars.len() / 2).any(|chunk_size| all_equal(chars.chunks(chunk_size)))
}

fn part_1(ranges: &[RangeInclusive<usize>]) -> usize {
    ranges
        .iter()
        .flat_map(|r| r.clone().filter(|&n| is_invalid_part_1(n)))
        .sum()
}

fn part_2(ranges: &[RangeInclusive<usize>]) -> usize {
    ranges
        .iter()
        .flat_map(|r| r.clone().filter(|&n| is_invalid_part_2(n)))
        .sum()
}

pub fn solve() -> SolverResult {
    let input = read_to_string("inputs/day_02.txt")?;
    let (_, ranges) = ranges(&input).map_err(|e| e.to_string())?;

    println!("Part 1: {}", part_1(&ranges));
    println!("Part 2: {}", part_2(&ranges));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1_example() {
        let (_, ranges) = ranges(EXAMPLE).unwrap();
        assert_eq!(part_1(&ranges), 1227775554);
    }

    #[test]
    fn test_part_2_example() {
        let (_, ranges) = ranges(EXAMPLE).unwrap();
        assert_eq!(part_2(&ranges), 4174379265);
    }
}
