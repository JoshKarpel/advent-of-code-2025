use crate::utils::{lines1, SolverResult};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::usize;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use num::Integer;
use std::cmp::minmax;
use std::collections::HashSet;
use std::iter::once;

type Point = (usize, usize);
type Tiles = Vec<(usize, usize)>;

fn red_tiles(input: &str) -> IResult<&str, Tiles> {
    lines1(separated_pair(usize, tag(","), usize)).parse(input)
}

fn rectangle_area((x1, y1): Point, (x2, y2): Point) -> usize {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

fn rectangle_points((x1, y1): Point, (x2, y2): Point) -> impl Iterator<Item = Point> {
    let [x1, x2] = minmax(x1, x2);
    let [y1, y2] = minmax(y1, y2);
    (x1..=x2).cartesian_product(y1..=y2)
}

fn horizontal_ray_intersects_side((px, py): Point, (x1, y1): Point, (x2, y2): Point) -> bool {
    let [x1, x2] = minmax(x1, x2);
    let [y1, y2] = minmax(y1, y2);

    // Horizontal ray to the right intersects vertical side
    (x1 == x2 && px <= x1 && py >= y1 && py <= y2)
        // Horizontal ray to the right intersects horizontal side
        || (y1 == y2 && py == y1 && px <= x2)
}

fn part_1(red_tiles: &Tiles) -> usize {
    red_tiles
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| rectangle_area(a, b))
        .max()
        .unwrap()
}

fn part_2(red_tiles: &Tiles) -> usize {
    let red_or_green_boundaries = red_tiles
        .iter()
        .chain(once(red_tiles.first().unwrap()))
        .tuple_windows()
        .fold(HashSet::new(), |mut greens, (&(x1, y1), &(x2, y2))| {
            // One or the other of these loops will only have one element in it, but whatever
            let [x1, x2] = minmax(x1, x2);
            let [y1, y2] = minmax(y1, y2);

            for x in x1..=x2 {
                for y in y1..=y2 {
                    greens.insert((x, y));
                }
            }
            greens
        });

    let sides = red_tiles
        .iter()
        .chain(once(red_tiles.first().unwrap()))
        .tuple_windows()
        .map(|(&a, &b)| (a, b))
        .collect_vec();

    let ((x_min, x_max), (y_min, y_max)) = red_tiles.iter().fold(
        ((usize::MAX, usize::MIN), (usize::MAX, usize::MIN)),
        |((x_min, x_max), (y_min, y_max)), &(x, y)| {
            ((x_min.min(x), x_max.max(x)), (y_min.min(y), y_max.max(y)))
        },
    );

    let mut red_or_green_tiles = red_or_green_boundaries.clone();
    for y in y_min - 1..=y_max + 1 {
        println!("{y}");
        for x in x_min - 1..=x_max + 1 {
            let crossings = sides
                .iter()
                .filter(|&&(a, b)| horizontal_ray_intersects_side((x, y), a, b))
                .count();

            if crossings.is_odd() {
                red_or_green_tiles.insert((x, y));
            }
        }
    }

    red_tiles
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b)| rectangle_points(a, b).all(|point| red_or_green_tiles.contains(&point)))
        .map(|(&a, &b)| rectangle_area(a, b))
        .max()
        .unwrap()
}

pub fn solve() -> SolverResult {
    let input = std::fs::read_to_string("inputs/day_09.txt")?;
    let (_, red_tiles) = red_tiles(&input).unwrap();

    println!("Part 1: {}", part_1(&red_tiles));
    println!("Part 2: {}", part_2(&red_tiles));

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
        let (_, red_tiles) = red_tiles(EXAMPLE).unwrap();

        assert_eq!(part_1(&red_tiles), 50);
    }

    #[test]
    fn test_part_2_example() {
        let (_, red_tiles) = red_tiles(EXAMPLE).unwrap();

        assert_eq!(part_2(&red_tiles), 24);
    }
}
