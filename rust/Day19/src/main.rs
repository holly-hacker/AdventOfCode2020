use challenge::Input;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| Input::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| input.solve_1());
    let (solution_2, time_solving_2) = time(|| input.solve_2());

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2", time_solving_2);

    assert!(solution_2 > 226);
}

mod challenge {
    #[derive(Clone)]
    pub struct Input<'a> {
        rules: Vec<Rule>,
        messages: Vec<&'a str>,
    }

    #[derive(Debug, Clone)]
    pub enum Rule {
        Constant(u8),
        And(Vec<usize>),
        Or(Vec<usize>, Vec<usize>),
    }

    impl<'a> Input<'a> {
        pub fn parse(input: &'a str) -> Self {
            let mut split = input.split("\n\n");
            let part1 = split.next().unwrap();
            let part2 = split.next().unwrap();
            debug_assert_eq!(None, split.next());

            let mut rules = vec![];
            for (idx, rule) in part1.split('\n').map(Rule::parse) {
                if idx >= rules.len() {
                    rules.resize(idx + 1, Rule::Constant(0));
                }
                rules[idx] = rule;
            }
            let messages = part2.split('\n').collect();

            Self { rules, messages }
        }

        pub fn solve_1(&self) -> usize {
            self.messages
                .iter()
                .filter(|&&m| self.rules[0].check(m, &self.rules))
                .count()
        }

        pub fn solve_2(&self) -> usize {
            let mut copy = self.clone();
            copy.rules[8] = Rule::Or(vec![42], vec![42, 8]);
            copy.rules[11] = Rule::Or(vec![42, 31], vec![42, 11, 31]);
            copy.solve_1()
        }
    }

    impl Rule {
        fn parse(line: &str) -> (usize, Self) {
            let mut split = line.split(": ");
            let idx = split.next().unwrap();
            let operand = split.next().unwrap();

            let idx = idx.parse().unwrap();

            let operand = if operand.bytes().nth(0).unwrap() == b'"' {
                Self::Constant(operand.bytes().nth(1).unwrap())
            } else if operand.contains('|') {
                let mut split = operand.split(" | ");
                let part1 = split.next().unwrap();
                let part2 = split.next().unwrap();

                Self::Or(
                    part1.split(' ').map(|x| x.parse().unwrap()).collect(),
                    part2.split(' ').map(|x| x.parse().unwrap()).collect(),
                )
            } else {
                Self::And(operand.split(' ').map(|x| x.parse().unwrap()).collect())
            };

            (idx, operand)
        }

        fn check(&self, input: &str, all_rules: &[Rule]) -> bool {
            let checked = self.check_inner(input, all_rules);
            checked.iter().any(|&x| x.is_empty())
        }

        fn check_inner<'a>(&self, input: &'a str, all_rules: &[Rule]) -> Vec<&'a str> {
            match self {
                Rule::Constant(c) => {
                    if input.bytes().nth(0) == Some(*c) {
                        vec![&input[1..]]
                    } else {
                        vec![]
                    }
                }
                Rule::And(rules) => Self::check_all_rules_match(&rules, input, all_rules).into(),
                Rule::Or(rules1, rules2) => {
                    let paths1 = Self::check_all_rules_match(&rules1, input, all_rules);
                    let paths2 = Self::check_all_rules_match(&rules2, input, all_rules);

                    let mut x = paths1.clone();
                    x.extend(paths2);
                    x
                }
            }
        }

        fn check_all_rules_match<'a>(
            rules: &[usize],
            input: &'a str,
            all_rules: &[Rule],
        ) -> Vec<&'a str> {
            if rules.is_empty() {
                return vec![input];
            }

            let rule = *rules.first().unwrap();
            let rule = &all_rules[rule];
            let rule_paths = rule.check_inner(input, all_rules);

            if rule_paths.is_empty() {
                return vec![];
            }

            // check next part
            let remaining_rules = &rules[1..];
            rule_paths
                .iter()
                .flat_map(|&path| Self::check_all_rules_match(remaining_rules, path, all_rules))
                .collect()
        }
    }

    impl Default for Rule {
        fn default() -> Self {
            Rule::Constant(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::Input;

    const TEST_INPUT: &str = "\
        0: 4 1 5\n\
        1: 2 3 | 3 2\n\
        2: 4 4 | 5 5\n\
        3: 4 5 | 5 4\n\
        4: \"a\"\n\
        5: \"b\"\n\
        \n\
        ababbb\n\
        bababa\n\
        abbbab\n\
        aaabbb\n\
        aaaabbb";

    const TEST_INPUT_2: &str = include_str!("../test_input.txt");

    #[test]
    fn test_1() {
        let parsed = Input::parse(TEST_INPUT);
        assert_eq!(2, parsed.solve_1());
    }

    #[test]
    fn test_2_1() {
        let parsed = Input::parse(TEST_INPUT_2);
        assert_eq!(3, parsed.solve_1());
    }

    #[test]
    fn test_2_2() {
        let parsed = Input::parse(TEST_INPUT_2);
        assert_eq!(12, parsed.solve_2());
    }
}
