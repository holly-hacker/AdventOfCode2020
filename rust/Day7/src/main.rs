mod string_interner;

use std::collections::HashMap;
use string_interner::StringInterner;

include!("../../helpers.rs");

// TODO: store in tree/graph form

type StringKey = usize;

#[derive(Default)]
pub struct InputData {
    pub map: HashMap<StringKey, Vec<ContainedLuggage>>,
    pub cache: StringInterner,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ContainedLuggage {
    color: StringKey,
    count: usize,
}

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| parse_input(&stdin));
    let (solution_1, time_solving_1) = time(|| solve_1(&input));
    let (solution_2, time_solving_2) = time(|| solve_2(&input));

    println!("solution 1: {:?}", solution_1);
    println!("solution 2: {:?}", solution_2);
    println!("took {:?} to read input", time_reading);
    println!("took {:?} to parse input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2", time_solving_2);
}

fn solve_1(data: &InputData) -> usize {
    const TARGET_COLOR: &str = "shiny gold";
    let target_color = data.cache.get_key(TARGET_COLOR);

    data.map
        .keys()
        .map(|k| (*k != target_color && can_hold_color(*k, target_color, data)) as usize)
        .sum()
}

fn solve_2(data: &InputData) -> usize {
    const TARGET_COLOR: &str = "shiny gold";
    let target_color = data.cache.get_key(TARGET_COLOR);

    check_required_bags(target_color, data)
}

fn can_hold_color(color: StringKey, target: StringKey, data_set: &InputData) -> bool {
    if color == target {
        return true;
    }

    let contained = &data_set.map[&color];
    contained
        .iter()
        .any(|c| can_hold_color(c.color, target, data_set))
}

fn check_required_bags(color: StringKey, data_set: &InputData) -> usize {
    let contained = &data_set.map[&color];
    contained
        .iter()
        .map(|c| (check_required_bags(c.color, data_set) + 1) * c.count)
        .sum()
}

fn parse_input(input: &str) -> InputData {
    let mut data = InputData::default();

    for line in input.split('\n') {
        let mut split = line.split(" bags contain ");
        let color = split.next().unwrap();
        let things = split.next().unwrap();

        data.map.insert(
            data.cache.get_key_or_insert(color),
            if things == "no other bags." {
                vec![]
            } else {
                let mut v = vec![];
                for s in things.trim_end_matches('.').split(", ") {
                    v.push(parse_contained(s, &mut data.cache));
                }
                v
            },
        );
    }

    data
}

fn parse_contained(s: &str, cache: &mut StringInterner) -> ContainedLuggage {
    let count_unparsed = s.split(' ').next().unwrap();
    let count_len = count_unparsed.len();
    let count = count_unparsed.parse::<usize>().unwrap();

    let color_len = s.len() - (count_len + "bag".len() + 2 + if count == 1 { 0 } else { 1 });
    let color = cache.get_key_or_insert(&s[count_len + 1..count_len + 1 + color_len]);

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

    const TEST_DATA_2: &str = "\
        shiny gold bags contain 2 dark red bags.\n\
        dark red bags contain 2 dark orange bags.\n\
        dark orange bags contain 2 dark yellow bags.\n\
        dark yellow bags contain 2 dark green bags.\n\
        dark green bags contain 2 dark blue bags.\n\
        dark blue bags contain 2 dark violet bags.\n\
        dark violet bags contain no other bags.";

    #[test]
    fn test_parsing() {
        let parsed = parse_input(TEST_DATA);

        assert_eq!(
            vec![
                ContainedLuggage {
                    color: parsed.cache.get_key("bright white"),
                    count: 1,
                },
                ContainedLuggage {
                    color: parsed.cache.get_key("muted yellow"),
                    count: 2,
                },
            ],
            parsed.map[&parsed.cache.get_key("light red")]
        );

        assert_eq!(
            Vec::<ContainedLuggage>::new(),
            parsed.map[&parsed.cache.get_key("dotted black")],
        )
    }

    #[test]
    fn test_solution_1() {
        let parsed = parse_input(TEST_DATA);
        assert_eq!(4, solve_1(&parsed));
    }

    #[test]
    fn test_solution_2() {
        let parsed = parse_input(TEST_DATA);
        assert_eq!(32, solve_2(&parsed));
    }

    #[test]
    fn test_solution_2_other() {
        let parsed = parse_input(TEST_DATA_2);
        assert_eq!(126, solve_2(&parsed));
    }
}
