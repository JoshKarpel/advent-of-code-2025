use crate::utils::SolverResult;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::Sub;

type PaperGrid = HashSet<(isize, isize)>;

fn parse_input(input: &str) -> PaperGrid {
    let mut grid = PaperGrid::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '@' {
                grid.insert((x as isize, y as isize));
            }
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
        .filter(|(dx, dy)| grid.contains(&(x + dx, y + dy)))
        .count()
}

fn part_1(grid: &PaperGrid) -> usize {
    grid.iter()
        .filter(|&xy| count_occupied_neighbours(grid, xy) < 4)
        .count()
}

fn part_2(grid: &PaperGrid) -> usize {
    let initial_occupied = grid.len();
    let mut grid = grid.clone();
    loop {
        let removed_rolls: PaperGrid = grid
            .iter()
            .filter_map(|xy| (count_occupied_neighbours(&grid, xy) < 4).then_some(*xy))
            .collect();

        if removed_rolls.is_empty() {
            break;
        } else {
            grid = grid.sub(&removed_rolls);
        }
    }

    initial_occupied - grid.len()
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
