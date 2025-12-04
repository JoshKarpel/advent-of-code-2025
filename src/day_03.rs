use crate::utils::SolverResult;
use itertools::Itertools;
use std::fs::read_to_string;

fn largest_joltage_part_1(bank: &str) -> usize {
    let batteries = bank.chars().collect_vec();

    let (max_battery_idx, max_battery) = batteries[0..batteries.len() - 1]
        .iter()
        .enumerate()
        .rev() // Iterate from the end to find the *first* maximum value instead of the last
        .max_by_key(|(_, battery)| *battery)
        .unwrap();

    let max_of_rest = batteries[max_battery_idx + 1..].iter().max().unwrap();

    format!("{max_battery}{max_of_rest}").parse().unwrap()
}

fn part_1(banks: &[&str]) -> usize {
    banks.iter().map(|bank| largest_joltage_part_1(bank)).sum()
}

fn part_2() -> usize {
    0
}

pub fn solve() -> SolverResult {
    let input = read_to_string("inputs/day_03.txt")?;
    let banks = input.lines().collect_vec();

    println!("Part 1: {}", part_1(&banks));
    println!("Part 2: {}", part_2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("9190", 99)]
    fn test_largest_joltage_part_1(#[case] batteries: &str, #[case] expected: usize) {
        assert_eq!(largest_joltage_part_1(batteries), expected);
    }
}
