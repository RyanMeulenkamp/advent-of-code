use advent_of_code::read_input;

fn count_calories(input: Vec<Vec<usize>>, top_n: usize) -> usize {
    let mut vec: Vec<usize> = input
        .into_iter()
        .map(|calories| calories.into_iter().sum())
        .collect();
    vec.sort();
    vec.reverse();
    vec[0..top_n].into_iter().sum()
}

fn parse_lines(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    println!("Answer 1: {:?}", count_calories(parse_lines(&read_input!()), 1));
    println!("Answer 2: {:?}", count_calories(parse_lines(&read_input!()), 3));
}

#[cfg(test)]
mod tests {
    use crate::{count_calories, parse_lines};

    const TEST_SET: &str = "
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
";

    #[test]
    fn test_calory_count() {
        assert_eq!(24000, count_calories(parse_lines(TEST_SET), 1));
        assert_eq!(45000, count_calories(parse_lines(TEST_SET), 3));
    }
}