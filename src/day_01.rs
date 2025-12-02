use crate::utils::SolverResult;
use nom::branch::alt;
use nom::character::complete::{isize, multispace0};
use nom::multi::separated_list1;
use nom::{bytes::complete::tag, IResult, Parser};

type Instruction = isize;

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = alt((tag("L"), tag("R"))).parse(input)?;
    let (input, distance) = isize(input)?;

    let signed_distance = match direction {
        "L" => -distance,
        "R" => distance,
        _ => unreachable!(),
    };

    Ok((input, signed_distance))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, _) = multispace0(input)?;
    let (input, instructions) = separated_list1(multispace0, parse_instruction).parse(input)?;

    Ok((input, instructions))
}

fn part_1(instructions: &Vec<Instruction>) -> usize {
    let mut pointer: isize = 50;
    let mut counter = 0;
    for instr in instructions {
        pointer = (pointer + instr).rem_euclid(100);
        if pointer == 0 {
            counter += 1;
        }
    }

    counter
}

fn part_2(instructions: &Vec<Instruction>) -> usize {
    let mut pointer: isize = 50;
    let mut counter = 0;
    for instr in instructions {
        // Could be more clever here, but oh well
        let sgn = instr.signum();
        for _ in 0..instr.abs() {
            pointer = (pointer + sgn).rem_euclid(100);
            if pointer == 0 {
                counter += 1;
            }
        }
    }

    counter
}

pub fn solve() -> SolverResult {
    let instructions = include_str!("../inputs/day_01.txt");
    let (_, parsed) = parse_instructions(instructions)?;

    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
    ";

    #[test]
    fn test_part_1_example() {
        let (_, instructions) = parse_instructions(EXAMPLE).unwrap();
        assert_eq!(part_1(&instructions), 3);
    }

    #[test]
    fn test_part_2_example() {
        let (_, instructions) = parse_instructions(EXAMPLE).unwrap();
        assert_eq!(part_2(&instructions), 6);
    }
}
