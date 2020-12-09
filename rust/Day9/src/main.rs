include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (mut input, time_parsing) = time(|| parse_input(&stdin));
    let (solution_1, time_solving_1) = time(|| solve_1(&mut input));
    let (solution_2, time_solving_2) = time(|| solve_2_fast_backward(&mut input, solution_1));

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2 (used solution 1)", time_solving_2);
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split('\n').map(|x| x.parse().unwrap()).collect()
}

#[derive(Default)]
struct RollingBuffer<T> {
    buffer: [T; 25],
    idx: usize,
}

impl<T> RollingBuffer<T>
where
    T: std::ops::Add<Output = T> + std::cmp::PartialEq + Copy,
{
    const SIZE: usize = 25;

    pub fn push(&mut self, data: T) {
        self.buffer[self.idx % Self::SIZE] = data;
        self.idx += 1;
    }

    pub fn is_filled(&self) -> bool {
        self.idx >= Self::SIZE
    }

    pub fn contains_sum(&self, x: T) -> bool {
        for (i, &item) in self.buffer.iter().enumerate() {
            if self.buffer.iter().skip(i).any(|&item2| (item + item2) == x) {
                return true;
            }
        }

        false
    }
}

fn find_min_max(data: &[usize]) -> (usize, usize) {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for &d in data {
        min = usize::min(min, d);
        max = usize::max(max, d);
    }

    (min, max)
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

#[allow(unused)]
fn solve_2_naive(data: &[usize], to_find: usize) -> usize {
    for i in 0..data.len() {
        for j in i..data.len() {
            if data[i..j].iter().sum::<usize>() == to_find {
                let (min, max) = find_min_max(&data[i..j]);
                return min + max;
            }
        }
    }

    unreachable!()
}

#[allow(unused)]
fn solve_2_fast_forward(data: &[usize], to_find: usize) -> usize {
    let mut start_idx = 0;
    let mut end_idx = 0;

    let mut sum = data[start_idx];
    loop {
        // add to end_idx until we have enough
        while sum < to_find {
            end_idx += 1;
            sum += data[end_idx];
        }

        if sum == to_find {
            let (min, max) = find_min_max(&data[start_idx..end_idx]);
            return min + max;
        }

        // sum > to_find
        sum -= data[start_idx];
        start_idx += 1;

        while sum > to_find {
            sum -= data[end_idx];
            end_idx -= 1;
        }

        if sum == to_find {
            let (min, max) = find_min_max(&data[start_idx..end_idx]);
            return min + max;
        }
    }
}

fn solve_2_fast_backward(data: &[usize], to_find: usize) -> usize {
    let to_find_idx = data
        .iter()
        .enumerate()
        .find(|(_, &num)| num == to_find)
        .unwrap()
        .0;
    let mut start_idx = to_find_idx - 1;
    let mut end_idx = start_idx;

    let mut sum = data[start_idx];
    loop {
        // remove from start_idx until we have enough
        while sum < to_find {
            start_idx -= 1;
            sum += data[start_idx];
        }

        if sum == to_find {
            let (min, max) = find_min_max(&data[start_idx..end_idx]);
            return min + max;
        }

        // sum > to_find
        sum -= data[end_idx];
        end_idx -= 1;

        while sum > to_find {
            sum -= data[start_idx];
            start_idx += 1;
        }

        if sum == to_find {
            let (min, max) = find_min_max(&data[start_idx..end_idx]);
            return min + max;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_challenge_1() {
        let parsed = parse_input(include_str!("../input.txt"));
        let solved = solve_1(&parsed);
        assert_eq!(375054920, solved);
    }

    #[test]
    fn test_challenge_2_naive() {
        let parsed = parse_input(include_str!("../input.txt"));
        let solved_1 = solve_1(&parsed);
        let solved = solve_2_naive(&parsed, solved_1);
        assert_eq!(54142584, solved);
    }

    #[test]
    fn test_challenge_2_fast_forward() {
        let parsed = parse_input(include_str!("../input.txt"));
        let solved_1 = solve_1(&parsed);
        let solved = solve_2_fast_forward(&parsed, solved_1);
        assert_eq!(54142584, solved);
    }

    #[test]
    fn test_challenge_2_fast_backward() {
        let parsed = parse_input(include_str!("../input.txt"));
        let solved_1 = solve_1(&parsed);
        let solved = solve_2_fast_backward(&parsed, solved_1);
        assert_eq!(54142584, solved);
    }
}
