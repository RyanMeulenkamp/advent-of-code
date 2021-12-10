#[macro_export]
macro_rules! read_input {
    () => {{
        let filename = file!();
        let filename = filename
            .strip_suffix(".rs")
            .unwrap()
            .strip_prefix("src/bin/")
            .unwrap();
        let path = std::env::current_dir()
            .unwrap()
            .join(format!("res/{}.txt", filename));
        std::fs::read_to_string(path).expect("File does not exist!")
    }};
}

#[macro_export]
macro_rules! read_input_lines {
    () => {{
        advent_of_code::read_input!()
            .split('\n')
            .filter(|string| !string.is_empty())
            .collect::<Vec<&str>>()
    }};
}

#[macro_export]
macro_rules! read_input_ints {
    () => {{
        advent_of_code::read_input_lines!()
            .iter()
            .map(|string| string.parse::<u32>().expect("Not all lines are integers"))
            .collect()
    }};
}
