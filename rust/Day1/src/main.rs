use std::{io::BufRead, time::Instant};

// const TEST_INPUT1: &[usize] = &[1721, 979, 366, 299, 675, 1456];

fn main() {
    let start = Instant::now();

    // scope to ensure everything is dropped before we check final
    {
        let now = Instant::now();
        let lines = read_challenge_input();
        let time_reading = now.elapsed();
        println!("took {:?} to read input", time_reading);
        
        let now = Instant::now();
        let solution = solve_challenge_1(&lines[..]);
        let time_solving = now.elapsed();
        println!("took {:?} to solve", time_solving);
        println!("solution 1: {}", solution);
        
        let now = Instant::now();
        let solution = solve_challenge_2(&lines[..]);
        let time_solving = now.elapsed();
        println!("took {:?} to solve", time_solving);
        println!("solution 2: {}", solution);
    }

    println!("took {:?} in total", start.elapsed());
}

fn read_challenge_input() -> Vec<usize> {
    std::io::stdin().lock().lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

fn solve_challenge_1(input: &[usize]) -> usize {
    for i in 0..input.len() {
        let operand1 = input[i];
        let mut iter = input.iter().skip(i).filter(|i| *i + operand1 == 2020);
        if let Some(operand2) = iter.next() {
            return operand1 * *operand2;
        }
    }

    unreachable!()
}

fn solve_challenge_2(input: &[usize]) -> usize {
    for i in 0..input.len() {
        let operand1 = input[i];
        let iter = input.iter().enumerate().skip(i).filter(|(_, i)| *i + operand1 <= 2020);
        
        for (idx, operand2) in iter {
            let mut iter2 = input.iter().skip(idx).filter(|i| *i + operand1 + operand2 == 2020);
            if let Some(operand3) = iter2.next() {
                return operand1 * operand2 * *operand3;
            }
        }
    }

    unreachable!()
}
