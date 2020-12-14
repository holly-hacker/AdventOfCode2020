use challenge::*;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| ProgramInstruction::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| ProgramInstruction::execute(&input));
    // let (solution_2, time_solving_2) = time(|| run(&input));

    println!("solution 1: {}", solution_1);
    // println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    // println!("took {:?} to solve 2", time_solving_2);
}

mod challenge {
    use std::collections::HashMap;

    #[derive(Debug, Copy, Clone)]
    pub enum ProgramInstruction {
        Mask(BitMask),
        Write(MemoryWrite),
    }

    #[derive(Debug, Copy, Clone, Default)]
    pub struct BitMask {
        mask: u64,
        overwrite: u64,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct MemoryWrite {
        offset: u16,
        value: u64,
    }

    impl ProgramInstruction {
        pub fn parse(input: &str) -> Vec<Self> {
            input.split('\n').map(Self::parse_line).collect()
        }

        fn parse_line(input: &str) -> Self {
            match &input[0..4] {
                "mask" => Self::Mask(BitMask::parse(input)),
                "mem[" => Self::Write(MemoryWrite::parse(input)),
                _ => panic!("Unknown input: {}", input),
            }
        }

        pub fn execute(input: &[Self]) -> u64 {
            let mut map = HashMap::<u16, u64>::new();

            let mut current_mask = BitMask::default();
            for &instruction in input {
                match instruction {
                    ProgramInstruction::Mask(m) => current_mask = m,
                    ProgramInstruction::Write(w) => {
                        let masked = current_mask.mask_value(w.value);
                        map.insert(w.offset, masked);
                    }
                }
            }

            map.values().sum::<u64>()
        }
    }

    impl BitMask {
        fn parse(input: &str) -> Self {
            let data = &input[7..];
            debug_assert_eq!(36, data.len());

            let mut mask = 0u64;
            let mut overwrite = 0u64;
            for (i, b) in data.bytes().enumerate() {
                let bit = 36 - i - 1;
                mask |= match b {
                    b'1' | b'0' => 1,
                    b'X' => 0,
                    _ => panic!(),
                } << bit;
                overwrite |= match b {
                    b'1' => 1,
                    b'X' | b'0' => 0,
                    _ => panic!(),
                } << bit;
            }

            Self { mask, overwrite }
        }

        pub fn mask_value(&self, mut value: u64) -> u64 {
            let mask_ones = self.mask & self.overwrite;
            let mask_zeroes = self.mask & !self.overwrite;

            value = value | mask_ones;
            value = !(!value | mask_zeroes);

            value
        }
    }

    impl MemoryWrite {
        fn parse(input: &str) -> Self {
            let mut split = input.split(']');
            let split1 = split.next().unwrap();
            let offset = (&split1[4..]).parse().unwrap();

            let split2 = split.next().unwrap();
            let value = (&split2[3..]).parse().unwrap();

            Self { offset, value }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::*;

    const TEST_INPUT: &str = "\
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
        mem[8] = 11\n\
        mem[7] = 101\n\
        mem[8] = 0";

    #[test]
    fn test_1() {
        let parsed = ProgramInstruction::parse(TEST_INPUT);
        let result = ProgramInstruction::execute(&parsed);

        assert_eq!(165, result);
    }
}
