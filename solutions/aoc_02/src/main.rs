use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument.");
    }

    match args[1].as_str() {
        "a" => {
            let lines = input_parser::parse_input("inputs/02a.txt");
            let result = solve_a(lines);

            println!("{}", result);
        }
        "b" => {
            let lines = input_parser::parse_input("inputs/02b.txt");
            let result = solve_b(lines);

            println!("{}", result);
        }
        _ => panic!("Invalid argument."),
    };
}

fn solve_a(lines: Vec<String>) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let re_game = Regex::new(r"^Game (?<game_number>\d*)\:").unwrap();
    let re_red = Regex::new(r"(?<count>\d*)\sred").unwrap();
    let re_green = Regex::new(r"(?<count>\d*)\sgreen").unwrap();
    let re_blue = Regex::new(r"(?<count>\d*)\sblue").unwrap();

    let mut sum = 0;

    for line in lines {
        let game_number = re_game.captures(&line).unwrap()["game_number"]
            .parse::<u32>()
            .unwrap();

        let mut red_draws = re_red.captures_iter(&line).map(|caps| {
            let (_, [count]) = caps.extract();
            count.parse::<u32>().unwrap()
        });

        let mut green_draws = re_green.captures_iter(&line).map(|caps| {
            let (_, [count]) = caps.extract();
            count.parse::<u32>().unwrap()
        });

        let mut blue_draws = re_blue.captures_iter(&line).map(|caps| {
            let (_, [count]) = caps.extract();
            count.parse::<u32>().unwrap()
        });

        if red_draws.any(|count| count > MAX_RED) {
            continue;
        }

        if green_draws.any(|count| count > MAX_GREEN) {
            continue;
        }

        if blue_draws.any(|count| count > MAX_BLUE) {
            continue;
        }

        sum += game_number;
    }

    sum
}
fn solve_b(lines: Vec<String>) -> u32 {
    let re_red = Regex::new(r"(?<count>\d*)\sred").unwrap();
    let re_green = Regex::new(r"(?<count>\d*)\sgreen").unwrap();
    let re_blue = Regex::new(r"(?<count>\d*)\sblue").unwrap();

    lines
        .iter()
        .map(|line| {
            let red_draws = re_red.captures_iter(line).map(|caps| {
                let (_, [count]) = caps.extract();
                count.parse::<u32>().unwrap()
            });

            let green_draws = re_green.captures_iter(line).map(|caps| {
                let (_, [count]) = caps.extract();
                count.parse::<u32>().unwrap()
            });

            let blue_draws = re_blue.captures_iter(line).map(|caps| {
                let (_, [count]) = caps.extract();
                count.parse::<u32>().unwrap()
            });

            let min_red = red_draws.max().unwrap_or(0);
            let min_green = green_draws.max().unwrap_or(0);
            let min_blue = blue_draws.max().unwrap_or(0);

            min_red * min_green * min_blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn it_solves_examples_a() {
        let input_text = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(solve_a(input_parser::parse(input_text)), 8);
    }

    #[test]
    fn it_solves_examples_b() {
        let input_text = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(solve_b(input_parser::parse(input_text)), 2286);
    }
}
