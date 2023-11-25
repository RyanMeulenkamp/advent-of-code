use advent_of_code::read_input;
use array_tool::vec::Join;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Default)]
struct CCleaner {
    stack: Vec<String>,
    sizes: HashMap<String, usize>,
}

impl CCleaner {
    const MAX_USED: usize = 70_000_000 - 30_000_000;

    fn pwd(&self) -> String {
        self.stack.join("/")
    }

    fn up(&mut self) {
        self.stack.pop();
    }

    fn du(&self, dir_name: String) -> usize {
        *self.sizes.get(&dir_name).unwrap()
    }

    fn cd(&mut self, dir_name: String) {
        self.stack.push(dir_name.into());
    }

    fn dir(&mut self, dir_name: String) -> &mut usize {
        self.sizes.entry(dir_name).or_insert(0)
    }

    fn cummulate(&mut self) {
        let dir_size = self.du(self.pwd());
        self.up();
        *self.dir(self.pwd()) += dir_size;
    }

    pub(crate) fn part_one(input: &str) -> usize {
        let cleaner: Self = input.parse().unwrap();
        cleaner
            .sizes
            .values()
            .filter(|value| **value <= 100_000)
            .sum()
    }

    pub(crate) fn part_two(input: &str) -> usize {
        let cleaner: Self = input.parse().unwrap();
        let to_be_freed = cleaner.du("/".into()) - Self::MAX_USED;
        *cleaner
            .sizes
            .values()
            .filter(|size| **size >= to_be_freed)
            .min()
            .unwrap()
    }
}

impl FromStr for CCleaner {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut cleaner: CCleaner = Default::default();
        for line in input.lines() {
            if line == "$ cd .." {
                cleaner.cummulate()
            } else if line.starts_with("$ cd ") {
                cleaner.cd(line.replace("$ cd ", ""));
            } else if line != "$ ls" && !line.starts_with("dir ") {
                let file_size = line.split(" ").next().unwrap().parse::<usize>().unwrap();
                *cleaner.dir(cleaner.pwd()) += file_size;
            }
        }
        for _ in 0..cleaner.stack.len() {
            cleaner.cummulate()
        }
        Ok(cleaner)
    }
}
fn main() {
    println!("Total candidates: {}", CCleaner::part_one(&read_input!()));
    println!("Best candidate: {}", CCleaner::part_two(&read_input!()));
}

#[cfg(test)]
mod tests {
    use crate::CCleaner;
    use indoc::indoc;

    const TEST_SET: &str = indoc! {
        "
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        "
    };

    #[test]
    fn no_space_left_on_device() {
        assert_eq!(95437, CCleaner::part_one(TEST_SET));
        assert_eq!(24933642, CCleaner::part_two(TEST_SET));
    }
}
