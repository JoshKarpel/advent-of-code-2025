use crate::utils::{lines1, SolverResult};
use bytes::complete::{tag, take_while};
use itertools::Itertools;
use nom::character::complete::{space1, usize};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::Parser;
use nom::{bytes, IResult};
use std::collections::HashSet;

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
    let mut indicators = HashSet::new();
    indicators.insert(vec![false; machine.target.len()]);
    loop {
        depth += 1;

        indicators = indicators
            .iter()
            .cartesian_product(machine.buttons.iter())
            .map(|(old_indicators, button)| {
                let mut new_indicators = old_indicators.clone();
                for &idx in button {
                    new_indicators[idx] = !new_indicators[idx];
                }
                new_indicators
            })
            .collect();

        if indicators.contains(&machine.target) {
            return depth;
        }
    }
}

fn part_1(machines: &[Machine]) -> usize {
    machines.iter().map(find_shortest_sequence_part_1).sum()
}

fn find_shortest_sequence_part_2(machine: &Machine) -> usize {
    let mut depth = 0;
    let mut joltages = HashSet::new();
    joltages.insert(vec![0usize; machine.joltages.len()]);
    loop {
        depth += 1;

        println!("{depth} {}", joltages.len());
        joltages = joltages
            .iter()
            .cartesian_product(machine.buttons.iter())
            .filter_map(|(old_joltages, button)| {
                let mut new_joltages = old_joltages.clone();
                for &idx in button {
                    new_joltages[idx] += 1;
                }

                // If we exceed the target joltages,
                // we can't go back down,
                // so we can prune this branch.
                new_joltages
                    .iter()
                    .zip(machine.joltages.iter())
                    .all(|(new_j, target_j)| new_j <= target_j)
                    .then_some(new_joltages)
            })
            .collect();

        if joltages.contains(&machine.joltages) {
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
