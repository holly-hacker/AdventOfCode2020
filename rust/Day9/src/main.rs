include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (mut input, time_parsing) = time(|| parse_input(&stdin));
    let (solution_1, time_solving_1) = time(|| solve_1(&mut input));
    // let (solution_2, time_solving_2) = time(|| solve_2(&mut input));

    println!("solution 1: {}", solution_1);
    // println!("solution 1: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    // println!("took {:?} to solve 2", time_solving_2);
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split('\n').map(|x| x.parse().unwrap()).collect()
}

#[derive(Default)]
struct RollingBuffer<T> {
    buffer: [T; 25],
    idx: usize,
}

impl<T> RollingBuffer<T> where T: std::ops::Add<Output = T> + std::cmp::PartialEq + Copy {
    const SIZE: usize = 25;

    pub fn push(&mut self, data: T) {
        self.buffer[self.idx % Self::SIZE] = data;
        self.idx += 1;
    }

    pub fn is_filled(&self) -> bool {
        self.idx >= Self::SIZE
    }

    pub fn contains_sum(&self, x: T) -> bool {
        for (i, item) in self.buffer.iter().enumerate() {
            if self.buffer.iter().skip(i).any(|item2| (*item + *item2) == x) {
                return true;
            }
        }

        false
    }
}

fn solve_1(data: &[usize]) -> usize {
    let mut buffer = RollingBuffer::<usize>::default();
    let iter = data.into_iter();
    
    for x in iter {
        if buffer.is_filled() && !buffer.contains_sum(*x) {
            return *x;
        }
        buffer.push(*x);
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_1};

    #[test]
    fn test_challenge_1() {
        let parsed = parse_input(include_str!("../input.txt"));
        let solved = solve_1(&parsed);
        assert_eq!(375054920, solved);
    }
}
