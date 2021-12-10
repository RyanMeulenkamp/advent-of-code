#![feature(int_abs_diff)]

use advent_of_code::read_input;
use std::collections::HashMap;

fn _mean(positions: &[usize]) -> usize {
    positions.iter().sum::<usize>() / positions.len()
}

fn _median(positions: &[usize]) -> usize {
    let mut positions = positions.to_vec();
    positions.sort_unstable();
    positions[positions.len() / 2]
}

fn _mode(positions: &[usize]) -> usize {
    let mut counts = HashMap::new();

    positions
        .iter()
        .copied()
        .max_by_key(|&n| {
            let count = counts.entry(n).or_insert(0);
            *count += 1;
            *count
        })
        .unwrap()
}

fn trivial_fuel_function(moved_positions: usize) -> usize {
    moved_positions
}

fn triangular_fuel_function(moved_positions: usize) -> usize {
    ((moved_positions as f32 / 2.0) * (moved_positions as f32 + 1.0)) as usize
}

fn align_crabs(
    positions: &[usize],
    fuel_function: fn(usize) -> usize,
    destination: usize,
) -> usize {
    positions
        .iter()
        .map(|position| position.abs_diff(destination))
        .map(fuel_function)
        .sum()
}

fn optimize_alignment(positions: &[usize], fuel_function: fn(usize) -> usize) -> (usize, usize) {
    let lowest = *positions.into_iter().min().unwrap();
    let highest = *positions.into_iter().max().unwrap();
    (lowest..=highest)
        .map(|statistic| (statistic, align_crabs(positions, fuel_function, statistic)))
        .min_by_key(|(_, fuel_consumption)| *fuel_consumption)
        .unwrap()
}

fn crab_positions(input: &str) -> Vec<usize> {
    input
        .split(',')
        .flat_map(|numstr| numstr.parse::<usize>())
        .collect()
}

fn main() {
    let crab_positions = crab_positions(read_input!().as_str());
    let (position, trivial_fuel_consumption) =
        optimize_alignment(&crab_positions, trivial_fuel_function);
    println!(
        "Trivial fuel consumption of {} when moving to {}",
        trivial_fuel_consumption, position
    );
    let (position, triangular_fuel_consumption) =
        optimize_alignment(&crab_positions, triangular_fuel_function);
    println!(
        "Triangular fuel consumption of {} when moving to {}",
        triangular_fuel_consumption, position
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        crab_positions, optimize_alignment, triangular_fuel_function, trivial_fuel_function,
    };

    const TEST_SET: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_crab_alignment() {
        let crab_positions = crab_positions(TEST_SET);
        let (position, fuel_consumption) =
            optimize_alignment(&crab_positions, trivial_fuel_function);
        assert_eq!(2, position);
        assert_eq!(37, fuel_consumption);
    }

    #[test]
    fn test_alternative_crab_alignment() {
        let crab_positions = crab_positions(TEST_SET);
        let (position, fuel_consumption) =
            optimize_alignment(&crab_positions, triangular_fuel_function);
        assert_eq!(5, position);
        assert_eq!(168, fuel_consumption);
    }
}
