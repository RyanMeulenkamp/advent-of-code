use advent_of_code::read_input_lines;

fn diagnose(input: &[&str]) -> usize {
    let word_size = input[0].len();

    let mut one_count = vec![0usize; word_size];
    for line in input.iter() {
        for (bit, character) in line.chars().enumerate() {
            if character == '1' {
                one_count[bit] += 1;
            }
        }
    }
    let half = input.len() / 2;
    let mut gamma = 0;
    for (bit, count) in one_count.iter().enumerate() {
        if count > &half {
            gamma |= 1 << (word_size - 1 - bit);
        }
    }

    let epsilon = (!gamma) & ((!0usize) >> (usize::BITS as usize - word_size));

    gamma * epsilon
}

const O2_GEN: fn(&usize, &usize) -> bool = usize::ge;
const CO2_SCRUB: fn(&usize, &usize) -> bool = usize::lt;

fn criteria_filter(input: Vec<&str>, criteria: fn(&usize, &usize) -> bool, bit: usize) -> usize {
    // Split list into 0 and 1 at index 'bit'
    let lists: (Vec<&str>, Vec<&str>) = input
        .into_iter()
        .partition(|line| line.as_bytes()[bit] == b'0');

    // Choose appropriate list from result
    let result_list = if criteria(&lists.1.len(), &lists.0.len()) {
        lists.1
    } else {
        lists.0
    };

    // If list contains a single result, this is the answer
    if result_list.len() == 1 {
        // Turn binary string into number
        usize::from_str_radix(result_list[0], 2).unwrap()
    } else {
        // Otherwise, recurse with filtered list
        criteria_filter(result_list, criteria, bit + 1)
    }
}

fn main() {
    println!("Power usage: {}", diagnose(&read_input_lines!()));

    let oxygen_generator_rating = criteria_filter(read_input_lines!(), O2_GEN, 0);
    let co2_scrubber_rating = criteria_filter(read_input_lines!(), CO2_SCRUB, 0);
    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    println!("CO² scrubber rating: {}", co2_scrubber_rating);
    println!("Answer: {}", oxygen_generator_rating * co2_scrubber_rating)
}

#[cfg(test)]
mod tests {
    use crate::{criteria_filter, diagnose, CO2_SCRUB, O2_GEN};

    const TEST_SET: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_diagnose() {
        assert_eq!(198, diagnose(&TEST_SET.to_vec()));
    }

    #[test]
    fn test_criteria() {
        let oxygen_generator_rating = criteria_filter(TEST_SET.to_vec(), O2_GEN, 0);
        let co2_scrubber_rating = criteria_filter(TEST_SET.to_vec(), CO2_SCRUB, 0);
        println!("Oxygen generator rating: {}", oxygen_generator_rating);
        println!("CO² scrubber rating: {}", co2_scrubber_rating);
        assert_eq!(230, oxygen_generator_rating * co2_scrubber_rating);
    }
}
