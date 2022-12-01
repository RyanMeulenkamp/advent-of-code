use std::collections::HashSet;
use std::convert::TryInto;
use advent_of_code::read_input;

fn main() {
    println!(
        "Total: {}",
        plan_path(parse_input(read_input!())),
    )
}

fn parse_input(input: String) -> Vec<Move> {
    input.chars().map(|movement| movement.try_into().unwrap()).collect()
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Self::Up,
            'v' | 'V' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => return Err(())
        })
    }
}

impl Move {
    fn axis(&self) -> usize {
        match self {
            Self::Left | Self::Right => 0,
            Self::Up | Self::Down => 1,
        }
    }

    fn movement(&self) -> isize {
        match self {
            Self::Left | Self::Down => -1,
            Self::Up | Self::Right => 1,
        }
    }

    fn apply(&self, mut coordinate: [isize; 2]) -> [isize; 2] {
        coordinate[self.axis()] += self.movement();
        coordinate
    }
}

fn plan_path(movements: Vec<Move>) -> usize {
    movements.into_iter().fold((HashSet::from([[0isize; 2]]), [0isize; 2]), |(mut set, coordinate), movement| {
        let coordinate = movement.apply(coordinate);
        set.insert(coordinate);
        (set, coordinate)
    }).0.len()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, plan_path};

    const TEST_SET: [(&str, usize); 3] = [
        (">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)
    ];

    #[test]
    fn test_parentheses_count() {
        for (input, output) in TEST_SET {
            println!("Input: {input}, output: {output}");
            assert_eq!(output, plan_path(parse_input(input.to_string())));
        }
    }
}
