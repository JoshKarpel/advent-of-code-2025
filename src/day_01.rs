use crate::utils::SolverResult;
use nom::branch::alt;
use nom::character::complete::{isize, multispace0};
use nom::multi::separated_list1;
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: isize,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, dir) = alt((tag("L"), tag("R"))).parse(input)?;
    let (input, distance) = isize(input)?;

    let direction = match dir {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => unreachable!(),
    };

    Ok((
        input,
        Instruction {
            direction,
            distance,
        },
    ))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, _) = multispace0(input)?;
    let (input, instructions) = separated_list1(multispace0, parse_instruction).parse(input)?;

    Ok((input, instructions))
}

fn part_1(_instructions: Vec<Instruction>) -> usize {
    let mut pointer: isize = 50;
    let mut counter = 0;
    for instr in _instructions {
        println!("{:?}", instr);
        match instr.direction {
            Direction::Left => {
                pointer -= instr.distance;
            }
            Direction::Right => {
                pointer += instr.distance;
            }
        }
        pointer = pointer.rem_euclid(100);
        if pointer == 0 {
            counter += 1;
        }
        println!("pointer:{pointer}    counter: {counter}");
    }

    counter
}

fn part_2() -> usize {
    0
}

pub fn solve() -> SolverResult {
    let instructions = include_str!("../inputs/day_01.txt");
    let (_, parsed) = parse_instructions(instructions)?;

    println!("Part 1: {}", part_1(parsed));
    println!("Part 2: {}", part_2());

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
        assert_eq!(part_1(instructions), 3);
    }
}
