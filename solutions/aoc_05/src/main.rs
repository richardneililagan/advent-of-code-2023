use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument.");
    }

    match args[1].as_str() {
        "a" => {
            let lines = input_parser::parse_input("inputs/05a.txt");
            let result = solve_a(lines);

            println!("{}", result);
        }
        "b" => {
            let lines = input_parser::parse_input("inputs/05b.txt");
            let result = solve_b(lines);

            println!("{}", result);
        }
        _ => panic!("Invalid argument."),
    };
}

fn solve_a(lines: Vec<String>) -> usize {
    let seeds = lines[0][6..]
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_key: Option<String> = None;

    for line in &lines[1..] {
        match line {
            // Can't do anything if it's an empty line.
            line if line.is_empty() => continue,

            // If it starts with an alphabetic character, then this is a new mappings group marker.
            line if line.chars().next().unwrap().is_alphabetic() => {
                current_key = Some(line.split_whitespace().next().unwrap().to_string());
                let key = current_key
                    .as_ref()
                    .expect("Mapping key is not set when it should already have been.")
                    .to_string();

                maps.insert(key, Vec::new());
            }

            // Otherwise, this is an entry in the current mappings group.
            _ => {
                let key = current_key
                    .as_ref()
                    .expect("Mapping key is not set when it should already have been.")
                    .to_string();

                let v = maps
                    .get_mut(&key)
                    .expect("Mapping vector did not exist when it should already have.");

                v.push(line.to_string());
            }
        }
    }

    let to_soil = create_translator(maps.get("seed-to-soil").expect("Not found").to_vec());
    let to_fert = create_translator(maps.get("soil-to-fertilizer").expect("Not found").to_vec());
    let to_water = create_translator(maps.get("fertilizer-to-water").expect("Not found").to_vec());
    let to_light = create_translator(maps.get("water-to-light").expect("Not found").to_vec());
    let to_temp = create_translator(
        maps.get("light-to-temperature")
            .expect("Not found")
            .to_vec(),
    );
    let to_humi = create_translator(
        maps.get("temperature-to-humidity")
            .expect("Not found")
            .to_vec(),
    );
    let to_loc = create_translator(
        maps.get("humidity-to-location")
            .expect("Not found")
            .to_vec(),
    );

    seeds
        .iter()
        .map(|s| to_loc(to_humi(to_temp(to_light(to_water(to_fert(to_soil(*s))))))))
        .min()
        .unwrap()
}

fn solve_b(_lines: Vec<String>) -> usize {
    0
}

/// Create a mapper function derived from a section of the almanac.
fn create_translator(maps: Vec<String>) -> impl Fn(usize) -> usize {
    // (source start, source end, dest offset)
    let mut mapping: Vec<(isize, isize, isize)> = Vec::new();

    for map in maps {
        let m = map
            .split_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let [dest_start, source_start, length] = &m[..] else {
            unreachable!("Something went wrong.")
        };

        mapping.push((
            *source_start,
            *source_start + *length,
            *dest_start - *source_start,
        ));
    }

    move |value: usize| -> usize {
        let v = value as isize;
        match mapping
            .iter()
            .find(|(start, end, _)| v >= *start && v <= *end)
        {
            Some((_, _, offset)) => (v + offset) as usize,
            None => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn it_correctly_creates_a_translator() {
        let input_text = indoc! {"
            50 98 2
            52 50 48
        "};

        let maps = input_parser::parse(input_text);
        let translator = create_translator(maps);

        assert_eq!(translator(0), 0);
        assert_eq!(translator(50), 52);
        assert_eq!(translator(51), 53);
        assert_eq!(translator(96), 98);
        assert_eq!(translator(98), 50);
        assert_eq!(translator(99), 51);
        assert_eq!(translator(200), 200);
    }

    #[test]
    fn it_solves_examples_a() {
        let input_text = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};

        assert_eq!(solve_a(input_parser::parse(input_text)), 35);
    }
}
