fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument.");
    }

    match args[1].as_str() {
        "a" => {
            let lines = input_parser::parse_input("inputs/04a.txt");
            let result = solve_a(lines);

            println!("{}", result);
        }
        "b" => {
            let lines = input_parser::parse_input("inputs/04b.txt");
            let result = solve_b(lines);

            println!("{}", result);
        }
        _ => panic!("Invalid argument."),
    };
}

fn sanitize_line(line: &str) -> String {
    let Some(index) = line.find(':') else {
        unreachable!("Something went wrong.");
    };

    line[(index + 1)..].to_string()
}

fn solve_a(lines: Vec<String>) -> u32 {
    let mut score = 0;

    for card in lines.iter().map(|line| sanitize_line(line)) {
        let c = card
            .split('|')
            .map(|numbers| {
                numbers
                    .split_whitespace()
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let [winning_numbers, numbers] = &c[..] else {
            unreachable!("Something went wrong.")
        };

        let win_count = numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count() as u32;

        if win_count > 0 {
            score += 2_u32.pow(win_count - 1);
        }
    }

    score
}

fn solve_b(lines: Vec<String>) -> u32 {
    let mut card_count = vec![1; lines.len()];
    for (i, card) in lines.iter().map(|l| sanitize_line(l)).enumerate() {
        let c = card
            .split('|')
            .map(|numbers| {
                numbers
                    .split_whitespace()
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let [winning_numbers, numbers] = &c[..] else {
            unreachable!("Something went wrong.")
        };

        let win_count = numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();

        let win_copies = card_count[i];
        ((i + 1)..(i + 1 + win_count)).for_each(|i| card_count[i] += win_copies);
    }

    card_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn it_solves_examples_a() {
        let input_text = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        assert_eq!(solve_a(input_parser::parse(input_text)), 13);
    }

    #[test]
    fn it_solves_examples_b() {
        let input_text = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        assert_eq!(solve_b(input_parser::parse(input_text)), 30);
    }
}
