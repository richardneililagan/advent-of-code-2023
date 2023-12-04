use std::fs;
use std::path;

pub fn parse_input(filepath: &str) -> Vec<String> {
    fs::read_to_string(get_input_filepath(filepath))
        .expect("Something went wrong reading the file")
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>()
}

fn get_input_filepath(filepath: &str) -> path::PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current path");
    println!("{}", current_dir.display());
    current_dir
        .join(filepath)
        .canonicalize()
        .expect("Failed to get canonical path")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = parse_input("test/data.txt");
        assert_eq!(result, vec!["a", "bc", "def", "gh", "ij"]);
    }
}
