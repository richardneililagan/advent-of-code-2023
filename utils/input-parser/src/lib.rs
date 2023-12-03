use std::fs;
use std::path;

pub fn parse_input(absolute_filepath: path::PathBuf) -> Vec<String> {
    fs::read_to_string(absolute_filepath)
        .expect("Something went wrong reading the file")
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let current_dir = std::env::current_dir().expect("Failed to get current path");
        let absolute_path = current_dir.join("test/data.txt");
        let canonical_path = absolute_path
            .canonicalize()
            .expect("Failed to get canonical path");

        let result = parse_input(canonical_path);
        assert_eq!(result, vec!["a", "bc", "def", "gh", "ij"]);
    }
}
