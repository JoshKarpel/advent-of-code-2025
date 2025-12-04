use crate::utils::SolverResult;
use itertools::Itertools;
use std::fs::read_to_string;

fn largest_joltage(bank: &str, count: usize) -> usize {
    let batteries = bank.chars().collect_vec();

    let mut turned_on: Vec<char> = vec![];
    let mut start = 0;

    for n in (0..count).rev() {
        let (max_battery_idx, max_battery) = batteries[start..batteries.len() - n]
            .iter()
            .enumerate()
            .rev() // Iterate from the end to find the *first* maximum value instead of the last
            .max_by_key(|(_, battery)| *battery)
            .unwrap();

        turned_on.push(*max_battery);
        start += max_battery_idx + 1; // += because the enumerated indices are relative to the sliced array starting at `start`
    }

    turned_on.into_iter().join("").parse().unwrap()
}

fn part_1(banks: &[&str]) -> usize {
    banks.iter().map(|bank| largest_joltage(bank, 2)).sum()
}

fn part_2(banks: &[&str]) -> usize {
    banks.iter().map(|bank| largest_joltage(bank, 12)).sum()
}

pub fn solve() -> SolverResult {
    let input = read_to_string("inputs/day_03.txt")?;
    let banks = input.lines().collect_vec();

    println!("Part 1: {}", part_1(&banks));
    println!("Part 2: {}", part_2(&banks));

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
    #[case("919000000000000", 99)]
    fn test_largest_joltage_part_1(#[case] batteries: &str, #[case] expected: usize) {
        assert_eq!(largest_joltage(batteries, 2), expected);
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_largest_joltage_part_2(#[case] batteries: &str, #[case] expected: usize) {
        assert_eq!(largest_joltage(batteries, 12), expected);
    }
}
