use crate::utils::{lines1, SolverResult};
use bytes::complete::{tag, take_while};
use itertools::Itertools;
use nom::character::complete::{space1, usize};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::Parser;
use nom::{bytes, IResult};

type Indicators = Vec<bool>;
type Button = Vec<usize>;
type Buttons = Vec<Button>;
type Joltages = Vec<usize>;

#[derive(Debug, Clone)]
struct Machine {
    target: Indicators,
    buttons: Buttons,
    joltages: Joltages,
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, (target, _, buttons, _, joltages)) = (
        delimited(tag("["), take_while(|c| c == '.' || c == '#'), tag("]")),
        space1,
        separated_list1(
            space1,
            delimited(tag("("), separated_list1(tag(","), usize), tag(")")),
        ),
        space1,
        delimited(tag("{"), separated_list1(tag(","), usize), tag("}")),
    )
        .parse(input)?;

    Ok((
        input,
        Machine {
            target: target.chars().map(|c| c == '#').collect(),
            buttons,
            joltages,
        },
    ))
}

fn machines(input: &str) -> IResult<&str, Vec<Machine>> {
    let (input, machines) = lines1(machine).parse(input)?;

    // For simplicity, just return the first machine for now
    Ok((input, machines))
}

fn find_shortest_sequence_part_1(machine: &Machine) -> usize {
    let mut depth = 0;
    loop {
        depth += 1;

        if machine
            .buttons
            .iter()
            .combinations_with_replacement(depth)
            .any(|buttons| {
                let mut indicators = vec![false; machine.target.len()];

                for button in buttons {
                    for &idx in button {
                        indicators[idx] = !indicators[idx];
                    }
                }

                indicators == machine.target
            })
        {
            return depth;
        }
    }
}

fn part_1(machines: &[Machine]) -> usize {
    machines.iter().map(find_shortest_sequence_part_1).sum()
}

fn find_shortest_sequence_part_2(machine: &Machine) -> usize {
    let mut depth = 0;
    loop {
        depth += 1;
        println!("{depth}");

        if machine
            .buttons
            .iter()
            .combinations_with_replacement(depth)
            .any(|buttons| {
                let mut joltages = vec![0usize; machine.target.len()];

                for button in buttons {
                    for &idx in button {
                        joltages[idx] += 1;
                    }
                }

                joltages == machine.joltages
            })
        {
            return depth;
        }
    }
}

fn part_2(machines: &[Machine]) -> usize {
    machines.iter().map(find_shortest_sequence_part_2).sum()
}

pub fn solve() -> SolverResult {
    let input = std::fs::read_to_string("inputs/day_10.txt")?;
    let (_, machines) = machines(&input).map_err(|e| e.to_string())?;

    println!("Part 1: {}", part_1(&machines));
    println!("Part 2: {}", part_2(&machines));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part_1_example() {
        let (_, machines) = machines(EXAMPLE).unwrap();

        assert_eq!(part_1(&machines), 7);
    }

    #[test]
    fn test_part_2_example() {
        let (_, machines) = machines(EXAMPLE).unwrap();

        assert_eq!(part_2(&machines), 33);
    }
}
