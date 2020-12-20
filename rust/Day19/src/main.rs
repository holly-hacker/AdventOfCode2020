use challenge::Input;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| Input::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| input.solve_1());
    // let (solution_2, time_solving_2) = time(|| solve(&input, 30000000));

    println!("solution 1: {}", solution_1);
    // println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    // println!("took {:?} to solve 2", time_solving_2);
}

mod challenge {
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
            checked.0 && checked.1.len() == 0
        }

        fn check_inner<'a>(&self, input: &'a str, all_rules: &[Rule]) -> (bool, &'a str) {
            match self {
                Rule::Constant(c) => (input.bytes().nth(0) == Some(*c), &input[1..]),
                Rule::And(list) => Self::check_multiple(&list, input, all_rules),
                Rule::Or(list1, list2) => {
                    let (b1, s1) = Self::check_multiple(&list1, input, all_rules);
                    let (b2, s2) = Self::check_multiple(&list2, input, all_rules);

                    // not supporting different length patterns
                    if b1 && b2 {
                        debug_assert_eq!(s1.len(), s2.len());
                        debug_assert_eq!(s1, s2);
                    }

                    if b1 {
                        (true, s1)
                    } else if b2 {
                        (true, s2)
                    } else {
                        (false, "")
                    }
                }
            }
        }

        fn check_multiple<'a>(
            rules: &[usize],
            mut input: &'a str,
            all_rules: &[Rule],
        ) -> (bool, &'a str) {
            for &idx in rules {
                let (b, input2) = all_rules[idx].check_inner(input, all_rules);
                input = input2;
                if !b {
                    return (false, "");
                }
            }
            (true, input)
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

    #[test]
    fn test_1() {
        let parsed = Input::parse(TEST_INPUT);
        assert_eq!(2, parsed.solve_1());
    }
}
