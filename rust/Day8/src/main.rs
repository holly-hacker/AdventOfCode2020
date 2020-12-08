include!("../../helpers.rs");

struct VirtualMachine {
    accumulator: i32,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Acc(i8),
    Jmp(i8),
    Nop,
}

impl Instruction {
    pub fn parse(string: &str) -> Self {
        let mut split = string.split(' ');
        match split.next() {
            Some("nop") => Instruction::Nop,
            Some("acc") => Instruction::Acc(split.next().unwrap().parse().unwrap()),
            Some("jmp") => Instruction::Jmp(split.next().unwrap().parse().unwrap()),
            _ => panic!("Unknown opcode"),
        }
    }
}

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| parse_input(&stdin));
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .filter(|l| *l != "")
        .map(Instruction::parse)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "\
        nop +0\n\
        acc +1\n\
        jmp +4\n\
        acc +3\n\
        jmp -3\n\
        acc -99\n\
        acc +1\n\
        jmp -4\n\
        acc +6";

    #[test]
    fn test() {
        let parsed = parse_input(INPUT);

        let expected = vec![
            Instruction::Nop,
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ];

        assert_eq!(expected.len(), parsed.len());
        for i in 0..parsed.len() {
            assert_eq!(expected[i], parsed[i]);
        }
    }
}
