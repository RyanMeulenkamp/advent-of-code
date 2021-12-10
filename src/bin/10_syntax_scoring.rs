use std::collections::HashMap;
use advent_of_code::read_input;

struct Stack {
    data: Vec<char>,
}

impl Stack {

    const CLOSING_CHARS: [char; 4] = [')', ']', '}', '>'];
    const OPENING_CHARS: [char; 4] = ['(', '[', '{', '<'];

    fn new(data: Vec<char>) -> Self {
        Self {
            data
        }
    }

    fn complete(input: &str) -> (String, usize) {
        let mut stack = Stack::new(Vec::new());
        for character in input.chars() {
            stack.process_char(character);
        }
        stack.data.into_iter()
            .rev()
            .flat_map(Self::opener_to_closer)
            .fold((String::new(), 0), |(string, score), character|
                (format!("{}{}", string, character), score * 5 + Self::char_to_completion_score(character))
            )
    }

    fn is_corrupt(input: &str) -> bool {
        let mut stack = Stack::new(Vec::new());
        for character in input.chars() {
            if !stack.process_char(character) {
                return true;
            }
        }
        false
    }

    fn calculate_corruption_score(input: &str) -> usize {
        let mut stack = Stack::new(Vec::new());
        let mut counts = HashMap::new();
        for character in input.chars() {
            if !stack.process_char(character) {
                *counts.entry(character).or_insert(0) += 1;
            }
        }
        counts.into_iter().map(|(key, value)| Self::char_to_corruption_score(key) * value).sum()
    }

    fn process_char(&mut self, c: char) -> bool {
        if Self::OPENING_CHARS.contains(&c) {
            self.data.push(c);
            true
        } else if Self::CLOSING_CHARS.contains(&c) {
            if let Some(opener) = self.data.pop() {
                matches!(Self::opener_to_closer(opener), Some(closer) if closer == c)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn opener_to_closer(c: char) -> Option<char> {
        match c {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '<' => Some('>'),
            _ => None,
        }
    }

    fn char_to_corruption_score(c: char) -> usize {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    fn char_to_completion_score(c: char) -> usize {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    }
}

fn completion_score(input: &str) -> usize {
    let mut score: Vec<usize> = input.lines()
        .filter(|line| !Stack::is_corrupt(line))
        .map(|line| (line, Stack::complete(line)))
        .inspect(|(line, (completion, score))| println!("{} -> {} (score: {})", line, completion, score))
        .map(|(_, (_, score))| score)
        .collect();
    score.sort_unstable();
    score[score.len() / 2]
}

fn main() {
    let score: usize = read_input!()
        .lines()
        .into_iter()
        .map(Stack::calculate_corruption_score)
        .inspect(|score| println!("Score: {}", score))
        .sum();

    println!("Syntax score: {}", score);

    println!("Completion score: {}", completion_score(read_input!().as_str()));
}

#[cfg(test)]
mod tests {
    use crate::Stack;

    const TEST_SET: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_low_points() {
        let score: usize = TEST_SET.lines()
            .map(|line| Stack::calculate_corruption_score(line))
            .inspect(|score| println!("Score: {}", score))
            .sum();
        assert_eq!(26397, score);
    }

    #[test]
    fn test_completion() {
        let mut score: Vec<usize> = TEST_SET.lines()
            .filter(|line| !Stack::is_corrupt(line))
            .map(|line| (line, Stack::complete(line)))
            .inspect(|(line, (completion, score))| println!("{} -> {} (score: {})", line, completion, score))
            .map(|(_, (_, score))| score)
            .collect();
        score.sort();
        assert_eq!(288957, score[score.len() / 2])
    }
}
