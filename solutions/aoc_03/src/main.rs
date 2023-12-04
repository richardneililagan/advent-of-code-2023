use regex::Regex;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument.");
    }

    match args[1].as_str() {
        "a" => {
            let lines = input_parser::parse_input("inputs/03a.txt");
            let result = solve_a(lines);

            println!("{}", result);
        }
        "b" => {
            let lines = input_parser::parse_input("inputs/03b.txt");
            let result = solve_b(lines);

            println!("{}", result);
        }
        _ => panic!("Invalid argument."),
    };
}

fn solve_a(lines: Vec<String>) -> u32 {
    let mut contact_cells: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for x in 0..line.len() {
            let c = line.chars().nth(x).unwrap();
            if !c.is_alphanumeric() && c != '.' {
                contact_cells.insert(((x - 1) as i32, (y - 1) as i32));
                contact_cells.insert(((x - 1) as i32, (y) as i32));
                contact_cells.insert(((x - 1) as i32, (y + 1) as i32));
                contact_cells.insert(((x) as i32, (y - 1) as i32));
                contact_cells.insert(((x) as i32, (y) as i32));
                contact_cells.insert(((x) as i32, (y + 1) as i32));
                contact_cells.insert(((x + 1) as i32, (y - 1) as i32));
                contact_cells.insert(((x + 1) as i32, (y) as i32));
                contact_cells.insert(((x + 1) as i32, (y + 1) as i32));
            }
        }
    }

    let mut sum = 0;
    let re_number = Regex::new(r"\d+").unwrap();
    for (y, line) in lines.iter().enumerate() {
        re_number.find_iter(line).for_each(|m| {
            let number = m.as_str().parse::<u32>().unwrap();

            if m.range()
                .map(|x| (x as i32, y as i32))
                .any(|c| contact_cells.contains(&c))
            {
                sum += number;
            }
        });
    }

    sum
}

fn solve_b(lines: Vec<String>) -> u32 {
    unimplemented!("Not implemented yet.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn it_solves_examples_a() {
        let input_text = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};

        assert_eq!(solve_a(input_parser::parse(input_text)), 4361);
    }
}
