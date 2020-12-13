use challenge::ChallengeData;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (parsed, time_parsing) = time(|| ChallengeData::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| parsed.solve_1());
    let (solution_2, time_solving_2) = time(|| parsed.solve_2());

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2", time_solving_2);
}

mod challenge {
    pub struct ChallengeData {
        pub earliest_timestamp: usize,
        pub busses: Vec<Option<usize>>,
    }

    #[derive(Debug, Default, Copy, Clone)]
    struct BufferItem {
        index: u64,
        bus_id: u64,
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

        pub fn solve_2(&self) -> u64 {
            let mut buffer = [BufferItem::default(); 16];
            self.busses
                .iter()
                .enumerate()
                .filter(|(_, &val)| val.is_some())
                .enumerate()
                .for_each(|(i_buf, (i, &v))| {
                    buffer[i_buf] = BufferItem {
                        index: i as u64,
                        bus_id: v.unwrap() as u64,
                    }
                });

            let buffer_size = buffer
                .iter()
                .enumerate()
                .filter(|(_, item)| item.index != 0)
                .last()
                .unwrap()
                .0
                + 1;

            let buffer = &buffer[..buffer_size];

            let mut start = 0;
            for i in 1..=buffer.len() {
                start = Self::solve_2_inner(&buffer[0..i], start);
            }

            start
        }

        fn solve_2_inner(slice: &[BufferItem], start_idx: u64) -> u64 {
            if slice.len() == 1 {
                return slice[0].bus_id;
            }

            let step: u64 = slice[..slice.len() - 1]
                .iter()
                .map(|item| item.bus_id)
                .product();

            let candidates = (start_idx..u64::MAX).step_by(step as usize);
            for i in candidates.skip(1) {
                let aaa = slice
                    .iter()
                    .all(|&item| (i + item.index) % item.bus_id == 0);
                if aaa {
                    return i;
                }
            }

            unreachable!("Exhausted keyspace");
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

    #[test]
    fn test_2() {
        let parsed = ChallengeData::parse(TEST_INPUT);
        assert_eq!(1068781, parsed.solve_2());
    }

    #[test]
    fn test_2_extra_1() {
        let input = "0\n17,x,13,19"; // 4199-3417=782
        assert_eq!(3417, ChallengeData::parse(input).solve_2());
    }

    #[test]
    fn test_2_extra_2() {
        let input = "0\n67,7,59,61";
        assert_eq!(754018, ChallengeData::parse(input).solve_2());
    }

    #[test]
    fn test_2_extra_3() {
        let input = "0\n67,x,7,59,61";
        assert_eq!(779210, ChallengeData::parse(input).solve_2());
    }

    #[test]
    fn test_2_extra_4() {
        let input = "0\n67,7,x,59,61";
        assert_eq!(1261476, ChallengeData::parse(input).solve_2());
    }

    #[test]
    fn test_2_extra_5() {
        let input = "0\n1789,37,47,1889";
        assert_eq!(1202161486, ChallengeData::parse(input).solve_2());
    }

    #[test]
    fn test_2_extra_own() {
        let input = "0\n2,3,5,7";
        assert_eq!(158, ChallengeData::parse(input).solve_2());
    }
}
