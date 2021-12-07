use advent_of_code::read_input_lines;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let direction: Vec<&str> = string.split_whitespace().collect();
        if direction.len() != 2 {
            return Err("Not a direction;unit pair!");
        }
        let units = match direction[1].parse::<usize>() {
            Ok(units) => units,
            Err(_) => return Err("not an integer!"),
        };
        match direction[0] {
            "forward" => Ok(Direction::Forward(units)),
            "up" => Ok(Direction::Up(units)),
            "down" => Ok(Direction::Down(units)),
            _ => Err("Not one of: [forward, up, down]"),
        }
    }
}

struct Position {
    pos: usize,
    depth: usize,
    aim: usize,
}

impl Position {
    fn new(pos: usize, depth: usize, aim: usize) -> Self {
        Position { pos, depth, aim }
    }

    fn progress(self, direction: Direction) -> Self {
        match direction {
            Direction::Forward(units) => Self::new(self.pos + units, self.depth, self.aim),
            Direction::Up(units) => Self::new(self.pos, self.depth - units, self.aim),
            Direction::Down(units) => Self::new(self.pos, self.depth + units, self.aim),
        }
    }

    fn aim(self, direction: Direction) -> Self {
        match direction {
            Direction::Forward(units) => {
                Self::new(self.pos + units, self.depth + self.aim * units, self.aim)
            }
            Direction::Up(units) => Self::new(self.pos, self.depth, self.aim - units),
            Direction::Down(units) => Self::new(self.pos, self.depth, self.aim + units),
        }
    }

    fn product(self) -> usize {
        self.pos * self.depth
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new(0, 0, 0)
    }
}

fn dive(input: &[Direction]) -> usize {
    input
        .iter()
        .fold(Position::default(), |pos, direction| {
            pos.progress(*direction)
        })
        .product()
}

fn aim(input: &[Direction]) -> usize {
    input
        .iter()
        .fold(Position::default(), |pos, direction| pos.aim(*direction))
        .product()
}

fn main() {
    let input: Vec<Direction> = read_input_lines!()
        .iter()
        .flat_map(|line| line.parse::<Direction>())
        .collect();
    println!("Dive: {:?}", dive(&*input));
    println!("Aim: {:?}", aim(&*input));
}

#[cfg(test)]
mod tests {
    use crate::Direction::{Down, Forward, Up};
    use crate::{aim, dive, Direction};

    const TEST_SET: [Direction; 6] = [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];

    #[test]
    fn test_dive() {
        assert_eq!(150, dive(&TEST_SET.to_vec()));
    }

    #[test]
    fn test_aim() {
        assert_eq!(900, aim(&TEST_SET.to_vec()));
    }
}
