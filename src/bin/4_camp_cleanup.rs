use advent_of_code::read_input_lines;
use std::ops::RangeInclusive;

type Pair = [RangeInclusive<usize>; 2];

fn fully_includes([one_range, another_range]: Pair) -> bool {
    one_range.start() >= another_range.start() && one_range.end() <= another_range.end()
        || another_range.start() >= one_range.start() && another_range.end() <= one_range.end()
}

fn partially_overlaps([one_range, another_range]: Pair) -> bool {
    one_range.contains(another_range.start())
        || one_range.contains(another_range.end())
        || another_range.contains(one_range.start())
        || another_range.contains(one_range.end())
}

fn camp_cleanup(pairs: Vec<&str>, overlap_checker: fn(Pair) -> bool) -> usize {
    pairs
        .into_iter()
        .filter(|pair| {
            overlap_checker(
                pair.split(",")
                    .map(|range| {
                        let [start, end]: [usize; 2] = range
                            .split("-")
                            .map(|operand| operand.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                            .try_into()
                            .unwrap();
                        start..=end
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .count()
}

fn main() {
    println!(
        "Full overlaps: {}",
        camp_cleanup(read_input_lines!(), fully_includes)
    );
    println!(
        "Full overlaps: {}",
        camp_cleanup(read_input_lines!(), partially_overlaps)
    );
}

#[cfg(test)]
mod tests {
    use crate::{camp_cleanup, fully_includes, partially_overlaps};
    use indoc::indoc;

    const TEST_SET: &str = indoc! {
        "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        "
    };

    #[test]
    fn test_rucksack_reorganization() {
        assert_eq!(2, camp_cleanup(TEST_SET.lines().collect(), fully_includes));
        assert_eq!(
            4,
            camp_cleanup(TEST_SET.lines().collect(), partially_overlaps)
        );
    }
}
