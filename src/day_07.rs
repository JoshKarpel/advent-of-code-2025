use crate::utils::SolverResult;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

type Splitters = HashSet<usize>;
type Tachyons = HashSet<usize>;

fn parse_input(input: &str) -> (Vec<Splitters>, Tachyons) {
    let mut lines = input.lines();

    let mut tachyons = Tachyons::new();
    tachyons.insert(lines.next().unwrap().find("S").unwrap());

    let splitters: Vec<Splitters> = lines
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, ch)| (ch == '^').then_some(i))
                .collect()
        })
        .filter(|s: &Splitters| !s.is_empty())
        .collect_vec();

    (splitters, tachyons)
}

fn advance(current: &Tachyons, splitters: &Splitters) -> (Tachyons, usize) {
    let mut next = Tachyons::new();
    let mut splits = 0;

    for &pos in current {
        if splitters.contains(&pos) {
            // No need to worry about the ends, the grid is always wide enough
            next.insert(pos - 1);
            next.insert(pos + 1);
            splits += 1;
        } else {
            next.insert(pos);
        }
    }

    (next, splits)
}

fn part_1(start: &Tachyons, splitters: &Vec<Splitters>) -> usize {
    let (_last, splits) =
        splitters
            .iter()
            .fold((start.clone(), 0), |(current, splits), splitters| {
                let (next, new_splits) = advance(&current, splitters);
                (next, splits + new_splits)
            });

    splits
}

fn part_2(_start: &Tachyons, _splitters: &Vec<Splitters>) -> usize {
    0
}

pub fn solve() -> SolverResult {
    let input = read_to_string("inputs/day_07.txt")?;
    let (splitters, start) = parse_input(&input);

    println!("Part 1: {}", part_1(&start, &splitters));
    println!("Part 2: {}", part_2(&start, &splitters));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_1_example() {
        let (splitters, start) = parse_input(EXAMPLE);

        assert_eq!(part_1(&start, &splitters), 21);
    }

    #[test]
    fn test_part_2_example() {
        let (splitters, start) = parse_input(EXAMPLE);

        assert_eq!(part_2(&start, &splitters), 40);
    }
}
