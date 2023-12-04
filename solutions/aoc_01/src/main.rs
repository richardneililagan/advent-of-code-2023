fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument.");
    }

    match args[1].as_str() {
        "a" => {
            let lines = input_parser::parse_input("inputs/01a.txt");
            let result = solve_a(lines);

            println!("{}", result);
        }
        "b" => {
            let lines = input_parser::parse_input("inputs/01b.txt");
            let result = solve_b(lines);

            println!("{}", result);
        }
        _ => panic!("Invalid argument."),
    };
}

fn solve_a(lines: Vec<String>) -> u32 {
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

fn solve_b(lines: Vec<String>) -> u32 {
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let calibrations = lines.iter().map(|line| {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;

        'forward: for i in 0..line.len() {
            let substring = &line[i..];
            let first_char = substring
                .chars()
                .next()
                .expect("Error retrieving first character.");

            if first_char.is_numeric() {
                first_digit = Some(first_char.to_digit(10).unwrap());
                break;
            }

            for (j, digit) in digits.iter().enumerate() {
                if substring.starts_with(digit) {
                    first_digit = Some(j as u32);
                    break 'forward;
                }
            }
        }

        'backward: for i in 0..line.len() {
            let substring = &line[..(line.len() - i)];
            let last_char = substring
                .chars()
                .last()
                .expect("Error retrieving first character.");

            if last_char.is_numeric() {
                last_digit = Some(last_char.to_digit(10).unwrap());
                break;
            }

            for (j, digit) in digits.iter().enumerate() {
                if substring.ends_with(digit) {
                    last_digit = Some(j as u32);
                    break 'backward;
                }
            }
        }

        first_digit.unwrap() * 10 + last_digit.unwrap()
    });

    calibrations.sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn it_solves_examples_a() {
        let input_text = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};

        assert_eq!(solve_a(input_parser::parse(input_text)), 142);
    }

    #[test]
    fn it_solves_examples_b() {
        let input_text = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};

        assert_eq!(solve_b(input_parser::parse(input_text)), 281);
    }
}
