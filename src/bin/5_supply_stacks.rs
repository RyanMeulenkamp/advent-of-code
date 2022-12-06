use advent_of_code::{read_input, read_input_lines};
use itertools::Itertools;
use std::ops::RangeInclusive;

fn transpose(rows: Vec<Vec<Option<char>>>, length: usize) -> Vec<Vec<char>> {
    (0..length)
        .map(|i| {
            rows.iter()
                .filter_map(|row| row.get(i))
                .filter_map(|a| *a)
                .collect()
        })
        .collect()
}

fn parse_stacks(stacks: Vec<&str>) -> Vec<Vec<char>> {
    let stack_count = stacks
        .last()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let max_stack_height = stacks.len() - 1;
    transpose(
        stacks
            .into_iter()
            .take(max_stack_height)
            .map(|row| {
                row.chars()
                    .chunks(4)
                    .into_iter()
                    .map(|mut krat| krat.nth(1).filter(|krat| krat.is_alphabetic()))
                    .collect()
            })
            .collect(),
        stack_count,
    )
}

#[derive(Debug, Clone, Copy)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn apply_9000(&self, stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut stacks = stacks.clone();
        for _ in 0..self.count {
            let element = stacks[self.from - 1].remove(0);
            stacks[self.to - 1].insert(0, element);
        }
        stacks
    }

    fn apply_9001(&self, stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut stacks = stacks.clone();
        for i in (0..self.count).rev() {
            let element = stacks[self.from - 1].remove(i);
            stacks[self.to - 1].insert(0, element);
        }
        stacks
    }
}

impl From<[usize; 3]> for Move {
    fn from([count, from, to]: [usize; 3]) -> Self {
        Move { count, from, to }
    }
}

impl FromIterator<usize> for Move {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let movement: [usize; 3] = iter.into_iter().collect::<Vec<usize>>().try_into().unwrap();
        movement.into()
    }
}

fn parse_moves(moves: Vec<&str>) -> Vec<Move> {
    moves
        .into_iter()
        .map(|movement| {
            movement
                .split(" ")
                .filter_map(|movement| movement.parse::<usize>().ok())
                .collect()
        })
        .collect()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    (
        parse_stacks(parts[0].lines().collect()),
        parse_moves(parts[1].lines().collect()),
    )
}

type MoveFn = fn(&Move, Vec<Vec<char>>) -> Vec<Vec<char>>;

fn apply_moves(stacks: Vec<Vec<char>>, moves: Vec<Move>, move_fn: MoveFn) -> Vec<Vec<char>> {
    moves
        .into_iter()
        .fold(stacks, |stacks, movement| move_fn(&movement, stacks))
}

fn get_code(stacks: Vec<Vec<char>>) -> String {
    stacks.into_iter().map(|stack| stack[0]).collect()
}

fn supply_stacks(input: &str, move_fn: MoveFn) -> String {
    let (stacks, moves) = parse_input(&input);
    get_code(apply_moves(stacks, moves.clone(), move_fn))
}

fn main() {
    println!(
        "Code: {}",
        supply_stacks(read_input!().as_str(), Move::apply_9000)
    );
    println!(
        "Code: {}",
        supply_stacks(read_input!().as_str(), Move::apply_9001)
    );
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, supply_stacks, Move};
    use indoc::indoc;

    const TEST_SET: &str = indoc! {
        "
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
        "
    };

    #[test]
    fn test_supply_stacks() {
        assert_eq!("CMZ", supply_stacks(TEST_SET, Move::apply_9000),);
        assert_eq!("MCD", supply_stacks(TEST_SET, Move::apply_9001),);
    }
}
