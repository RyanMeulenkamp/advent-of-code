use advent_of_code::read_input_ints;

fn sonar_sweep(input: Vec<u32>) -> usize {
    input
        .as_slice()
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn filtered_sonar_sweep(input: Vec<u32>) -> usize {
    sonar_sweep(
        input
            .as_slice()
            .windows(3)
            .map(|window| window.iter().sum())
            .collect(),
    )
}

fn main() {
    println!("{:?}", filtered_sonar_sweep(read_input_ints!()));
}

#[cfg(test)]
mod tests {
    use crate::{filtered_sonar_sweep, sonar_sweep};

    const TEST_SET: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_sonar_sweep() {
        assert_eq!(7, sonar_sweep(TEST_SET.to_vec()));
    }

    #[test]
    fn test_filtered_sonar_sweep() {
        assert_eq!(5, filtered_sonar_sweep(TEST_SET.to_vec()));
    }
}
