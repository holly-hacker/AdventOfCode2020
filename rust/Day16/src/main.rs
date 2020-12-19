use crate::challenge::Input;
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
    debug_assert!(solution_2 > 863343189013);
    debug_assert!(solution_2 > 1952671648819);
    debug_assert_eq!(2766491048287, solution_2);
}

mod challenge {
    use std::convert::TryInto;

    pub struct Input {
        pub ranges: Vec<Range>,
        pub ticket: Ticket,
        pub nearby_tickets: Vec<Ticket>,
    }

    pub struct Range {
        pub name: String,
        pub range1: (u16, u16),
        pub range2: (u16, u16),
    }

    pub struct Ticket(Vec<u16>); // would use array but const generics is not stabilized yet

    impl Input {
        pub fn parse(input: &str) -> Self {
            // multiple stages
            let mut chunks = input.split("\n\n");

            let ranges = chunks
                .next()
                .unwrap()
                .split('\n')
                .map(Range::parse)
                .collect::<Vec<Range>>();

            let ticket = Ticket::parse(chunks.next().unwrap().split('\n').skip(1).next().unwrap());

            let nearby_tickets = chunks
                .next()
                .unwrap()
                .split('\n')
                .skip(1)
                .map(Ticket::parse)
                .collect::<Vec<Ticket>>();

            Self {
                ranges,
                ticket,
                nearby_tickets,
            }
        }

        pub fn solve_1(&self) -> usize {
            self.nearby_tickets
                .iter()
                .flat_map(|x| &x.0)
                .filter(|&&x| !self.is_valid(x))
                .map(|&x| x as usize)
                .sum()
        }

        pub fn solve_2(&self) -> usize {
            let correct_tickets = self
                .nearby_tickets
                .iter()
                .filter(|ticket| ticket.0.iter().all(|&val| self.is_valid(val)))
                //.chain(std::iter::once(&self.ticket))
                .collect::<Vec<&Ticket>>();

            let masks = self.calculate_range_masks(&correct_tickets);
            let map = self.convert_masks_to_map(masks);
            // println!("{:?}", map);

            map.into_iter()
                .enumerate()
                .filter(|(_, i)| self.ranges[*i].name.starts_with("departure"))
                .map(|(i, _)| self.ticket.0[i] as usize)
                .product()
        }

        fn is_valid(&self, ticket_value: u16) -> bool {
            self.ranges.iter().any(|r| r.in_range(ticket_value))
        }

        fn calculate_range_masks(&self, correct_tickets: &Vec<&Ticket>) -> Vec<Vec<bool>> {
            let mut vec = vec![];
            for range in &self.ranges {
                let mut correct = vec![true; self.ranges.len()];
                for &ticket in correct_tickets {
                    for i in 0..correct.len() {
                        correct[i] &= range.in_range(ticket.0[i]);
                    }
                }

                vec.push(correct);
            }
            vec
        }

        fn convert_masks_to_map(&self, masks: Vec<Vec<bool>>) -> Vec<usize> {
            debug_assert!(masks.len() > 0);

            let mut map = vec![None; masks[0].len()];
            loop {
                for (i, mask) in masks.iter().enumerate() {
                    let mut iter = mask
                        .iter()
                        .enumerate()
                        .filter(|(i, &b)| b && map[*i].is_none())
                        .map(|(i, _)| i);
                    let one = iter.next();
                    let two = iter.next();

                    if one.is_some() && two.is_none() {
                        let index = one.unwrap();
                        map[index] = Some(i);
                    }
                }
                if map.iter().all(|x| x.is_some()) {
                    break;
                }
            }

            map.into_iter()
                .map(|x| x.expect("Couldn't solve masks"))
                .collect()
        }
    }

    impl Range {
        pub fn parse(input: &str) -> Self {
            let mut split = input.split(": ");
            let name = split.next().unwrap().into();
            let part2 = split.next().unwrap();

            let mut split2 = part2.split(" or ");
            let range1_str = split2.next().unwrap();
            let range2_str = split2.next().unwrap();

            Self {
                name,
                range1: Self::parse_range(range1_str),
                range2: Self::parse_range(range2_str),
            }
        }

        fn parse_range(input: &str) -> (u16, u16) {
            let mut split = input.split('-');
            let p1 = split.next().unwrap().parse().unwrap();
            let p2 = split.next().unwrap().parse().unwrap();
            (p1, p2)
        }

        fn in_range(&self, value: u16) -> bool {
            let valid1 = value >= self.range1.0 && value <= self.range1.1;
            let valid2 = value >= self.range2.0 && value <= self.range2.1;
            valid1 || valid2
        }
    }

    impl Ticket {
        pub fn parse(input: &str) -> Self {
            let vec = input
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u16>>();
            let buffer = vec.try_into().unwrap();
            Self(buffer)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::Input;

    const TEST_INPUT_1: &str = "\
        class: 1-3 or 5-7\n\
        row: 6-11 or 33-44\n\
        seat: 13-40 or 45-50\n\
        \n\
        your ticket:\n\
        7,1,14\n\
        \n\
        nearby tickets:\n\
        7,3,47\n\
        40,4,50\n\
        55,2,20\n\
        38,6,12";

    const TEST_INPUT_2: &str = "\
        departure-test class: 0-1 or 4-19\n\
        departure-test row: 0-5 or 8-19\n\
        departure-test seat: 0-13 or 16-19\n\
        \n\
        your ticket:\n\
        11,12,13\n\
        \n\
        nearby tickets:\n\
        3,9,18\n\
        15,1,5\n\
        5,14,9";

    #[test]
    fn test_1() {
        let parsed = Input::parse(TEST_INPUT_1);
        assert_eq!(71, parsed.solve_1());
    }

    #[test]
    fn test_2() {
        // just making sure it doesnt panic
        let parsed = Input::parse(TEST_INPUT_2);
        let solved = parsed.solve_2();
        assert_eq!(12*11*13, solved);
    }
}
