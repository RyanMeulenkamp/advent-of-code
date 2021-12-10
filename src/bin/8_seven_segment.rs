use advent_of_code::read_input;
use std::collections::HashMap;
use std::hash::Hash;
extern crate colored;
use colored::*;

#[derive(Clone)]
struct Display {
    map: HashMap<u8, String>,
}

trait WithInsert<K: Clone + Eq + Hash, V: Clone> {
    fn with_insert(self, key: K, value: V) -> Self;
}

impl<K: Clone + Eq + Hash, V: Clone> WithInsert<K, V> for HashMap<K, V> {
    fn with_insert(self, key: K, value: V) -> Self {
        let mut clone = self;
        clone.insert(key, value);
        clone
    }
}

impl Display {
    const KNOWN_SET: [usize; 4] = [2, 3, 4, 7];

    const HORIZONTAL_SEGMENT_CHARS: [&'static str; 2] = ["─", "━"];
    const VERTICAL_SEGMENT_CHARS: [&'static str; 2] = ["│", "┃"];

    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn from_notes(notes: Vec<&str>) -> Self {
        let (mut first_iteration, second_iteration): (Vec<&str>, Vec<&str>) = notes
            .into_iter()
            .partition(|note| Self::KNOWN_SET.contains(&note.len()));
        first_iteration.extend(second_iteration);
        first_iteration.iter().fold(Self::new(), |display, note| {
            display.process_information(note)
        })
    }

    fn overlap(a: &str, b: &str) -> u8 {
        b.chars().filter(|character| a.contains(*character)).count() as u8
    }

    fn decode_digit(&self, input: &str) -> Option<u8> {
        Some(match input.len() {
            // First iteration
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,

            // Second iteration
            5 => {
                if Self::overlap(input, &*self.map[&7]) == 3 {
                    3
                } else if Self::overlap(&*self.map[&4], input) == 2 {
                    2
                } else {
                    5
                }
            }

            6 => {
                if Self::overlap(input, &*self.map[&7]) != 3 {
                    6
                } else if Self::overlap(&*self.map[&4], input) == 4 {
                    9
                } else {
                    0
                }
            }

            _ => return None,
        })
    }

    fn process_information(self, input: &str) -> Self {
        if let Some(result) = self.decode_digit(input) {
            Self {
                map: self.map.with_insert(result, input.to_string()),
            }
        } else {
            self
        }
    }

    fn decode_number(self, input: Vec<&str>) -> usize {
        input
            .iter()
            .flat_map(|digit| self.decode_digit(digit))
            .fold(String::new(), |string, digit| {
                format!("{}{}", string, digit)
            })
            .parse::<usize>()
            .unwrap()
    }

    fn num_to_segments(num: u8) -> [bool; 7] {
        match num {
            1 => [false, true, true, false, false, false, false],
            2 => [true, true, false, true, true, false, true],
            3 => [true, true, true, true, false, false, true],
            4 => [false, true, true, false, false, true, true],
            5 => [true, false, true, true, false, true, true],
            6 => [true, false, true, true, true, true, true],
            7 => [true, true, true, false, false, false, false],
            8 => [true, true, true, true, true, true, true],
            9 => [true, true, true, true, false, true, true],
            0 => [true, true, true, true, true, true, false],
            _ => [false; 7],
        }
    }

    fn segment_to_character(segment: u8, status: bool, color: Color) -> ColoredString {
        let set = match segment {
            0 | 3 | 6 => Self::HORIZONTAL_SEGMENT_CHARS,
            _ => Self::VERTICAL_SEGMENT_CHARS,
        };
        if status {
            set[1].color(color)
        } else {
            set[0].black()
        }
    }

    fn zip_digits(one: String, another: String) -> String {
        one.trim()
            .split('\n')
            .zip(another.trim().split('\n'))
            .map(|(one, another)| format!("{}    {}", one, another))
            .fold(String::new(), |a, b| a + "\n" + b.as_str())
    }

    fn display(number: usize, color: Color) -> String {
        format!("{}", number)
            .chars()
            .flat_map(|character| character.to_digit(10).map(|digit| digit as u8))
            .map(|digit| Self::display_digit(digit, color))
            .reduce(Self::zip_digits)
            .unwrap()
    }

    fn display_digit(number: u8, color: Color) -> String {
        let segments = Self::num_to_segments(number);
        format!(
            "\
        ╭{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}╮\n\
        {5}{5}┏{0}{0}{0}{0}{0}{0}┓{1}{1}\n\
        {5}{5}{5}      {1}{1}{1}\n\
        {5}{5}{5}      {1}{1}{1}\n\
        {5}{5}{5}      {1}{1}{1}\n\
        {5}{5}┗{6}{6}{6}{6}{6}{6}┛{1}{1}\n\
        ┣{6}{6}{6}{6}{6}{6}{6}{6}{6}{6}┫\n\
        {4}{4}{4}      {2}{2}{2}\n\
        {4}{4}{4}      {2}{2}{2}\n\
        {4}{4}{4}      {2}{2}{2}\n\
        {4}{4}┗{3}{3}{3}{3}{3}{3}┛{2}{2}\n\
        ╰{3}{3}{3}{3}{3}{3}{3}{3}{3}{3}╯\n\
        \
        ",
            Self::segment_to_character(0, segments[0], color),
            Self::segment_to_character(1, segments[1], color),
            Self::segment_to_character(2, segments[2], color),
            Self::segment_to_character(3, segments[3], color),
            Self::segment_to_character(4, segments[4], color),
            Self::segment_to_character(5, segments[5], color),
            Self::segment_to_character(6, segments[6], color),
        )
    }
}

fn count_unique_digits(input: Vec<&str>) -> usize {
    input
        .iter()
        .filter(|string| Display::KNOWN_SET.contains(&(string.len())))
        .count()
}

fn read_input_digits(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .trim()
        .split('\n')
        .map(|entry| -> Vec<&str> { entry.split('|').collect() })
        .map(|parts| {
            (
                parts[0].trim().split(' ').collect(),
                parts[1].trim().split(' ').collect(),
            )
        })
        .collect()
}

fn main() {
    let input = read_input!();
    let entries = read_input_digits(input.as_str());
    println!("Entries: {}", entries.len());
    let count = entries
        .clone()
        .into_iter()
        .map(|(_, output)| count_unique_digits(output))
        .sum::<usize>();
    println!("Count: {}", count);
    let sum: usize = entries
        .into_iter()
        .map(|(notes, output)| Display::from_notes(notes).decode_number(output))
        .inspect(|number| println!("{}\n", Display::display(*number, Color::Red)))
        .sum();
    println!("Sum: {}", Display::display(sum, Color::BrightGreen));
}
// wrong: 1103780

#[cfg(test)]
mod tests {
    use crate::{count_unique_digits, read_input_digits, Display};

    const TEST_SET: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_digit_count() {
        let entries = read_input_digits(TEST_SET);
        let unique_digit_count = entries
            .into_iter()
            .map(|(_, output)| count_unique_digits(output))
            .sum::<usize>();
        assert_eq!(26, unique_digit_count);
    }

    #[test]
    fn test_digit_deduction() {
        let entries = read_input_digits(TEST_SET);
        let sum: usize = entries
            .into_iter()
            .map(|(notes, output)| Display::from_notes(notes).decode_number(output))
            .sum();
        assert_eq!(61229, sum);
    }
}
