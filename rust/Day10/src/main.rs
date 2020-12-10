include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| parse_input(&stdin));
    let (solution_1, time_solving_1) = time(|| solve_1(&input));
    let (solution_2, time_solving_2) = time(|| solve_2(&input));

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2", time_solving_2);
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut vec = input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    vec.sort();
    vec.push(vec.last().unwrap() + 3);
    vec
}

fn count_adapter_jumps(data: &[usize]) -> (usize, usize, usize) {
    let (mut count1, mut count2, mut count3) = (0, 0, 0);

    let mut last = 0;
    for &d in data {
        match d - last {
            1 => count1 += 1,
            2 => count2 += 1,
            3 => count3 += 1,
            _ => panic!(),
        }

        last = d;
    }

    (count1, count2, count3)
}

fn solve_1(data: &[usize]) -> usize {
    let tuple = count_adapter_jumps(data);
    tuple.0 * tuple.2
}

fn count_paths(data: &[usize], map: &mut [usize; 256]) {
    let start = data[0];
    let mut paths = 0;

    for i in 1..=3 {
        if let Some(&first) = data.get(i) {
            if matches!(first - start, 1 | 2 | 3) {
                paths += map[first];
            }
        }
    }

    map[start] = paths;
}

fn solve_2(data: &[usize]) -> usize {
    // TODO: can change with stackbuffer
    let mut map = [0usize; 256];
    map[data[data.len() - 1]] = 1;
    for idx in (0..data.len() - 1).rev() {
        count_paths(&data[idx..], &mut map);
    }

    map[1] + map[2] + map[3]
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE_1: &str = "\
        16\n\
        10\n\
        15\n\
        5\n\
        1\n\
        11\n\
        7\n\
        19\n\
        6\n\
        12\n\
        4";

    #[test]
    fn test_1() {
        let parsed = parse_input(EXAMPLE_1);
        let solution = solve_1(&parsed);
        assert_eq!(5 * 7, solution);
    }

    #[test]
    fn test_2() {
        let parsed = parse_input(EXAMPLE_1);
        let solution = solve_2(&parsed);
        assert_eq!(8, solution);
        // 9023189417984 is too low
    }
}
