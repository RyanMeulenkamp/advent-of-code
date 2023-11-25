use advent_of_code::read_input;
use itertools::Itertools;
use std::assert_eq;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Clone)]
struct Grid<T: Display>(Vec<Vec<T>>);

impl<T: Display> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Display> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

fn tree_mapper(height: i32) -> char {
    match height {
        0 => '🌱',
        1 => '🎋',
        2 => '🌾',
        3 => '🌵',
        4 => '🌿',
        5 => '🌴',
        6 => '🥦',
        7 => '🌳',
        8 => '🎄',
        9 => '🌲',
        _ => '⚫',
    }
}

fn color_mapper(height: i32) -> char {
    match height {
        0 => '⬜',
        1 => '🟨',
        2 => '🟧',
        3 => '🟪',
        4 => '🟩',
        5 => '🟥',
        6 => '🟦',
        7 => '🟫',
        8 => '⬛',
        9 => '🔳',
        _ => '⚫',
    }
}

fn height_mapper(height: i32) -> char {
    match height {
        0 => '▁',
        1 => '▂',
        2 => '▃',
        3 => '▄',
        4 => '▅',
        5 => '▆',
        6 => '▇',
        7 => '█',
        8 => '▉',
        9 => '▊',
        _ => '⚫',
    }
}

impl Display for Grid<i32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "┌{}┐", "─".repeat(self.0.len() + 2))?;
        for row in &self.0 {
            // write!(f, "│ ")?;
            writeln!(f)?;
            for height in row {
                write!(f, "{}", color_mapper(*height))?;
            }
            // writeln!(f, " │")?;
        }
        // writeln!(f, "└{}┘", "─".repeat(self.0.len() + 2))?;
        Ok(())
    }
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "┌{}┐", "─".repeat(self.0.len() + 2))?;
        for row in &self.0 {
            // write!(f, "│ ")?;
            writeln!(f)?;
            for visible in row {
                write!(f, "{}", if *visible { "🔲️" } else { "🔳" })?;
            }
            // writeln!(f, " │")?;
        }
        // writeln!(f, "└{}┘", "─".repeat(self.0.len() + 2))?;
        Ok(())
    }
}

fn find_visible(grid: Grid<i32>) -> usize {
    println!("Grid: \n{grid}");
    let grid_size = grid.0.len();
    let mut visibility = Grid(vec![vec![false; grid_size]; grid_size]);

    // Square
    assert_eq!(grid_size, grid.0[0].len());

    for one_dimension in 0..grid_size {
        let mut biggest = [[-1; 2]; 2];
        for another_dimension in 0..grid_size {
            for (eendex, another_dimension) in
                [another_dimension, grid_size - another_dimension - 1]
                    .into_iter()
                    .enumerate()
            {
                for (tweedex, [row, col]) in [
                    [one_dimension, another_dimension],
                    [another_dimension, one_dimension],
                ]
                .into_iter()
                .enumerate()
                {
                    if grid[row][col] > biggest[eendex][tweedex] {
                        biggest[eendex][tweedex] = grid[row][col];
                        visibility[row][col] = true;
                    }
                }
            }
        }
    }
    println!("Visibility grid:\n{visibility}");
    visibility
        .0
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|tree| *tree)
        .count()
}

fn find_scenic_score(grid: Grid<i32>) -> usize {
    println!("Grid: \n{grid}");
    let grid_size = grid.0.len();
    let mut visibility = Grid(vec![vec![false; grid_size]; grid_size]);

    // Square
    assert_eq!(grid_size, grid.0[0].len());

    (0..grid_size)
        .flat_map(|row| {
            (0..grid_size)
                .map(move |col| (row, col))
                .map(|(row, col)| (row, col, grid[row][col]))
        })
        .map(|(row, col, our_height)| {
            let save_stuff_because_of_debugger = [
                (0..col + 1)
                    .into_iter()
                    .rev()
                    .map(|col| grid[row][col])
                    .take_while(|height| *height < our_height)
                    .count(),
                (col + 1..grid_size)
                    .into_iter()
                    .map(|col| grid[row][col])
                    .take_while(|height| *height < our_height)
                    .count(),
                (0..row + 1)
                    .into_iter()
                    .rev()
                    .map(|row| grid[row][col])
                    .take_while(|height| *height < our_height)
                    .count(),
                (row + 1..grid_size)
                    .into_iter()
                    .map(|row| grid[row][col])
                    .take_while(|height| *height < our_height)
                    .count(),
            ];
            save_stuff_because_of_debugger.into_iter().product()
        })
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> Grid<i32> {
    Grid(
        input
            .chars()
            .group_by(|character| *character == '\n')
            .into_iter()
            .filter(|(condition, _)| !*condition)
            .map(|(_, group)| {
                group
                    .map(|character| character.to_digit(10).map(|digit| digit as i32).unwrap())
                    .collect()
            })
            .collect(),
    )
}

fn main() {
    println!(
        "Visible trees: {}",
        find_visible(parse_input(&read_input!()))
    )
}

#[cfg(test)]
mod tests {
    use crate::{find_scenic_score, find_visible, parse_input};
    use indoc::indoc;

    const TEST_SET: &str = indoc! {
        "
        30373
        25512
        65332
        33549
        35390
        "
    };

    #[test]
    fn no_space_left_on_device() {
        let grid = parse_input(TEST_SET);
        assert_eq!(21, find_visible(grid.clone()));
        assert_eq!(8, find_scenic_score(grid));
    }
}
