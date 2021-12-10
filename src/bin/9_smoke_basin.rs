use advent_of_code::read_input;
use colored::{Color, Colorize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone)]
struct Point {
    row: usize,
    col: usize,
    height: usize,
}

impl Point {
    fn new(row: usize, col: usize, risk: usize) -> Self {
        Point {
            row,
            col,
            height: risk,
        }
    }

    fn risk(&self) -> usize {
        self.height + 1
    }
}

struct HeightMap {
    rows: Vec<Vec<usize>>,
    low_points: Vec<Point>,
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rows
                .iter()
                .enumerate()
                .map(|(row, cols)| cols
                    .iter()
                    .enumerate()
                    .map(|(col, height)| {
                        let is_low_point = self.low_points.contains(&Point::new(row, col, *height));
                        if is_low_point { "🭭 " } else { "██" }
                            .color(Color::TrueColor {
                                r: 0,
                                g: if is_low_point { 255 } else { 0 },
                                b: if is_low_point { 0 } else { 15 * *height as u8 },
                            })
                            .on_black()
                    })
                    .fold(String::new(), |string, character| string
                        + character.to_string().as_str()))
                .fold(String::new(), |a, b| format!("{}\n{}", a, b)),
        )
    }
}

fn find_low_points(input: &[Vec<usize>]) -> Vec<Point> {
    let mut result = Vec::new();

    for (row_index, row) in input.iter().enumerate() {
        for (col, point) in row.iter().enumerate() {
            let vertically_lowest = if row_index == 0 {
                point < &input[row_index + 1][col]
            } else if row_index == input.len() - 1 {
                point < &input[row_index - 1][col]
            } else {
                point < &input[row_index + 1][col] && point < &input[row_index - 1][col]
            };

            let horizontally_lowest = if col == 0 {
                point < &input[row_index][col + 1]
            } else if col == row.len() - 1 {
                point < &input[row_index][col - 1]
            } else {
                point < &input[row_index][col + 1] && point < &input[row_index][col - 1]
            };

            if vertically_lowest && horizontally_lowest {
                result.push(Point::new(row_index, col, *point));
            }
        }
    }

    result
}

fn trace_low_point(
    map: &[Vec<usize>],
    current_position: (usize, usize),
) -> HashSet<(usize, usize)> {
    let current_height = map[current_position.0][current_position.1];
    if current_height == 9 {
        HashSet::new()
    } else {
        let mut result = HashSet::new();
        result.insert(current_position);
        if current_position.0 + 1 < map.len()
            && map[current_position.0 + 1][current_position.1] > current_height
        {
            result.extend(trace_low_point(
                map,
                (current_position.0 + 1, current_position.1),
            ));
        }
        if current_position.1 + 1 < map[current_position.0].len()
            && map[current_position.0][current_position.1 + 1] > current_height
        {
            result.extend(trace_low_point(
                map,
                (current_position.0, current_position.1 + 1),
            ));
        }
        if current_position.0 != 0
            && map[current_position.0 - 1][current_position.1] > current_height
        {
            result.extend(trace_low_point(
                map,
                (current_position.0 - 1, current_position.1),
            ))
        }
        if current_position.1 != 0
            && map[current_position.0][current_position.1 - 1] > current_height
        {
            result.extend(trace_low_point(
                map,
                (current_position.0, current_position.1 - 1),
            ))
        }
        result
    }
}

fn find_basins(
    input: &[Vec<usize>],
    low_points: Vec<(usize, usize)>,
) -> BTreeMap<(usize, usize), usize> {
    low_points
        .into_iter()
        .enumerate()
        .flat_map(|(basin, low_point)| {
            trace_low_point(input, low_point)
                .into_iter()
                .map(move |coordinates| (coordinates, basin))
        })
        .collect()
}

fn read_input_to_map(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            row.trim()
                .chars()
                .flat_map(|character| format!("{}", character).parse::<usize>())
                .collect()
        })
        .collect()
}

fn basin_frequencies(input: BTreeMap<(usize, usize), usize>) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    for (_, basin) in input {
        *result.entry(basin).or_insert(0) += 1;
    }

    result
}

fn main() {
    let input = read_input_to_map(&*read_input!());
    let mut height_map = HeightMap {
        rows: input.clone(),
        low_points: Vec::new(),
    };
    println!("{}", height_map);
    let low_points = find_low_points(&input);

    height_map.low_points = low_points.clone();
    println!("{}", height_map);

    let result: usize = low_points.iter().map(|point| point.risk()).sum();
    println!("Risk: {}", result);
    let basins = find_basins(
        &input,
        low_points
            .iter()
            .map(|point| (point.row, point.col))
            .collect(),
    );
    let basin_frequencies = basin_frequencies(basins);
    let mut basin_sizes: Vec<usize> = basin_frequencies.values().copied().collect();
    basin_sizes.sort_unstable();
    println!(
        "Result: {}",
        basin_sizes.iter().rev().take(3).product::<usize>()
    );
}
// wrong: 1103780

#[cfg(test)]
mod tests {
    use crate::{basin_frequencies, find_basins, find_low_points, read_input_to_map};

    const TEST_SET: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_low_points() {
        let input = read_input_to_map(TEST_SET);
        let result = find_low_points(&input);
        assert_eq!(15usize, result.iter().map(|point| point.risk()).sum());
    }

    #[test]
    fn test_basins() {
        let input = read_input_to_map(TEST_SET);

        let low_points = find_low_points(&input);
        let basins = find_basins(
            &input,
            low_points
                .iter()
                .map(|point| (point.row, point.col))
                .collect(),
        );
        let mut clone = input.clone();
        for (coordinates, basin) in basins.iter() {
            clone[coordinates.0][coordinates.1] = *basin;
        }
        for row in clone.iter() {
            for col in row {
                print!("{}", col)
            }
            println!()
        }
        let basin_frequencies = basin_frequencies(basins);
        let mut basin_sizes: Vec<usize> = basin_frequencies.values().map(|a| *a).collect();
        basin_sizes.sort();
        let result = basin_sizes
            .iter()
            .rev()
            .take(3)
            .inspect(|a| println!("{}", a))
            .product::<usize>();
        assert_eq!(1134, result)
    }
}
