include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| parse(&stdin));
    let (solution_1, time_solving_1) = time(|| solve(&input, 2020));
    let (solution_2, time_solving_2) = time(|| solve(&input, 30000000));

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2", time_solving_2);
}

fn parse(s: &str) -> Vec<usize> {
    s.split(',').map(|x| x.parse().unwrap()).collect()
}

fn solve(data: &[usize], limit: usize) -> usize {
    let mut map = vec![usize::MAX; limit];
    for (i, &val) in data.iter().enumerate().take(data.len() - 1) {
        // println!("Adding starting number {}", val);
        map[val] = i; // 0-0, 3-1, 6-2
    }

    let mut next = *data.iter().last().unwrap();
    // println!("Adding starting number {}", next);
    for i in (data.len() - 1)..(limit - 1) {
        let mut next2 = map[next];
        next2 = if next2 != usize::MAX {
            //println!("last spoken in {}, which has been spoken before in turn {}. returning {}-{}={}", next, n, i, n, i-n);
            i - next2
        } else {
            //println!("last spoken in {}, which is new", next);
            0
        };
        map[next] = i;
        next = next2;
    }

    next
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_1() {
        let solved = solve(&[0, 3, 6], 2020);
        assert_eq!(436, solved);
    }

    #[test]
    fn test_1_extra() {
        assert_eq!(1, solve(&[1, 3, 2], 2020));
        assert_eq!(10, solve(&[2, 1, 3], 2020));
        assert_eq!(27, solve(&[1, 2, 3], 2020));
        assert_eq!(78, solve(&[2, 3, 1], 2020));
        assert_eq!(438, solve(&[3, 2, 1], 2020));
        assert_eq!(1836, solve(&[3, 1, 2], 2020));
    }

    #[test]
    fn test_2() {
        let solved = solve(&[0, 3, 6], 30000000);
        assert_eq!(175594, solved);
    }

    #[test]
    fn test_2_extra() {
        assert_eq!(2578, solve(&[1, 3, 2], 30000000));
        assert_eq!(3544142, solve(&[2, 1, 3], 30000000));
        assert_eq!(261214, solve(&[1, 2, 3], 30000000));
        assert_eq!(6895259, solve(&[2, 3, 1], 30000000));
        assert_eq!(18, solve(&[3, 2, 1], 30000000));
        assert_eq!(362, solve(&[3, 1, 2], 30000000));
    }
}
