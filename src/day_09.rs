use crate::utils::{lines1, SolverResult};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::usize;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use num::Integer;
use std::cmp::minmax;
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

fn rectangle_perimeter_points((x1, y1): Point, (x2, y2): Point) -> impl Iterator<Item = Point> {
    let [x1, x2] = minmax(x1, x2);
    let [y1, y2] = minmax(y1, y2);

    let top_bottom = (x1..=x2).flat_map(move |x| vec![(x, y1), (x, y2)]);
    let left_right = (y1..=y2).flat_map(move |y| vec![(x1, y), (x2, y)]);

    top_bottom.chain(left_right)
}

fn part_1(red_tiles: &Tiles) -> usize {
    red_tiles
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b)| a != b)
        .map(|(&a, &b)| rectangle_area(a, b))
        .max()
        .unwrap()
}

fn right_horizontal_ray_intersects_side((px, py): Point, (x1, y1): Point, (x2, y2): Point) -> bool {
    let [x1, x2] = minmax(x1, x2);
    let [y1, y2] = minmax(y1, y2);

    // Corner case: ray hits corner
    if (px == x1 && py == y1) || (px == x2 && py == y2) {
        return false;
    }

    // Horizontal ray to the right intersects vertical side
    (x1 == x2 && px <= x1 && py >= y1 && py <= y2)
        // Horizontal ray to the right intersects horizontal side
        || (y1 == y2 && py == y1 && px <= x2)
}

fn is_inside(p: Point, sides: &[(Point, Point)]) -> bool {
    println!("is {p:?} inside {sides:?}?");
    let r = sides
        .iter()
        // TODO: am I counting corners right?
        .inspect(|&&(a, b)| {
            if right_horizontal_ray_intersects_side(p, a, b) {
                println!("Ray from {:?} intersects side {:?}-{:?}", p, a, b);
            } else {
                println!("Ray from {:?} does NOT intersect side {:?}-{:?}", p, a, b);
            }
        })
        .filter(|&&(a, b)| right_horizontal_ray_intersects_side(p, a, b))
        .count()
        .is_odd();

    // println!("{r}");

    r
}

fn part_2(red_tiles: &Tiles) -> usize {
    let sides = red_tiles
        .iter()
        .chain(once(red_tiles.first().unwrap()))
        .tuple_windows()
        .map(|(&a, &b)| (a, b))
        .collect_vec();

    for y in 0..=8 {
        for x in 0..=13 {
            let p = (x, y);
            let is_red = red_tiles.contains(&p);
            let inside = is_inside(p, &sides);
            let marker = if is_red {
                "#"
            } else if inside {
                "X"
            } else {
                "."
            };
            print!("{marker}");
        }
        println!();
    }

    // red_tiles
    //     .iter()
    //     .tuple_combinations()
    //     .filter(|(&a, &b)| {
    //         // println!("Checking rectangle {:?} to {:?}", a, b);
    //         // println!("{:?}", rectangle_perimeter_points(a, b).collect_vec());
    //         rectangle_perimeter_points(a, b).all(|p| is_inside(p, &sides))
    //     })
    //     .map(|(&a, &b)| rectangle_area(a, b))
    //     .max()
    //     .unwrap()

    0
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
