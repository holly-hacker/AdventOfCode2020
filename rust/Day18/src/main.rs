use challenge::solve;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    // let (input, time_parsing) = time(|| Input::parse(&stdin));
    let (solution_1, time_solving_1) =
        time(|| stdin.split('\n').map(|line| solve(line)).sum::<isize>());
    // let (solution_2, time_solving_2) = time(|| input.solve_2());

    println!("solution 1: {}", solution_1);
    // println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    // println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    // println!("took {:?} to solve 2", time_solving_2);
}

mod challenge {
    #[derive(Debug)]
    enum Operation {
        Add,
        Multiply,
    }

    pub fn solve(input: &str) -> isize {
        let (mut input, num1) = read_digit(input);
        let mut num1 = num1.unwrap();

        while input.len() > 0 {
            let (input2, operation) = read_operation(input);
            let (input2, num2) = read_digit(input2);
            input = input2;

            num1 = operation.execute(num1, num2.unwrap());
        }

        num1
    }

    impl Operation {
        fn execute(&self, num1: isize, num2: isize) -> isize {
            match self {
                Operation::Add => num1 + num2,
                Operation::Multiply => num1 * num2,
            }
        }
    }

    // read until no more digits
    // TODO: support negation signs
    fn read_digit(mut input: &str) -> (&str, Option<isize>) {
        let mut ret = 0isize;
        let mut iterated_once = false;
        loop {
            let maybe_c = input.bytes().nth(0);
            if maybe_c.is_none() {
                if iterated_once {
                    return (input, Some(ret));
                } else {
                    return (input, None);
                }
            }

            let c = maybe_c.unwrap() as char;

            if c.is_digit(10) {
                ret *= 10;
                ret += c.to_digit(10).unwrap() as isize;
                input = &input[1..];
            } else if c == '(' {
                // need to find next ), but need to take into account nested parens: (x + (y + z))
                input = &input[1..];

                // need to somehow loop until we find the matching paren
                let mut depth = 1;
                let mut close_idx = usize::MAX;
                for (i, b) in input.bytes().enumerate()
                // .filter(|(_, c)| matches!(c, b'(' | b')'))
                {
                    match b {
                        b'(' => depth += 1,
                        b')' => {
                            depth -= 1;
                            if depth == 0 {
                                close_idx = i;
                                break;
                            }
                        }
                        _ => (),
                    }
                }

                let section = &input[0..close_idx];
                ret = solve(section);
                input = &input[close_idx + 1..];
                break;
            } else {
                break;
            }

            iterated_once = true;
        }

        input = skip_whitespace(input);
        (input, Some(ret))
    }

    fn read_operation(input: &str) -> (&str, Operation) {
        (
            skip_whitespace(&input[1..]),
            match input.bytes().nth(0).unwrap() {
                b'+' => Operation::Add,
                b'*' => Operation::Multiply,
                x => panic!("Unknown operation {}", x as char),
            },
        )
    }

    fn skip_whitespace(mut input: &str) -> &str {
        loop {
            let b = input.bytes().nth(0);
            if b.is_none() {
                return input;
            }
            let b = b.unwrap() as char;
            if b.is_whitespace() {
                input = &input[1..];
            } else {
                return input;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::solve;

    #[test]
    fn test_1_normal() {
        assert_eq!(26, solve("2 * 3 + (4 * 5)"));
        assert_eq!(437, solve("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn test_1_nested() {
        assert_eq!(12240, solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(
            13632,
            solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
