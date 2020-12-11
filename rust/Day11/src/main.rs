use std::fmt::Debug;

include!("../../helpers.rs");

#[derive(Copy, Clone, Eq, PartialEq)]
enum Seat {
    OutOfBounds,
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

    pub fn to_char(&self) -> char {
        match self {
            Seat::OutOfBounds => panic!(),
            Seat::Floor => '.',
            Seat::Empty => 'L',
            Seat::Occupied => '#',
        }
    }
}

#[derive(Clone)]
struct SeatConfiguration {
    width: usize,
    height: usize,
    buffer1: Vec<Vec<Seat>>,
    buffer2: Vec<Vec<Seat>>,
    current_buffer: bool,
}

impl SeatConfiguration {
    pub fn parse(input: &str) -> SeatConfiguration {
        let buffer1 = Self::parse_input(input);
        let buffer2 = buffer1.iter().map(|x| x.to_vec()).collect();

        Self {
            width: buffer1.len(),
            height: buffer1[0].len(),
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
            self.run_iteration(Self::calculate_direct_neighbours, 4);
            if self.do_buffers_match() {
                return self.count_taken_seats();
            }
        }
    }

    pub fn solve_2(&mut self) -> usize {
        loop {
            self.run_iteration(Self::calculate_indirect_neighbours, 5);
            if self.do_buffers_match() {
                return self.count_taken_seats();
            }
        }
    }

    fn count_taken_seats(&self) -> usize {
        self.buffer1
            .iter()
            .flat_map(|v| v.iter())
            .filter(|&&x| x == Seat::Occupied)
            .count()
    }

    fn run_iteration<F>(&mut self, f: F, max_seats: usize)
    where
        F: Fn(&SeatConfiguration, usize, usize) -> usize,
    {
        for x in 0..self.width {
            for y in 0..self.height {
                self.get_buffers_mut().1[x as usize][y as usize] =
                    self.calculate_layout(x, y, &f, max_seats);
            }
        }

        self.swap_buffers();
    }

    fn do_buffers_match(&self) -> bool {
        let width = self.width;

        let (source, target) = self.get_buffers();
        for x in 0..width as usize {
            if !source[x].iter().zip(&target[x]).all(|(&a, &b)| a == b) {
                return false;
            }
        }

        true
    }

    fn calculate_layout<F>(&self, x: usize, y: usize, f: &F, max_seats: usize) -> Seat
    where
        F: Fn(&SeatConfiguration, usize, usize) -> usize,
    {
        // possible optimization: seats on border shouldn't really change once occupied
        // possible optimization: only iterate enough times to see if we're over max_seats
        let sum = f(&self, x, y);

        let (source, _) = self.get_buffers();
        match source[x as usize][y as usize] {
            Seat::Floor => Seat::Floor,
            Seat::Empty if sum == 0 => Seat::Occupied,
            Seat::Occupied if sum >= max_seats => Seat::Empty,
            _ => source[x as usize][y as usize],
        }
    }

    fn calculate_direct_neighbours(&self, x: usize, y: usize) -> usize {
        let mut sum = 0;
        for x1 in -1..=1 {
            for y1 in -1..=1 {
                if x1 == 0 && y1 == 0 {
                    continue;
                }
                sum += if self.is_occupied(x as i32 + x1, y as i32 + y1) == Seat::Occupied {
                    1
                } else {
                    0
                }
            }
        }
        sum
    }

    fn calculate_indirect_neighbours(&self, x: usize, y: usize) -> usize {
        let mut sum = 0;
        for x1 in -1..=1 {
            for y1 in -1..=1 {
                if x1 == 0 && y1 == 0 {
                    continue;
                }

                let mut x = x as i32;
                let mut y = y as i32;
                let found = loop {
                    x += x1;
                    y += y1;
                    let occupation = self.is_occupied(x, y);
                    match occupation {
                        Seat::OutOfBounds => break false,
                        Seat::Empty => break false,
                        Seat::Occupied => break true,
                        Seat::Floor => continue,
                    }
                };

                sum += if found { 1 } else { 0 }
            }
        }
        sum
    }

    fn is_occupied(&self, x: i32, y: i32) -> Seat {
        if x < 0 || x >= self.width as i32 {
            Seat::OutOfBounds
        } else if y < 0 || y >= self.height as i32 {
            Seat::OutOfBounds
        } else {
            let (buffer, _) = self.get_buffers();
            buffer[x as usize][y as usize]
        }
    }

    fn get_buffers(&self) -> (&Vec<Vec<Seat>>, &Vec<Vec<Seat>>) {
        match self.current_buffer {
            false => (&self.buffer1, &self.buffer2),
            true => (&self.buffer2, &self.buffer1),
        }
    }

    fn get_buffers_mut(&mut self) -> (&mut Vec<Vec<Seat>>, &mut Vec<Vec<Seat>>) {
        match self.current_buffer {
            false => (&mut self.buffer1, &mut self.buffer2),
            true => (&mut self.buffer2, &mut self.buffer1),
        }
    }

    fn swap_buffers(&mut self) {
        self.current_buffer = !self.current_buffer;
    }
}

impl Debug for SeatConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.get_buffers().0 {
            f.write_str(&line.iter().map(|b| b.to_char()).collect::<String>())?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| SeatConfiguration::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| input.clone().solve_1());
    let (solution_2, time_solving_2) = time(|| input.clone().solve_2());

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2", time_solving_2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "\
        L.LL.LL.LL\n\
        LLLLLLL.LL\n\
        L.L.L..L..\n\
        LLLL.LL.LL\n\
        L.LL.LL.LL\n\
        L.LLLLL.LL\n\
        ..L.L.....\n\
        LLLLLLLLLL\n\
        L.LLLLLL.L\n\
        L.LLLLL.LL";

    #[test]
    fn test_1() {
        let mut parsed = SeatConfiguration::parse(TEST_INPUT);
        assert_eq!(37, parsed.solve_1());
    }

    #[test]
    fn test_2() {
        let mut parsed = SeatConfiguration::parse(TEST_INPUT);
        assert_eq!(26, parsed.solve_2());
    }
}
