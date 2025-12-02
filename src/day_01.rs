use crate::utils::{lines1, whitespace_surrounded, SolverResult};
use nom::branch::alt;
use nom::character::complete::isize;
use nom::combinator::all_consuming;
use nom::{bytes::complete::tag, IResult, Parser};

type Instruction = isize;

fn instruction(input: &str) -> IResult<&str, Instruction> {
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
    all_consuming(whitespace_surrounded(lines1(instruction))).parse(input)
}

const INITIAL_POINTER: isize = 50;
const DIAL_SIZE: isize = 100;

fn part_1(instructions: &Vec<Instruction>) -> usize {
    let mut pointer: isize = INITIAL_POINTER;
    let mut counter = 0;
    for instr in instructions {
        pointer = (pointer + instr).rem_euclid(DIAL_SIZE);
        if pointer == 0 {
            counter += 1;
        }
    }

    counter
}

fn part_2(instructions: &Vec<Instruction>) -> isize {
    let mut pointer: isize = INITIAL_POINTER;
    let mut counter = 0;
    for instr in instructions {
        // Could be more clever here, but oh well
        let sgn = instr.signum();
        for _ in 0..instr.abs() {
            pointer = (pointer + sgn).rem_euclid(DIAL_SIZE);
            if pointer == 0 {
                counter += 1;
            }
        }
    }

    counter
}

pub fn solve() -> SolverResult {
    let input = include_str!("../inputs/day_01.txt");
    let (_, parsed) = parse_instructions(input)?;

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
