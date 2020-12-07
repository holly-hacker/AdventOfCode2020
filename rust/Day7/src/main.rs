mod string_interner;

use string_interner::{StringInterner, StringKey};
use arr_macro::arr;

include!("../../helpers.rs");

// TODO: store in tree/graph form

const LOOKUP_SIZE: usize = 1024;
pub struct InputData {
    map: [Vec<ContainedLuggage>; LOOKUP_SIZE],
    cache: StringInterner,
}

impl Default for InputData {
    fn default() -> Self {
        Self {
            cache: StringInterner::default(),
            map: arr![vec![]; 1024], // manually entered
        }
    }
}

impl InputData {
    pub fn look_up_container(&self, data: StringKey) -> &Vec<ContainedLuggage> {
        &self.map[data.as_usize()]
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ContainedLuggage {
    color: StringKey,
    count: u16,
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

#[derive(Copy, Clone)]
struct LookupEntry(u8);

const TRUE: u8 = 1;
const FALSE: u8 = 0;
const UNKNOWN: u8 = 2;
impl LookupEntry {
    pub fn is_known(&self) -> bool {
        self.0 != UNKNOWN
    }
    pub fn is_true(&self) -> bool {
        self.0 == TRUE
    }

    pub fn new_unknown() -> Self {
        LookupEntry(UNKNOWN)
    }
    pub fn from_bool(b: bool) -> Self {
        LookupEntry(if b { TRUE } else { FALSE })
    }
}

fn solve_1(data: &InputData) -> usize {
    const TARGET_COLOR: &str = "shiny gold";
    let target_color = data.cache.get_key(TARGET_COLOR);

    let mut sum = 0;
    let mut buffer = [LookupEntry::new_unknown(); LOOKUP_SIZE];
    for k in (0..data.map.len()).map(|i| StringKey::from(i as u16)) {
        // TODO: would create enumerator method
        // checking against target_color again, since we don't want to return if we are target_color
        let b = k != target_color && can_hold_color(k, target_color, data, &mut buffer);
        sum += b as usize;
    }

    sum
}

fn solve_2(data: &InputData) -> u16 {
    const TARGET_COLOR: &str = "shiny gold";
    let target_color = data.cache.get_key(TARGET_COLOR);

    check_required_bags(target_color, data)
}

fn can_hold_color(
    color: StringKey,
    target: StringKey,
    data_set: &InputData,
    cache: &mut [LookupEntry; LOOKUP_SIZE],
) -> bool {
    if color == target {
        cache[color.as_usize()] = LookupEntry::from_bool(true);
        return true;
    }

    let lookup = &cache[color.as_usize()];
    if lookup.is_known() {
        return lookup.is_true();
    }

    let contained = data_set.look_up_container(color);
    let found = contained
        .iter()
        .any(|c| can_hold_color(c.color, target, data_set, cache));

    cache[color.as_usize()] = LookupEntry::from_bool(found);
    found
}

fn check_required_bags(color: StringKey, data_set: &InputData) -> u16 {
    let contained = data_set.look_up_container(color);
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

        let key = data.cache.intern(color);
        // assert_eq!(data.map.len(), key.as_usize());

        data.map[key.as_usize()] = if things == "no other bags." {
            vec![]
        } else {
            let mut v = vec![];
            for s in things.trim_end_matches('.').split(", ") {
                v.push(parse_contained(s, &mut data.cache));
            }
            v
        };
    }

    data
}

fn parse_contained(s: &str, cache: &mut StringInterner) -> ContainedLuggage {
    let count_unparsed = s.split(' ').next().unwrap();
    let count_len = count_unparsed.len();
    let count = count_unparsed.parse::<usize>().unwrap() as u16;

    let color_len = s.len() - (count_len + "bag".len() + 2 + if count == 1 { 0 } else { 1 });
    let color = cache.intern(&s[count_len + 1..count_len + 1 + color_len]);

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
            parsed.map[parsed.cache.get_key("light red").as_usize()]
        );

        assert_eq!(
            Vec::<ContainedLuggage>::new(),
            parsed.map[parsed.cache.get_key("dotted black").as_usize()],
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
