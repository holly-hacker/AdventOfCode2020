use challenge::Input;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| Input::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| input.solve_1());
    // let (solution_2, time_solving_2) = time(|| input.solve_2());

    println!("solution 1: {}", solution_1);
    // println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    // println!("took {:?} to solve 2", time_solving_2);
}

mod challenge {
    use std::{collections::VecDeque, fmt::Debug};

    #[derive(Clone)]
    pub struct Input {
        plane: Conway2D,
    }

    #[derive(Clone)]
    pub struct Buffer3D {
        buffer_1: Conway3D,
        buffer_2: Conway3D,
        current_buffer: bool,
    }

    pub type Conway3D = BetterVec<Conway2D>;
    pub type Conway2D = BetterVec<Conway1D>;
    pub type Conway1D = BetterVec<bool>;

    #[derive(Clone, Default)]
    pub struct BetterVec<T> {
        data: VecDeque<T>,
        start_idx: usize,
    }

    #[derive(Debug)]
    struct Dimensions3D {
        x: (isize, isize),
        y: (isize, isize),
        z: (isize, isize),
    }

    #[derive(Clone, Copy)]
    struct Position3D {
        x: isize,
        y: isize,
        z: isize,
    }

    #[derive(Debug)]
    struct Neighbours3D([bool; 3 * 3 * 3]);

    impl Input {
        pub fn parse(input: &str) -> Self {
            let plane = Conway2D::parse(input);
            Self {
                plane
            }
        }

        pub fn solve_1(&self) -> usize {
            let mut buffer = Buffer3D::create(self.plane.clone());
            buffer.solve();
            buffer.get_buffers().0.get_total_live_count()
        }
    }

    impl Buffer3D {
        pub fn create(plane: Conway2D) -> Self {
            let cubes = Conway3D::create(plane);

            Self {
                buffer_2: cubes.clone(),
                buffer_1: cubes,
                current_buffer: false,
            }
        }

        fn get_buffers(&mut self) -> (&mut Conway3D, &mut Conway3D) {
            if !self.current_buffer {
                (&mut self.buffer_1, &mut self.buffer_2)
            } else {
                (&mut self.buffer_2, &mut self.buffer_1)
            }
        }

        fn swap_buffers(&mut self) {
            self.current_buffer = !self.current_buffer;
        }

        fn solve(&mut self) {
            for _ in 0..6 {
                let (src, dst) = self.get_buffers();
                let dim = src.get_dimensions();

                for z in (dim.z.0 - 1)..(dim.z.1 + 1) {
                    for y in (dim.y.0 - 1)..(dim.y.1 + 1) {
                        for x in (dim.x.0 - 1)..(dim.x.1 + 1) {
                            let position = Position3D { x, y, z };
                            let neighbours = src.get_neighbours(position);

                            let active = neighbours.get_self();
                            let active_neighbours = neighbours.get_live_neighbours();
                            debug_assert_eq!(src.get_3d(position), active);

                            if active && !(active_neighbours == 2 || active_neighbours == 3) {
                                *dst.get_3d_mut(position) = false;
                            } else if !active && (active_neighbours == 3) {
                                *dst.get_3d_mut(position) = true;
                            } else if dst.get_3d(position) != active {
                                *dst.get_3d_mut(position) = active;
                            }
                        }
                    }
                }

                self.swap_buffers();
            }
        }
    }

    impl Conway3D {
        fn create(vec_2d: Conway2D) -> Self {
            let vec_3d = std::iter::once(vec_2d)
                .collect::<VecDeque<BetterVec<BetterVec<bool>>>>()
                .into();

            vec_3d
        }

        fn get_neighbours(&mut self, position: Position3D) -> Neighbours3D {
            let mut buffer = [false; 3 * 3 * 3];
            for z2 in -1..=1isize {
                for y2 in -1..=1isize {
                    for x2 in -1..=1isize {
                        let x = position.x + x2 as isize;
                        let y = position.y + y2 as isize;
                        let z = position.z + z2 as isize;
                        let position = Position3D { x, y, z };
                        let offset = (x2 + 1) + (y2 + 1) * 3 + (z2 + 1) * 3 * 3;
                        buffer[offset as usize] = self.get_3d(position);
                    }
                }
            }

            Neighbours3D(buffer)
        }

