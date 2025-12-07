use crate::utils::SolverResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::character::complete::{multispace1, usize};
use nom::multi::separated_list1;
use nom::{IResult, Parser};

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, PartialEq)]
struct Problems {
    numbers: Vec<Vec<usize>>,
    operations: Vec<Op>,
}

fn numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, usize).parse(input)
}

fn operations(input: &str) -> IResult<&str, Vec<Op>> {
    let (input, ops) = separated_list1(space1, alt((tag("+"), tag("*")))).parse(input)?;
    Ok((
        input,
        ops.iter()
            .map(|op| match *op {
                "+" => Op::Add,
                "*" => Op::Mul,
                _ => unreachable!(),
            })
            .collect(),
    ))
}

fn parse_input(input: &str) -> IResult<&str, Problems> {
    let (input, (nums, _, ops)) = (
        separated_list1(multispace1, numbers),
        multispace1,
        operations,
    )
        .parse(input)?;

    Ok((
        input,
        Problems {
            numbers: nums,
            operations: ops,
        },
    ))
}

fn part_1(problems: &Problems) -> usize {
    problems
        .operations
        .iter()
        .enumerate()
        .map(|(idx, op)| {
            let nums = problems.numbers.iter().map(|row| row[idx]);
            match op {
                Op::Add => nums.sum::<usize>(),
                Op::Mul => nums.product(),
            }
        })
        .sum()
}

fn part_2(problems: &Problems) -> usize {
    0
}

pub fn solve() -> SolverResult {
    let input = std::fs::read_to_string("inputs/day_06.txt")?;
    let (_, problems) = parse_input(&input).map_err(|e| e.to_string())?;

    println!("Part 1: {}", part_1(&problems));
    println!("Part 2: {}", part_2(&problems));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_parse_example() {
        let (_, problems) = parse_input(EXAMPLE).unwrap();
        assert_eq!(
            problems,
            Problems {
                numbers: [
                    vec![123, 328, 51, 64],
                    vec![45, 64, 387, 23],
                    vec![6, 98, 215, 314],
                ],
                operations: vec![Op::Mul, Op::Add, Op::Mul, Op::Add],
            }
        )
    }

    #[test]
    fn test_part_1_example() {
        let (_, problems) = parse_input(EXAMPLE).unwrap();
        assert_eq!(part_1(&problems), 4277556);
    }

    #[test]
    fn test_part_2_example() {
        let (_, problems) = parse_input(EXAMPLE).unwrap();
        assert_eq!(part_2(&problems), 0);
    }
}
