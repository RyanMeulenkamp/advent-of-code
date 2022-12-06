use advent_of_code::read_input;
use std::collections::HashSet;

fn tune<const WINDOW_SIZE: usize>(input: String) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(WINDOW_SIZE)
        .take_while(|window| window.into_iter().collect::<HashSet<_>>().len() != WINDOW_SIZE)
        .count()
        + WINDOW_SIZE
}

fn main() {
    println!("First: {}", tune::<4>(read_input!()));
    println!("First: {}", tune::<14>(read_input!()));
}

#[cfg(test)]
mod tests {
    use crate::tune;

    const TEST_SET: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn test_tuning_trouble() {
        for (input, expected) in TEST_SET {
            assert_eq!(expected, tune::<4>(input.to_string()),);
        }
    }
}
