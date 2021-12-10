use advent_of_code::read_input;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
struct Shoaling {
    data: [usize; 9],
    shift: u8,
}

impl Index<u8> for Shoaling {
    type Output = usize;

    fn index(&self, index: u8) -> &Self::Output {
        if index > 8 {
            panic!("Index out of bounds: {}", index);
        } else {
            &self.data[self.real_index(index)]
        }
    }
}

impl IndexMut<u8> for Shoaling {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        if index > 8 {
            panic!("Index out of bounds: {}", index);
        } else {
            &mut self.data[self.real_index(index)]
        }
    }
}

impl Shoaling {
    fn from_ages(ages: Vec<u8>) -> Self {
        let mut data = [0; 9];
        for age in ages {
            data[age as usize] += 1;
        }
        Self { data, shift: 0 }
    }

    fn fish_count(&self) -> usize {
        self.data.iter().sum()
    }

    fn real_index(&self, index: u8) -> usize {
        let new_index = (index + self.shift) as usize;
        new_index % 9
    }

    fn progress_day(self) -> Self {
        let mut replacement = Self {
            data: self.data,
            shift: if self.shift == 8 {
                0
            } else {
                self.shift + 1
            },
        };
        replacement[6] += replacement[8];
        replacement
    }

    fn progress_n_days(self, days: u16) -> Self {
        (0..days).fold(self, |shoaling, _| shoaling.progress_day())
    }
}

fn get_init_state(input: String) -> Vec<u8> {
    input
        .split(',')
        .flat_map(|numstr| numstr.parse::<u8>())
        .collect()
}

fn main() {
    let shoaling = Shoaling::from_ages(get_init_state(read_input!()));
    println!("Lanternfish count day 0: {}", shoaling.fish_count());
    let shoaling = shoaling.progress_n_days(80);
    println!("Lanternfish count day 80: {}", shoaling.fish_count());
    let shoaling = shoaling.progress_n_days(256 - 80);
    println!("Lanternfish count day 256: {}", shoaling.fish_count());

}

#[cfg(test)]
mod tests {
    use crate::{get_init_state, Shoaling};

    const RAW_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_shoaling_translation() {
        let mut shoaling = Shoaling::default();

        shoaling.data[3] = 1;
        assert_eq!(1, shoaling[3]);
        shoaling = shoaling.progress_day();
        assert_eq!(0, shoaling[3]);
        assert_eq!(1, shoaling[2]);
        shoaling = shoaling.progress_day();
        shoaling = shoaling.progress_day();
        assert_eq!(1, shoaling[0]);
        shoaling = shoaling.progress_day();
        assert_eq!(1, shoaling[8]);
    }

    #[test]
    fn test_lanternfish() {
        assert_eq!(
            5934,
            Shoaling::from_ages(get_init_state(RAW_INPUT.into()))
                .progress_n_days(80)
                .fish_count()
        )
    }

    #[test]
    fn test_takeover() {
        assert_eq!(
            26984457539,
            Shoaling::from_ages(get_init_state(RAW_INPUT.into()))
                .progress_n_days(256)
                .fish_count()
        )
    }
}
