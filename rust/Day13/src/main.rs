use challenge::ChallengeData;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (parsed, time_parsing) = time(|| ChallengeData::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| parsed.solve_1());

    println!("solution 1: {}", solution_1);
    // println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    // println!("took {:?} to solve 2", time_solving_2);
}

mod challenge {
    pub struct ChallengeData {
        pub earliest_timestamp: usize,
        pub busses: Vec<Option<usize>>,
    }

    impl ChallengeData {
        pub fn parse(data: &str) -> Self {
            let mut lines = data.split('\n');

            let line1 = lines.next().unwrap();
            let earliest_timestamp = line1.parse().unwrap();

            let line2 = lines.next().unwrap();
            let busses = line2.split(',').map(|s| s.parse().ok()).collect();

            Self {
                earliest_timestamp,
                busses,
            }
        }

        pub fn solve_1(&self) -> usize {
            let (id, time) = self
                .busses
                .iter()
                .filter(|&&x| x.is_some())
                .map(|&x| (x.unwrap(), self.earliest_timestamp % x.unwrap()))
                .max_by_key(|(_, x)| *x)
                .unwrap();

            id * (id - time)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::ChallengeData;

    const TEST_INPUT: &str = "939\n7,13,x,x,59,x,31,19";

    #[test]
    fn test_1() {
        let parsed = ChallengeData::parse(TEST_INPUT);
        assert_eq!(295, parsed.solve_1());
    }
}
