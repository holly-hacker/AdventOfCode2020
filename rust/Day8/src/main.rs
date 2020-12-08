use std::todo;

include!("../../helpers.rs");

#[derive(Default)]
struct VirtualMachine<'a> {
    accumulator: i32,
    instructions: &'a [Instruction],
    instruction_pointer: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            ..Default::default()
        }
    }

    pub fn run_instruction(&mut self) {
        let instruction = self.instructions[self.instruction_pointer];
        let mut instruction_jump = None;
        match instruction {
            Instruction::Acc(i) => {
                self.accumulator += i as i32;
            }
            Instruction::Jmp(i) => {
                instruction_jump = Some(i as i32);
            }
            Instruction::Nop => (),
        }

        self.instruction_pointer = (self.instruction_pointer as i32
            + match instruction_jump {
                Some(j) => j,
                None => 1,
            }) as usize;
    }

    pub fn get_ip(&self) -> usize {
        self.instruction_pointer
    }
    pub fn get_acc(&self) -> i32 {
        self.accumulator
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    Acc(i16),
    Jmp(i16),
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
    let (solution_1, time_solving_1) = time(|| solve_1(&input));

    println!("solution 1: {}", solution_1);
}

fn solve_1(instructions: &Vec<Instruction>) -> i32 {
    let mut recursion_stack = vec![]; // TODO: can be optimized with stack array
    let mut vm = VirtualMachine::new(instructions);

    loop {
        let ip = vm.get_ip();
        if recursion_stack.contains(&ip) {
            return vm.get_acc();
        }
        recursion_stack.push(ip);
        vm.run_instruction();
    }
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
    fn test_parsing() {
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

    #[test]
    fn test_solution_1() {
        let parsed = parse_input(INPUT);
        assert_eq!(5, solve_1(&parsed));
    }
}
