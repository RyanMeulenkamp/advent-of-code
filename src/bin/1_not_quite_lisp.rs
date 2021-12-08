use advent_of_code::read_input;

fn count_parentheses(input: &String) -> isize {
    input.chars().fold(0, |result, b| if b == '(' {
        result + 1
    } else {
        result - 1
    })
}

fn find_basement_stop(input: String) -> usize {
    let mut floor = 0;
    for (index, character) in input.chars().enumerate() {
        if character == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            return index + 1;
        }
    }
    panic!("Should not be reached!")
}

fn main() {
    let input = read_input!();
    println!("Floor: {}", count_parentheses(&input));
    println!("Basement stop: {}", find_basement_stop(input))
}

#[cfg(test)]
mod tests {
    use crate::{count_parentheses};

    const TEST_SET: [(&str, isize); 9] = [
        ("(())", 0), ("()()", 0), ("(((", 3), ("(()(()(", 3), ("))(((((", 3), ("())", -1),
        ("))(", -1), (")))", -3), (")())())", -3)
    ];

    #[test]
    fn test_parentheses_count() {
        for (input, output) in TEST_SET {
            assert_eq!(output, count_parentheses(input.to_string()));
        }
    }
}
