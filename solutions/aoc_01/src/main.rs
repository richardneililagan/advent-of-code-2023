fn main() {
    let lines = input_parser::parse_input("inputs/01a.txt");
    let result = solve(lines);

    println!("{}", result);
}

fn solve(lines: Vec<String>) -> u32 {
    let calibrations = lines.iter().map(|line| {
        let chars = line.chars();
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;

        for c in chars.clone() {
            if c.is_numeric() {
                first_digit = Some(c.to_digit(10).unwrap());
                break;
            }
        }

        for c in chars.clone().rev() {
            if c.is_numeric() {
                last_digit = Some(c.to_digit(10).unwrap());
                break;
            }
        }

        first_digit.unwrap() * 10 + last_digit.unwrap()
    });

    calibrations.sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_examples() {
        let input = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];

        assert_eq!(solve(input), 142);
    }
}
