include!("../../helpers.rs");

#[derive(Default)]
struct VirtualMachine<'a> {
    accumulator: i32,
    instructions: &'a mut [Instruction],
    instruction_pointer: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(instructions: &'a mut [Instruction]) -> Self {
        Self {
            instructions,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.instruction_pointer = 0;
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
            Instruction::Nop(_) => (),
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

    pub fn run_until_recursion(&mut self) -> bool {
        let mut recursion_stack = vec![]; // TODO: can be optimized with stack array

        loop {
            let ip = self.get_ip();
            if ip == self.instructions.len() {
                return true;
            }
            if recursion_stack.contains(&ip) {
                return false;
            }
            recursion_stack.push(ip);
            self.run_instruction();
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    Acc(i16),
    Jmp(i16),
    Nop(i16),
}

impl Instruction {
    pub fn parse(string: &str) -> Self {
        let mut split = string.split(' ');

        let opcode = split.next();
        let operand = split.next().unwrap().parse().unwrap();
        match opcode {
            Some("nop") => Instruction::Nop(operand),
            Some("acc") => Instruction::Acc(operand),
            Some("jmp") => Instruction::Jmp(operand),
            _ => panic!("Unknown opcode"),
        }
    }
}

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (mut input, time_parsing) = time(|| parse_input(&stdin));
    let (solution_1, time_solving_1) = time(|| solve_1(&mut input));
    let (solution_2, time_solving_2) = time(|| solve_2(&mut input));

    println!("solution 1: {}", solution_1);
    println!("solution 1: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2", time_solving_2);
}

fn solve_1(instructions: &mut [Instruction]) -> i32 {
    let mut vm = VirtualMachine::new(instructions);
    let b = vm.run_until_recursion();
    assert!(!b);
    vm.get_acc()
}

fn solve_2(instructions: &mut [Instruction]) -> i32 {
    let mut vm = VirtualMachine::new(instructions);
    for i in 0..vm.instructions.len() {
        vm.instructions[i] = match vm.instructions[i] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(op) => Instruction::Nop(op),
            Instruction::Nop(op) => Instruction::Jmp(op),
        };

        if vm.run_until_recursion() {
            return vm.get_acc();
        }

        vm.reset();

        vm.instructions[i] = match vm.instructions[i] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(op) => Instruction::Nop(op),
            Instruction::Nop(op) => Instruction::Jmp(op),
        };
    }

    panic!("couldnt find good mutation of the program")
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
            Instruction::Nop(0),
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
        let mut parsed = parse_input(INPUT);
        assert_eq!(5, solve_1(&mut parsed));
    }

    #[test]
    fn test_solution_2() {
        let mut parsed = parse_input(INPUT);
        assert_eq!(8, solve_2(&mut parsed));
    }
}
