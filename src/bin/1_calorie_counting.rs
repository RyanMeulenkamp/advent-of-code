use advent_of_code::read_input;
use itertools::Itertools;

fn count_calories(input: Vec<Vec<usize>>, top_n: usize) -> usize {
    input
        .into_iter()
        .map(|calories| calories.into_iter().sum())
        .sorted_by(|a, b| usize::cmp(b, a))
        .take(top_n)
        .sum()
}

fn parse_lines(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    println!(
        "Answer 1: {:?}",
        count_calories(parse_lines(&read_input!()), 1)
    );
    println!(
        "Answer 2: {:?}",
        count_calories(parse_lines(&read_input!()), 3)
    );
}

#[cfg(test)]
mod tests {
    use crate::{count_calories, parse_lines};
    use indoc::indoc;

    const TEST_SET: &str = indoc! {
        "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
        "
    };

    #[test]
    fn test_calorie_count() {
        assert_eq!(24000, count_calories(parse_lines(TEST_SET), 1));
        assert_eq!(45000, count_calories(parse_lines(TEST_SET), 3));
    }
}
