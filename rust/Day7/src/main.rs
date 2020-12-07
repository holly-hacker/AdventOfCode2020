use std::todo;

include!("../../helpers.rs");

// TODO: could intern strings for perf
// TODO: store in tree/graph form

#[derive(Debug, PartialEq, Eq)]
struct LuggageRule {
    color: String,
    rules: Vec<ContainedLuggage>,
}

#[derive(Debug, PartialEq, Eq)]
struct ContainedLuggage {
    color: String,
    count: usize,
}

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| parse_input(&stdin));

    println!("took {:?} to read input", time_reading);
    println!("took {:?} to parse input", time_parsing);
}

fn parse_input(input: &str) -> Vec<LuggageRule> {
    let mut v = vec![];

    for line in input.split('\n') {
        let mut split = line.split(" bags contain ");
        let color = split.next().unwrap();
        let things = split.next().unwrap();

        v.push(LuggageRule {
            color: color.into(),
            rules: if things == "no other bags." {
                vec![]
            } else {
                things
                    .trim_end_matches('.')
                    .split(", ")
                    .map(parse_contained)
                    .collect()
            },
        })
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
    use crate::{parse_input, ContainedLuggage, LuggageRule};

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
    fn test() {
        let parsed = parse_input(TEST_DATA);
        let mut iter = parsed.into_iter();

        assert_eq!(
            LuggageRule {
                color: "light red".into(),
                rules: vec![
                    ContainedLuggage {
                        color: "bright white".into(),
                        count: 1,
                    },
                    ContainedLuggage {
                        color: "muted yellow".into(),
                        count: 2,
                    },
                ]
            },
            iter.next().unwrap()
        );

        assert_eq!(
            LuggageRule {
                color: "dotted black".into(),
                rules: vec![],
            },
            iter.last().unwrap()
        )
    }
}
