use std::fmt::Debug;

use challenge::*;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| Instruction::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| run_for::<ShipPosition>(&input));
    let (solution_2, time_solving_2) = time(|| run_for::<ShipWithWaypoint>(&input));

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2", time_solving_2);
}

fn run_for<T>(instructions: &[Instruction]) -> usize
where
    T: Default,
    T: Debug,
    T: CanSolveChallenge,
{
    T::default()
        .run_instructions(instructions)
        .get_manhattan_distance()
}

mod challenge {
    use std::fmt::Debug;

    #[derive(Debug)]
    pub struct ShipWithWaypoint {
        pub ship_location: Location,
        pub waypoint_offset: Location,
    }

    #[derive(Default, Debug)]
    pub struct ShipPosition {
        pub location: Location,
        pub direction: Direction,
    }

    #[derive(Default, Debug, Copy, Clone)]
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

    pub trait CanSolveChallenge {
        fn run_instructions(mut self, instructions: &[Instruction]) -> Self
        where
            Self: Sized,
            Self: Debug,
        {
            for instruction in instructions {
                self = self.run_instruction(instruction);
            }

            self
        }

        fn run_instruction(self, instruction: &Instruction) -> Self;
        fn get_manhattan_distance(&self) -> usize;
    }

    impl Default for ShipWithWaypoint {
        fn default() -> Self {
            Self {
                ship_location: Location::default(),
                waypoint_offset: Location(10, 1),
            }
        }
    }

    impl CanSolveChallenge for ShipWithWaypoint {
        fn run_instruction(self, instruction: &Instruction) -> Self {
            match instruction.action {
                Action::North => Self {
                    waypoint_offset: self.waypoint_offset.with_y_offset(instruction.value),
                    ..self
                },
                Action::South => Self {
                    waypoint_offset: self.waypoint_offset.with_y_offset(-instruction.value),
                    ..self
                },
                Action::East => Self {
                    waypoint_offset: self.waypoint_offset.with_x_offset(instruction.value),
                    ..self
                },
                Action::West => Self {
                    waypoint_offset: self.waypoint_offset.with_x_offset(-instruction.value),
                    ..self
                },
                Action::Left => Self {
                    waypoint_offset: self.waypoint_offset.rotate(-instruction.value),
                    ..self
                },
                Action::Right => Self {
                    waypoint_offset: self.waypoint_offset.rotate(instruction.value),
                    ..self
                },
                Action::Forward => Self {
                    ship_location: self
                        .ship_location
                        .move_in_direction(self.waypoint_offset, instruction.value),
                    ..self
                },
            }
        }

        fn get_manhattan_distance(&self) -> usize {
            self.ship_location.get_manhattan_distance()
        }
    }

    impl CanSolveChallenge for ShipPosition {
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

        fn get_manhattan_distance(&self) -> usize {
            self.location.get_manhattan_distance()
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

        pub fn move_in_direction(self, direction: Location, amount: i32) -> Self {
            let (x, y) = (direction.0, direction.1);
            Self(self.0 + amount * x, self.1 + amount * y)
        }

        pub fn rotate(self, degrees: i32) -> Self {
            debug_assert!(degrees.abs() % 90 == 0);
            match (degrees + 360) % 360 {
                90 => Self {
                    0: self.1,
                    1: -self.0,
                },
                _ => self.rotate(90).rotate(degrees - 90),
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
    fn test_1() {
        let parsed = Instruction::parse(TEST_INPUT);
        let position = ShipPosition::default();
        let position = position.run_instructions(&parsed);

        assert_eq!(17, position.location.0);
        assert_eq!(-8, position.location.1);
        assert_eq!(25, position.get_manhattan_distance());
    }

    #[test]
    fn test_2() {
        let parsed = Instruction::parse(TEST_INPUT);
        let position = ShipWithWaypoint::default();
        let position = position.run_instructions(&parsed);

        assert_eq!(214, position.ship_location.0);
        assert_eq!(-72, position.ship_location.1);
        assert_eq!(286, position.get_manhattan_distance());
    }
}
