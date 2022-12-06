extern crate core;

use crate::Choice::{Paper, Rock, Scissors};
use crate::Outcome::{Draw, Loose, Win};
use advent_of_code::read_input_lines;
use core::panicking::panic;
use debugless_unwrap::*;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_uppercase().as_str() {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Nat good!"),
        })
    }
}

impl Score for Choice {
    fn score(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl Choice {
    fn increment(&self) -> Self {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn decrement(&self) -> Self {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

impl PartialOrd<Self> for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Ordering::Greater,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Ordering::Less,
            _ => Ordering::Equal,
        })
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Loose,
    Draw,
    Win,
}

trait Score {
    fn score(&self) -> usize;
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_uppercase().as_str() {
            "X" => Loose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Nat good!"),
        })
    }
}

impl Score for Outcome {
    fn score(&self) -> usize {
        match self {
            Loose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

impl Outcome {
    fn reverse(&self, choice: Choice) -> Choice {
        match self {
            Draw => choice,
            Win => choice.increment(),
            Loose => choice.decrement(),
        }
    }
}

struct Game<Part> {
    foe: Choice,
    var: Part,
}

impl From<Ordering> for Outcome {
    fn from(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Greater => Win,
            Ordering::Equal => Draw,
            Ordering::Less => Loose,
        }
    }
}

impl Score for Game<Choice> {
    fn score(&self) -> usize {
        self.var.score() + self.outcome().score()
    }
}

impl Game<Choice> {
    fn outcome(&self) -> Outcome {
        self.var.partial_cmp(&self.foe).unwrap().into()
    }
}

impl Score for Game<Outcome> {
    fn score(&self) -> usize {
        self.var.score() + self.var.reverse(self.foe).score()
    }
}

impl From<Vec<Choice>> for Game<Choice> {
    fn from(mut choices: Vec<Choice>) -> Self {
        Game {
            foe: choices.remove(0),
            var: choices.remove(0),
        }
    }
}

impl<Part: FromStr> FromStr for Game<Part> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        Ok(Game {
            foe: parts[0].parse().debugless_unwrap(),
            var: parts[1].parse().debugless_unwrap(),
        })
    }
}

fn parse_input<Part: FromStr>(input: Vec<&str>) -> Vec<Game<Part>>
where
    String: From<<Part as FromStr>::Err>,
{
    input
        .into_iter()
        .map(|game| game.parse().unwrap())
        .collect()
}

fn rock_paper_scissors<Part>(games: Vec<Game<Part>>) -> usize
where
    Game<Part>: Score,
{
    games.into_iter().map(|game| game.score()).sum()
}

fn main() {
    println!(
        "Score (1): {}",
        rock_paper_scissors(parse_input::<Choice>(read_input_lines!()))
    );
    println!(
        "Score (2): {}",
        rock_paper_scissors(parse_input::<Outcome>(read_input_lines!()))
    );
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, rock_paper_scissors, Choice, Outcome};
    use indoc::indoc;

    const TEST_SET: &str = indoc! {
        "
        A Y
        B X
        C Z
        "
    };

    #[test]
    fn test_rock_paper_scissors() {
        assert_eq!(
            15,
            rock_paper_scissors(parse_input::<Choice>(TEST_SET.trim().lines().collect()))
        );
        assert_eq!(
            12,
            rock_paper_scissors(parse_input::<Outcome>(TEST_SET.trim().lines().collect()))
        );
    }
}
