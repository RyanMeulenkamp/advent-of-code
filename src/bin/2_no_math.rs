use advent_of_code::read_input_lines;

fn main() {
    println!(
        "Total: {}",
        parse_input(read_input_lines!())
            .into_iter()
            .map(|dimensions| calculate_sides(dimensions))
            .map(|sides| calculate_area(sides) + calculate_slack(sides))
            .sum::<usize>()
    )
}

fn parse_input(input: Vec<&str>) -> Vec<[usize; 3]> {
    let mut result = Vec::new();
    for line in input {
        result.push(
            line.split("x")
                .map(|dimension| dimension.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap()
        )
    }
    result
}

fn calculate_sides(dimensions: [usize; 3]) -> [usize; 3] {
    [
        dimensions[0] * dimensions[1],
        dimensions[1] * dimensions[2],
        dimensions[2] * dimensions[0],
    ]
}

fn calculate_area(sides: [usize; 3]) -> usize {
    sides.map(|sides| sides * 2).into_iter().sum()
}

fn calculate_slack(sides: [usize; 3]) -> usize {
    sides.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_area, calculate_sides, calculate_slack};

    const TEST_SET: [([usize; 3], usize, usize); 2] = [
        ([2, 3, 4], 52, 6), ([1, 1, 10], 42, 1)
    ];

    #[test]
    fn test_parentheses_count() {
        for test_case in TEST_SET {
            let sides = calculate_sides(test_case.0);
            assert_eq!(test_case.1, calculate_area(sides));
            assert_eq!(test_case.2, calculate_slack(sides));
        }
    }
}
