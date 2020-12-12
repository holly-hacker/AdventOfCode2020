use challenge::*;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| Instruction::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| {
        ShipPosition::default()
            .run_instructions(&input)
            .get_manhattan_distance()
    });

    println!("solution 1: {}", solution_1);
    // println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    // println!("took {:?} to solve 2", time_solving_2);
}

mod challenge {
    #[derive(Default, Debug)]
    pub struct ShipPosition {
        pub x: i32,
        pub y: i32,
        pub direction: Direction,
    }

    pub struct Instruction {
        action: Action,
        value: i32,
    }

    #[derive(Eq, PartialEq, Debug)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }

    enum Action {
        North,
        South,
        East,
        West,
        Left,
        Right,
        Forward,
    }

    impl ShipPosition {
        pub fn run_instructions(self, instructions: &[Instruction]) -> Self {
            let mut x = self;
            for instruction in instructions {
                x = x.run_instruction(instruction);
            }

            x
        }

        pub fn get_manhattan_distance(&self) -> usize {
            (self.x.abs() + self.y.abs()) as usize
        }

        fn run_instruction(self, instruction: &Instruction) -> Self {
            match instruction.action {
                Action::North => self.with_y_offset(instruction.value),
                Action::South => self.with_y_offset(-instruction.value),
                Action::East => self.with_x_offset(instruction.value),
                Action::West => self.with_x_offset(-instruction.value),
                Action::Left => self.rotate(-instruction.value),
                Action::Right => self.rotate(instruction.value),
                Action::Forward => self.forward(instruction.value),
            }
        }

        fn with_x_offset(self, x: i32) -> Self {
            Self {
                x: self.x + x,
                ..self
            }
        }

        fn with_y_offset(self, y: i32) -> Self {
            Self {
                y: self.y + y,
                ..self
            }
        }

        fn rotate(self, degrees: i32) -> Self {
            Self {
                direction: self.direction.rotate_right(degrees),
                ..self
            }
        }

        fn forward(self, amount: i32) -> Self {
            let (x, y) = match self.direction {
                Direction::North => (0, 1),
                Direction::South => (0, -1),
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
            };

            Self {
                x: self.x + amount * x,
                y: self.y + amount * y,
                ..self
            }
        }
    }

    impl Default for Direction {
        fn default() -> Self {
            Self::East
        }
    }

    impl Direction {
        pub fn rotate_right(&self, degrees: i32) -> Self {
            debug_assert!(degrees.abs() % 90 == 0);
            match (degrees + 360) % 360 {
                90 => match *self {
                    Self::North => Self::East,
                    Self::East => Self::South,
                    Self::South => Self::West,
                    Self::West => Self::North,
                },
                _ => self.rotate_right(90).rotate_right(degrees - 90),
            }
        }
    }

    impl Action {
        fn parse(c: u8) -> Self {
            match c {
                b'N' => Self::North,
                b'S' => Self::South,
                b'E' => Self::East,
                b'W' => Self::West,
                b'L' => Self::Left,
                b'R' => Self::Right,
                b'F' => Self::Forward,
                _ => panic!(),
            }
        }
    }

    impl Instruction {
        pub fn parse(data: &str) -> Vec<Self> {
            data.split('\n').map(Self::parse_single).collect()
        }

        fn parse_single(line: &str) -> Self {
            Self {
                action: Action::parse(line.bytes().next().unwrap()),
                value: line[1..].parse().unwrap(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::*;

    const TEST_INPUT: &str = "F10\nN3\nF7\nR90\nF11";

    #[test]
    fn test() {
        let parsed = Instruction::parse(TEST_INPUT);
        let position = ShipPosition::default();
        let position = position.run_instructions(&parsed);
        let distance = position.get_manhattan_distance();

        assert_eq!(17, position.x);
        assert_eq!(-8, position.y);
        assert_eq!(25, distance);
    }
}
