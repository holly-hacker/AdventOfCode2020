use std::time::Instant;

#[derive(Debug)]
struct Map(Vec<Vec<bool>>);

impl Map {
    #[cfg(test)]
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| if *y { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn solve_1(&self) -> usize {
        self.calculate_for_idx(3)
    }

    pub fn solve_2(&self) -> usize {
        self.calculate_for_idx(1)
            * self.calculate_for_idx(3)
            * self.calculate_for_idx(5)
            * self.calculate_for_idx(7)
            * self.calculate_for_double_idx(1)
    }

    fn calculate_for_idx(&self, skip: usize) -> usize {
        self.0
            .iter()
            .enumerate()
            .skip(1)
            .map(|(idx, v)| if v[(idx * skip) % v.len()] { 1 } else { 0 })
            .sum()
    }

    fn calculate_for_double_idx(&self, skip: usize) -> usize {
        self.0
            .iter()
            .step_by(2)
            .enumerate()
            .skip(1)
            .map(|(idx, v)| if v[(idx * skip) % v.len()] { 1 } else { 0 })
            .sum()
    }
}

fn main() {
    let time_total = Instant::now();

    {
        let time_reading = Instant::now();
        let input = parse_stdin();
        println!("took {:?} to read input", time_reading.elapsed());

        let time_solving = Instant::now();
        let count_correct = input.solve_1();
        println!("took {:?} to solve 1", time_solving.elapsed());
        println!("solution 1: {}", count_correct);

        let time_solving = Instant::now();
        let count_correct = input.solve_2();
        println!("took {:?} to solve 2", time_solving.elapsed());
        println!("solution 2: {}", count_correct);
    }

    println!("took {:?} in total", time_total.elapsed());
}

fn parse_stdin() -> Map {
    use std::io::Read;
    let mut input_string = String::new();
    std::io::stdin().read_to_string(&mut input_string).expect("Failed to parse input");
    parse_string(&input_string)
}

fn parse_string(input: &str) -> Map {
    let mut total_vec = Vec::new();
    for line in input.split('\n') {
        if line.len() == 0 {
            break;
        }
        let vec: Vec<bool> = line
            .bytes()
            .map(|b| match b as char {
                '#' => true,
                '.' => false,
                _ => panic!("invalid input char: {}", b as char),
            })
            .collect();
        total_vec.push(vec);
    }
    Map(total_vec)
}

#[cfg(test)]
mod tests {
    use crate::parse_string;

    const TEST_INPUT: &str = "\
        ..##.......\n\
        #...#...#..\n\
        .#....#..#.\n\
        ..#.#...#.#\n\
        .#...##..#.\n\
        ..#.##.....\n\
        .#.#.#....#\n\
        .#........#\n\
        #.##...#...\n\
        #...##....#\n\
        .#..#...#.#";

    #[test]
    fn test_parse() {
        let parsed = parse_string(TEST_INPUT);
        assert_eq!(parsed.to_string(), TEST_INPUT);
    }

    #[test]
    fn test_1() {
        let parsed = parse_string(TEST_INPUT);
        assert_eq!(parsed.solve_1(), 7);
    }

    #[test]
    fn test_2() {
        let parsed = parse_string(TEST_INPUT);
        assert_eq!(parsed.solve_2(), 336);
    }
}
