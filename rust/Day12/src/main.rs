use challenge::*;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| Instruction::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| {
        ShipPosition::default()
            .run_instructions(&input)
            .location
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
        pub location: Location,
        pub direction: Direction,
    }

    #[derive(Default, Debug)]
    pub struct Location(pub i32, pub i32);

    pub struct Instruction {
        action: Action,
        value: i32,
    }

    #[derive(Eq, PartialEq, Debug, Copy, Clone)]
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

        fn run_instruction(self, instruction: &Instruction) -> Self {
            match instruction.action {
                Action::North => Self {
                    location: self.location.with_y_offset(instruction.value),
                    ..self
                },
                Action::South => Self {
                    location: self.location.with_y_offset(-instruction.value),
                    ..self
                },
                Action::East => Self {
                    location: self.location.with_x_offset(instruction.value),
                    ..self
                },
                Action::West => Self {
                    location: self.location.with_x_offset(-instruction.value),
                    ..self
                },
                Action::Left => Self {
                    direction: self.direction.rotate_right(-instruction.value),
                    ..self
                },
                Action::Right => Self {
                    direction: self.direction.rotate_right(instruction.value),
                    ..self
                },
                Action::Forward => Self {
                    location: self
                        .location
                        .move_towards(self.direction, instruction.value),
                    ..self
                },
            }
        }
    }

    impl Location {
        pub fn get_manhattan_distance(&self) -> usize {
            (self.0.abs() + self.1.abs()) as usize
        }

        pub fn with_x_offset(self, x: i32) -> Self {
            Self(self.0 + x, self.1)
        }

        pub fn with_y_offset(self, y: i32) -> Self {
            Self(self.0, self.1 + y)
        }

        pub fn move_towards(self, direction: Direction, amount: i32) -> Self {
            let (x, y) = match direction {
                Direction::North => (0, 1),
                Direction::South => (0, -1),
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
            };

            Self(self.0 + amount * x, self.1 + amount * y)
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
        let distance = position.location.get_manhattan_distance();

        assert_eq!(17, position.location.0);
        assert_eq!(-8, position.location.1);
        assert_eq!(25, distance);
    }
}
