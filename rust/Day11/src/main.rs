include!("../../helpers.rs");

#[derive(Copy, Clone, Eq, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Seat {
    pub fn parse(b: u8) -> Seat {
        match b {
            b'.' => Seat::Floor,
            b'L' => Seat::Empty,
            b'#' => Seat::Occupied,
            _ => panic!("Tried to parse byte {}", b),
        }
    }
}

#[derive(Clone)]
struct SeatConfiguration {
    width: i32,
    height: i32,
    buffer1: Vec<Vec<Seat>>,
    buffer2: Vec<Vec<Seat>>,
    current_buffer: bool,
}

impl SeatConfiguration {
    pub fn parse(input: &str) -> SeatConfiguration {
        let buffer1 = Self::parse_input(input);
        let buffer2 = buffer1.iter().map(|x| x.to_vec()).collect();

        Self {
            width: buffer1.len() as i32,
            height: buffer1[0].len() as i32,
            buffer1,
            buffer2,
            current_buffer: false,
        }
    }

    fn parse_input(input: &str) -> Vec<Vec<Seat>> {
        let mut total = vec![];

        for line in input.split('\n') {
            total.push(line.as_bytes().iter().map(|&b| Seat::parse(b)).collect());
        }

        total
    }

    pub fn solve_1(&mut self) -> usize {
        loop {
            self.run_iteration();
            if self.do_buffers_match() {
                return self.buffer1.iter().flat_map(|v| v.iter()).filter(|&&x| x == Seat::Occupied).count();
            }
        }
    }

    fn run_iteration(&mut self) {
        let width = self.width;
        let height = self.height;

        for x in 0..width {
            for y in 0..height {
                let new = self.calculate_new_config(x, y);
                let (_, target) = self.get_buffers();
                target[x as usize][y as usize] = new;
            }
        }

        self.swap_buffers();
    }

    fn do_buffers_match(&mut self) -> bool {
        let width = self.width;

        let (source, target) = self.get_buffers();
        for x in 0..width as usize {
            if !source[x].iter().zip(&target[x]).all(|(&a, &b)| a == b) {
                return false;
            }
        }

        true
    }

    fn calculate_new_config(&mut self, x: i32, y: i32) -> Seat {
        // possible optimization: seats on border shouldn't really change once occupied
        let (tl, to, tr) = (
            self.is_occupied(x - 1, y - 1),
            self.is_occupied(x + 0, y - 1),
            self.is_occupied(x + 1, y - 1),
        );
        let (ml, mr) = (
            self.is_occupied(x - 1, y + 0),
            //self.is_occupied(x + 0, y + 0),
            self.is_occupied(x + 1, y + 0),
        );
        let (bl, bo, br) = (
            self.is_occupied(x - 1, y + 1),
            self.is_occupied(x + 0, y + 1),
            self.is_occupied(x + 1, y + 1),
        );
        let sum: usize = [tl, to, tr, ml, mr, bl, bo, br]
            .iter()
            .map(|&b| if b { 1 } else { 0 })
            .sum();

        let (source, _) = self.get_buffers();
        match source[x as usize][y as usize] {
            Seat::Floor => Seat::Floor,
            Seat::Empty if sum == 0 => Seat::Occupied,
            Seat::Occupied if sum >= 4 => Seat::Empty,
            _ => source[x as usize][y as usize],
        }
    }

    fn is_occupied(&mut self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width {
            false
        } else if y < 0 || y >= self.height {
            false
        } else {
            let (buffer, _) = self.get_buffers();
            buffer[x as usize][y as usize] == Seat::Occupied
        }
    }

    fn get_buffers(&mut self) -> (&mut Vec<Vec<Seat>>, &mut Vec<Vec<Seat>>) {
        match self.current_buffer {
            false => (&mut self.buffer1, &mut self.buffer2),
            true => (&mut self.buffer2, &mut self.buffer1),
        }
    }

    fn swap_buffers(&mut self) {
        self.current_buffer = !self.current_buffer;
    }
}

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| SeatConfiguration::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| input.clone().solve_1());
    // let (solution_2, time_solving_2) = time(|| input.solve_2(&mut input, solution_1));

    println!("solution 1: {}", solution_1);
    // println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    // println!("took {:?} to solve 2", time_solving_2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "\
        #.##.##.##\n\
        #######.##\n\
        #.#.#..#..\n\
        ####.##.##\n\
        #.##.##.##\n\
        #.#####.##\n\
        ..#.#.....\n\
        ##########\n\
        #.######.#\n\
        #.#####.##";

    #[test]
    fn test() {
        let mut parsed = SeatConfiguration::parse(TEST_INPUT);
        assert_eq!(37, parsed.solve_1());
    }
}
