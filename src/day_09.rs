use crate::utils::{lines1, SolverResult};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::usize;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

type Tiles = Vec<(usize, usize)>;

fn tiles(input: &str) -> IResult<&str, Tiles> {
    lines1(separated_pair(usize, tag(","), usize)).parse(input)
}

fn rectangle_area((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

fn part_1(tiles: &Tiles) -> usize {
    tiles
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| rectangle_area(a, b))
        .max()
        .unwrap()
}

fn part_2() -> usize {
    0
}

pub fn solve() -> SolverResult {
    let input = std::fs::read_to_string("inputs/day_09.txt")?;
    let (_, tiles) = tiles(&input).unwrap();

    println!("Part 1: {}", part_1(&tiles));
    println!("Part 2: {}", part_2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_1_example() {
        let (_, tiles) = tiles(EXAMPLE).unwrap();

        assert_eq!(part_1(&tiles), 50);
    }
}
