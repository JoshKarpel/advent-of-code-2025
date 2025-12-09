use crate::utils::{lines1, SolverResult};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::usize;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Junction {
    x: usize,
    y: usize,
    z: usize,
}

fn distance(a: &Junction, b: &Junction) -> f64 {
    ((a.x.abs_diff(b.x).pow(2) + a.y.abs_diff(b.y).pow(2) + a.z.abs_diff(b.z).pow(2)) as f64).sqrt()
}

fn junction(input: &str) -> IResult<&str, Junction> {
    let (input, (x, _, y, _, z)) = (usize, tag(","), usize, tag(","), usize).parse(input)?;
    Ok((input, Junction { x, y, z }))
}

fn junctions(input: &str) -> IResult<&str, Vec<Junction>> {
    lines1(junction).parse(input)
}

fn part_1(junctions: &Vec<Junction>, num_connections: usize) -> usize {
    let distances = junctions
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| (a != b).then_some(((a, b), distance(a, b))))
        .k_smallest_by(num_connections, |&(_, d1), &(_, d2)| d1.total_cmp(&d2))
        .collect_vec();

    let mut circuits: HashMap<Junction, usize> = junctions
        .clone()
        .into_iter()
        .enumerate()
        .map(|(j, i)| (i, j))
        .collect();

    for ((a, b), _) in distances {
        let circuit_a = *circuits.get(a).unwrap();
        let circuit_b = *circuits.get(b).unwrap();

        for (_, circuit) in circuits.iter_mut() {
            if *circuit == circuit_b {
                *circuit = circuit_a;
            }
        }
    }

    circuits.values().counts().values().k_largest(3).product()
}

fn part_2(junctions: &Vec<Junction>) -> usize {
    let distances = junctions
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| (a != b).then_some(((a, b), distance(a, b))))
        .sorted_by(|&(_, d1), &(_, d2)| d1.total_cmp(&d2))
        .collect_vec();

    let mut circuits: HashMap<Junction, usize> = junctions
        .clone()
        .into_iter()
        .enumerate()
        .map(|(j, i)| (i, j))
        .collect();

    distances
        .iter()
        .fold_while(0, |_, ((a, b), _)| {
            let circuit_a = *circuits.get(a).unwrap();
            let circuit_b = *circuits.get(b).unwrap();

            for (_, circuit) in circuits.iter_mut() {
                if *circuit == circuit_b {
                    *circuit = circuit_a;
                }
            }

            if circuits.values().all_equal() {
                itertools::FoldWhile::Done(a.x * b.x)
            } else {
                itertools::FoldWhile::Continue(0)
            }
        })
        .into_inner()
}

pub fn solve() -> SolverResult {
    let input = std::fs::read_to_string("inputs/day_08.txt")?;
    let (_, junctions) = junctions(&input).unwrap();

    println!("Part 1: {}", part_1(&junctions, 1000));
    println!("Part 2: {}", part_2(&junctions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_1_example() {
        let (_, junctions) = junctions(EXAMPLE).unwrap();

        assert_eq!(part_1(&junctions, 10), 40);
    }

    #[test]
    fn test_part_2_example() {
        let (_, junctions) = junctions(EXAMPLE).unwrap();

        assert_eq!(part_2(&junctions), 25272);
    }
}