        fn get_3d(&self, position: Position3D) -> bool {
            *self
                .get(position.z)
                .and_then(|a| a.get(position.y).and_then(|a| a.get(position.x)))
                .unwrap_or(&false)
        }

        fn get_3d_mut(&mut self, position: Position3D) -> &mut bool {
            self.get_mut(position.z)
                .get_mut(position.y)
                .get_mut(position.x)
        }

        fn get_dimensions(&self) -> Dimensions3D {
            let mut dim = Dimensions3D::with_extremes();

            dim.z = self.get_min_max();

            for vec_2d in &self.data {
                let dim_y = vec_2d.get_min_max();
                dim.y.0 = isize::min(dim.y.0, dim_y.0);
                dim.y.1 = isize::max(dim.y.1, dim_y.1);

                for vec_1d in &vec_2d.data {
                    let dim_x = vec_1d.get_min_max();
                    dim.x.0 = isize::min(dim.x.0, dim_x.0);
                    dim.x.1 = isize::max(dim.x.1, dim_x.1);
                }
            }

            dim
        }

        fn get_total_live_count(&self) -> usize {
            self.data
                .iter()
                .flat_map(|y| y.data.iter().flat_map(|x| x.data.iter()))
                .filter(|&&b| b)
                .count()
        }
    }

    impl Debug for Conway3D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (i, z) in self.data.iter().enumerate() {
                f.write_fmt(format_args!("z={}\n", i))?;

                for y in &z.data {
                    for &x in &y.data {
                        f.write_str(if x { "#" } else { "." })?;
                    }
                    f.write_str("\n")?;
                }
            }

            Ok(())
        }
    }

    impl Conway2D {
        pub fn parse(input: &str) -> Self {
            input
                .split('\n')
                .map(|line| {
                    line.bytes()
                        .map(|b| match b {
                            b'.' => false,
                            b'#' => true,
                            _ => panic!("unknown character: {}", b),
                        })
                        .collect::<VecDeque<bool>>()
                        .into()
                })
                .collect::<VecDeque<BetterVec<bool>>>()
                .into()
        }
    }

    impl<T> BetterVec<T> {
        fn push_front(&mut self, value: T) {
            self.data.push_front(value);
            self.start_idx += 1;
        }

        fn push_back(&mut self, value: T) {
            self.data.push_back(value);
        }

        fn includes(&self, idx: isize) -> bool {
            if idx >= 0 {
                (idx as usize) < (self.data.len() - self.start_idx)
            } else {
                idx >= -(self.start_idx as isize)
            }
        }
    }

    impl<T> BetterVec<T>
    where
        T: Default,
    {
        fn grow_to_include(&mut self, idx: isize) {
            while !self.includes(idx) {
                if idx >= 0 {
                    self.push_back(T::default());
                } else {
                    self.push_front(T::default());
                }
            }
        }

        fn get(&self, index: isize) -> Option<&T> {
            if !self.includes(index) {
                return None;
            }
            Some(&self.data[(index + self.start_idx as isize) as usize])
        }

        fn get_mut(&mut self, index: isize) -> &mut T {
            self.grow_to_include(index);
            &mut self.data[(index + self.start_idx as isize) as usize]
        }

        fn get_min_max(&self) -> (isize, isize) {
            (
                -(self.start_idx as isize),
                (self.data.len() + self.start_idx) as isize,
            )
        }
    }

    impl<T> From<VecDeque<T>> for BetterVec<T> {
        fn from(data: VecDeque<T>) -> BetterVec<T> {
            BetterVec { data, start_idx: 0 }
        }
    }

    impl Dimensions3D {
        fn with_extremes() -> Self {
            Self {
                x: (isize::MAX, isize::MIN),
                y: (isize::MAX, isize::MIN),
                z: (isize::MAX, isize::MIN),
            }
        }
    }

    impl Neighbours3D {
        const CENTER: usize = 1 + 1 * 3 + 1 * 3 * 3;
        fn get_self(&self) -> bool {
            self.0[Self::CENTER]
        }

        fn get_live_neighbours(&self) -> usize {
            self.0
                .iter()
                .enumerate()
                .map(|(i, &b)| i != Self::CENTER && b)
                .filter(|&b| b)
                .count()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::Input;

    const TEST_INPUT: &str = ".#.\n..#\n###";

    #[test]
    fn test_1() {
        let parsed = Input::parse(TEST_INPUT);
        assert_eq!(112, parsed.solve_1());
    }
}
