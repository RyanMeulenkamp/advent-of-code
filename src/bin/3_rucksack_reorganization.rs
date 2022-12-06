use advent_of_code::read_input_lines;
use array_tool::vec::Intersect;
use itertools::Itertools;

fn score(character: char) -> usize {
    if character.is_uppercase() {
        character as usize - 38
    } else {
        character as usize - 96
    }
}

fn intersection(chunks: Vec<Vec<char>>) -> char {
    chunks
        .into_iter()
        .reduce(|one, another| one.intersect(another))
        .unwrap()[0]
}

fn chunks<const COUNT: usize>(rucksack: &str) -> Vec<Vec<char>> {
    rucksack
        .chars()
        .chunks(rucksack.len() / COUNT)
        .into_iter()
        .map(|chunk| chunk.into_iter().collect::<Vec<char>>())
        .collect()
}

fn reorganize_rucksacks(rucksacks: Vec<&str>) -> usize {
    rucksacks
        .into_iter()
        .map(chunks::<2>)
        .map(intersection)
        .map(score)
        .sum()
}

fn group_elves(rucksacks: Vec<&str>) -> usize {
    rucksacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .into_iter()
                .map(|rucksack| rucksack.chars().collect())
                .collect()
        })
        .map(intersection)
        .map(score)
        .sum()
}

fn main() {
    println!("Score(1): {}", reorganize_rucksacks(read_input_lines!()));
    println!("Score(2): {}", group_elves(read_input_lines!()));
}

#[cfg(test)]
mod tests {
    use crate::{group_elves, reorganize_rucksacks};
    use indoc::indoc;

    const TEST_SET: &str = indoc! {
        "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        "
    };

    #[test]
    fn test_rucksack_reorganization() {
        assert_eq!(157, reorganize_rucksacks(TEST_SET.lines().collect()));
        assert_eq!(70, group_elves(TEST_SET.lines().collect()));
    }
}
