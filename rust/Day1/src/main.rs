std::include!("../../helpers.rs");

// const TEST_INPUT1: &[usize] = &[1721, 979, 366, 299, 675, 1456];

fn main() {
    let ((), time_total) = time(||
    {
        let (stdin, time_reading) = time(|| read_stdin());
        let (lines, time_parsing) = time(|| read_challenge_input(stdin));
        
        let (solution_1, time_solving_1) = time(|| solve_challenge_1(&lines[..]));
        let (solution_2, time_solving_2) = time(|| solve_challenge_2(&lines[..]));
        
        println!("solution 1: {}", solution_1);
        println!("solution 2: {}", solution_2);
        println!("took {:?} to read input", time_reading);
        println!("took {:?} to parse input", time_parsing);
        println!("took {:?} to solve", time_solving_1);
        println!("took {:?} to solve", time_solving_2);
    });

    println!("took {:?} in total", time_total);
}

fn read_challenge_input(input: String) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
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
