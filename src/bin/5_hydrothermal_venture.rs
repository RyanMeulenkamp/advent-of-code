#![feature(step_trait)]
#![feature(int_abs_diff)]

use advent_of_code::read_input_lines;
use std::cmp::Ordering;
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = string.splitn(2, ',').collect();
        if let (Ok(x), Ok(y)) = (split[0].parse(), split[1].parse()) {
            Ok(Point::new(x, y))
        } else {
            Err("Please pass two numbers separated by a comma!")
        }
    }
}

#[derive(Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn _is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn length(&self) -> usize {
        if self.is_vertical() {
            self.start.y.abs_diff(self.end.y)
        } else {
            self.start.x.abs_diff(self.end.x)
        }
    }

    fn generate_1d_points(start: usize, end: usize, length: usize) -> Vec<isize> {
        match start.cmp(&end) {
            Ordering::Less => RangeInclusive::new(start as isize, end as isize).collect(),
            Ordering::Greater => RangeInclusive::new(end as isize, start as isize)
                .rev()
                .collect(),
            Ordering::Equal => vec![start as isize; length + 1],
        }
    }

    fn points(&self) -> Vec<Point> {
        let length = self.length();
        let x_s: Vec<isize> = Self::generate_1d_points(self.start.x, self.end.x, length);
        let y_s: Vec<isize> = Self::generate_1d_points(self.start.y, self.end.y, length);
        x_s.into_iter()
            .zip(y_s.into_iter())
            .map(|(x, y)| Point::new(x as usize, y as usize))
            .collect()
    }
}

impl From<Line> for Range<Point> {
    fn from(line: Line) -> Self {
        line.start..line.end
    }
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = string.splitn(2, "->").collect();
        if let (Ok(start), Ok(end)) = (split[0].trim().parse(), split[1].trim().parse()) {
            Ok(Line::new(start, end))
        } else {
            Err("Please pass two (comma separated) coordinates separated by a arrow!")
        }
    }
}

fn generate_field(lines: Vec<Line>) -> Vec<Vec<usize>> {
    let flatten: Vec<Point> = lines.iter().flat_map(|line| line.points()).collect();
    let x_size = flatten.iter().map(|point| point.x).max().unwrap() + 1;
    let y_size = flatten.iter().map(|point| point.y).max().unwrap() + 1;

    let mut field = vec![vec![0usize; x_size]; y_size];

    for point in lines.into_iter().flat_map(|line| line.points()) {
        field[point.y][point.x] += 1;
    }

    for row in field.clone() {
        for point in row {
            if point == 0 {
                print!("░",);
            } else if point == 1 {
                print!("▒");
            } else {
                print!("█");
            }
        }
        println!();
    }
    println!();

    field
}

fn count_intersections(field: Vec<Vec<usize>>) -> usize {
    field
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|count| count > &1)
        .count()
}

fn input_to_lines(input: Vec<&str>) -> Vec<Line> {
    input.iter().flat_map(|line| line.parse::<Line>()).collect()
}

fn main() {
    let lines = input_to_lines(read_input_lines!());
    let field = generate_field(lines);
    let intersections = count_intersections(field);
    println!("Intersections: {}", intersections);
}

#[cfg(test)]
mod tests {
    use crate::{count_intersections, generate_field, input_to_lines};

    const RAW_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_straight_intersection_count() {
        let lines = input_to_lines(RAW_INPUT.split('\n').collect())
            .into_iter()
            .filter(|line| line._is_horizontal() || line.is_vertical())
            .collect();
        let field = generate_field(lines);
        let intersections = count_intersections(field);

        assert_eq!(5, intersections)
    }

    #[test]
    fn test_all_intersection_count() {
        let lines = input_to_lines(RAW_INPUT.split('\n').collect());
        let field = generate_field(lines);
        let intersections = count_intersections(field);

        assert_eq!(12, intersections)
    }
}
