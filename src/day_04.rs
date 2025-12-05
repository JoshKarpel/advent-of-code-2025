use crate::utils::SolverResult;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

type PaperGrid = HashMap<(isize, isize), bool>;

fn parse_input(input: &str) -> PaperGrid {
    let mut grid = PaperGrid::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert((x as isize, y as isize), ch == '@');
        }
    }

    grid
}

const DX_DY: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn count_occupied_neighbours(grid: &PaperGrid, (x, y): &(isize, isize)) -> usize {
    DX_DY
        .iter()
        .filter(|(dx, dy)| *grid.get(&(x + dx, y + dy)).unwrap_or(&false))
        .count()
}

fn part_1(grid: &PaperGrid) -> usize {
    grid.iter()
        .filter(|(xy, occupied)| **occupied && count_occupied_neighbours(grid, xy) < 4)
        .count()
}

fn part_2(grid: &PaperGrid) -> usize {
    let initial_occupied = grid.iter().filter(|(_xy, occupied)| **occupied).count();
    let mut grid = grid.clone();
    loop {
        let removed_rolls = grid
            .iter()
            .filter_map(|(xy, occupied)| {
                (*occupied && count_occupied_neighbours(&grid, xy) < 4).then_some((*xy, false))
            })
            .collect_vec();

        if removed_rolls.is_empty() {
            break;
        } else {
            grid.extend(removed_rolls);
        }
    }

    initial_occupied - grid.iter().filter(|(_xy, occupied)| **occupied).count()
}

pub fn solve() -> SolverResult {
    let input = read_to_string("inputs/day_04.txt").unwrap();
    let grid = parse_input(&input);

    println!("Part 1: {}", part_1(&grid));
    println!("Part 2: {}", part_2(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1_example() {
        let grid = parse_input(EXAMPLE.trim());
        assert_eq!(part_1(&grid), 13);
    }

    #[test]
    fn test_part_2_example() {
        let grid = parse_input(EXAMPLE.trim());
        assert_eq!(part_2(&grid), 43);
    }
}
