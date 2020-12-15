use std::collections::HashMap;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| parse(&stdin));
    let (solution_1, time_solving_1) = time(|| solve_1(&input));
    // let (solution_2, time_solving_2) = time(|| solve_2(&input));

    println!("solution 1: {}", solution_1);
    // println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    // println!("took {:?} to solve 2", time_solving_2);
}

fn parse(s: &str) -> Vec<usize> {
    s.split(',').map(|x| x.parse().unwrap()).collect()
}

fn solve_1(data: &[usize]) -> usize {
    let mut map = HashMap::<usize, usize>::new();
    for (i, &val) in data.iter().enumerate().take(data.len() - 1) {
        println!("Adding starting number {}", val);
        map.insert(val, i); // 0-0, 3-1, 6-2
    }

    let mut next = *data.iter().last().unwrap();
    // println!("Adding starting number {}", next);
    for i in (data.len() - 1)..2019 {
        let next2 = map
            .get(&next)
            .map(|&n| {
                //println!("last spoken in {}, which has been spoken before in turn {}. returning {}-{}={}", next, n, i, n, i-n);
                i - n
            })
            .unwrap_or_else(|| {
                //println!("last spoken in {}, which is new", next);
                0
            });
        map.insert(next, i);
        next = next2;
    }

    next
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_1() {
        let solved = solve_1(&[0, 3, 6]);
        assert_eq!(436, solved);
    }

    #[test]
    fn test_1_extra() {
        assert_eq!(1, solve_1(&[1, 3, 2]));
        assert_eq!(10, solve_1(&[2, 1, 3]));
        assert_eq!(27, solve_1(&[1, 2, 3]));
        assert_eq!(78, solve_1(&[2, 3, 1]));
        assert_eq!(438, solve_1(&[3, 2, 1]));
        assert_eq!(1836, solve_1(&[3, 1, 2]));
    }
}
