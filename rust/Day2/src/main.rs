std::include!("../../helpers.rs");

#[derive(Debug)]
struct PolicyWithPassword {
    pub policy: Policy,
    pub password: String,
}

#[derive(Debug)]
struct Policy {
    pub min: usize,
    pub max: usize,
    pub chr: u8,
}

impl PolicyWithPassword {
    pub fn check_1(&self) -> bool {
        let iter = self.password.bytes().filter(|c| *c == self.policy.chr);
        let count = iter.take(self.policy.max + 1).count();
        count >= self.policy.min && count <= self.policy.max
    }

    pub fn check_2(&self) -> bool {
        let min = self.password.bytes().nth(self.policy.min - 1).unwrap();
        let max = self.password.bytes().nth(self.policy.max - 1).unwrap();
        (min == self.policy.chr) ^ (max == self.policy.chr)
    }
}

fn main() {
    let (_, time_total) = time(|| {
        let (stdin, time_reading) = time(|| read_stdin());
        let (passwords, time_parsing) = time(|| read_input(&stdin));
        let (count_correct_1, time_solving_1) = time(|| passwords.iter().filter(|p| p.check_1()).count());
        let (count_correct_2, time_solving_2) = time(|| passwords.iter().filter(|p| p.check_2()).count());

        println!("solution 1: {}", count_correct_1);
        println!("solution 2: {}", count_correct_2);
        println!("took {:?} to read stdin", time_reading);
        println!("took {:?} to parse input", time_parsing);
        println!("took {:?} to solve 1", time_solving_1);
        println!("took {:?} to solve 2", time_solving_2);
    });

    println!("took {:?} in total", time_total);
}

fn read_input(input: &str) -> Vec<PolicyWithPassword> {
    input.split('\n').filter(|l| l != &"").map(parse_line).collect()
}

fn parse_line(line: &str) -> PolicyWithPassword {
    // parse int
    let mut split = line.split(' ');

    let split1 = split.next().unwrap();
    let mut split1 = split1.split('-');
    let min = split1.next().unwrap().parse().unwrap();
    let max = split1.next().unwrap().parse().unwrap();

    let split2 = split.next().unwrap();
    let chr = split2.bytes().next().unwrap();

    let split3 = split.next().unwrap();
    let password = split3;

    PolicyWithPassword {
        policy: Policy {
            min, max, chr,
        },
        password: password.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_DATA: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n";

    #[test]
    fn test1() {
        let parsed = read_input(TEST_DATA);
        for i in 0..3 {
            let parsed = dbg!(parsed.get(i)).unwrap();
            let correct = dbg!(parsed.check_1());

            match i {
                0 => assert!(correct && parsed.password == "abcde"),
                1 => assert!(!correct && parsed.password == "cdefg"),
                2 => assert!(correct && parsed.password == "ccccccccc"),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn test2() {
        let parsed = read_input(TEST_DATA);
        for i in 0..3 {
            let parsed = dbg!(parsed.get(i)).unwrap();
            let correct = dbg!(parsed.check_2());

            match i {
                0 => assert!(correct),
                1 => assert!(!correct),
                2 => assert!(!correct),
                _ => panic!(),
            }
        }
    }
}
