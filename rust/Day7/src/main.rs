use std::{collections::HashMap, todo};

include!("../../helpers.rs");

// TODO: could intern strings for perf
// TODO: store in tree/graph form

type InputData = HashMap<String, Vec<ContainedLuggage>>;

#[derive(Debug, PartialEq, Eq)]
struct ContainedLuggage {
    color: String,
    count: usize,
}

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| parse_input(&stdin));
    let (solution_1, time_solving_1) = time(|| solve_1(input));

    println!("solution 1: {:?}", solution_1);
    println!("took {:?} to read input", time_reading);
    println!("took {:?} to parse input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
}

fn solve_1(data: InputData) -> usize {
    const TARGET_COLOR: &str = "shiny gold";
    
    data.keys().map(|k| (k != TARGET_COLOR && can_hold_color(k, TARGET_COLOR, &data)) as usize).sum()
}

fn can_hold_color(color: &str, target: &str, data_set: &InputData) -> bool {
    if color == target {
        return true;
    }

    let contained = &data_set[color];
    contained.iter().any(|c| can_hold_color(&c.color, target, data_set))
}

fn parse_input(input: &str) -> InputData {
    let mut v = HashMap::new();

    for line in input.split('\n') {
        let mut split = line.split(" bags contain ");
        let color = split.next().unwrap();
        let things = split.next().unwrap();

        v.insert(
            color.into(),
            if things == "no other bags." {
                vec![]
            } else {
                things
                    .trim_end_matches('.')
                    .split(", ")
                    .map(parse_contained)
                    .collect()
            },
        );
    }

    v
}

fn parse_contained(s: &str) -> ContainedLuggage {
    let count_unparsed = s.split(' ').next().unwrap();
    let count_len = count_unparsed.len();
    let count = count_unparsed.parse::<usize>().unwrap();

    let color_len = s.len() - (count_len + "bag".len() + 2 + if count == 1 { 0 } else { 1 });
    let color = (&s[count_len + 1..count_len + 1 + color_len]).into();

    ContainedLuggage { count, color }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_DATA: &str = "\
    light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
    bright white bags contain 1 shiny gold bag.\n\
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
    faded blue bags contain no other bags.\n\
    dotted black bags contain no other bags.";

    #[test]
    fn test_parsing() {
        let parsed = parse_input(TEST_DATA);

        assert_eq!(
            vec![
                ContainedLuggage {
                    color: "bright white".into(),
                    count: 1,
                },
                ContainedLuggage {
                    color: "muted yellow".into(),
                    count: 2,
                },
            ],
            parsed["light red"]
        );

        assert_eq!(
            Vec::<ContainedLuggage>::new(),
            parsed["dotted black"],
        )
    }

    #[test]
    fn test_solution_1() {
        let parsed = parse_input(TEST_DATA);
        assert_eq!(4, solve_1(parsed));
    }
}
