use challenge::Input;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| Input::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| input.clone().solve_1());
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
        buffer_1: Conway3D,
        buffer_2: Conway3D,
        current_buffer: bool,
    }

    type Conway3D = BetterVec<Conway2D>;
    type Conway2D = BetterVec<Conway1D>;
    type Conway1D = BetterVec<bool>;

    #[derive(Clone, Default)]
    struct BetterVec<T> {
        data: VecDeque<T>,
        start_idx: usize,
    }

    #[derive(Debug)]
    struct Dimensions {
        x: (isize, isize),
        y: (isize, isize),
        z: (isize, isize),
    }

    #[derive(Debug)]
    struct Neighbours([bool; 3 * 3 * 3]);

    impl Input {
        pub fn parse(input: &str) -> Self {
            let cubes = Conway3D::parse(input);

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

        pub fn solve_1(&mut self) -> usize {
            for _ in 0..6 {
                let (src, dst) = self.get_buffers();
                let dim = src.get_dimensions();

                for z in (dim.z.0 - 1)..(dim.z.1 + 1) {
                    for y in (dim.y.0 - 1)..(dim.y.1 + 1) {
                        for x in (dim.x.0 - 1)..(dim.x.1 + 1) {
                            let neighbours = src.get_neighbours(x, y, z);

                            let active = neighbours.get_self();
                            let active_neighbours = neighbours.get_live_neighbours();
                            debug_assert_eq!(src.get_3d(x, y, z), active);

                            if active && !(active_neighbours == 2 || active_neighbours == 3) {
                                *dst.get_3d_mut(x, y, z) = false;
                            } else if !active && (active_neighbours == 3) {
                                *dst.get_3d_mut(x, y, z) = true;
                            } else if dst.get_3d(x, y, z) != active {
                                *dst.get_3d_mut(x, y, z) = active;
                            }
                        }
                    }
                }

                self.swap_buffers();
            }

            self.get_buffers().0.get_total_live_count()
        }
    }

    impl Conway3D {
        fn parse(input: &str) -> Self {
            let vec_2d = input
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
                .into();

            let vec_3d = std::iter::once(vec_2d)
                .collect::<VecDeque<BetterVec<BetterVec<bool>>>>()
                .into();

            vec_3d
        }

        fn get_neighbours(&mut self, x: isize, y: isize, z: isize) -> Neighbours {
            let mut buffer = [false; 3 * 3 * 3];
            for z2 in -1..=1isize {
                for y2 in -1..=1isize {
                    for x2 in -1..=1isize {
                        let x = x + x2 as isize;
                        let y = y + y2 as isize;
                        let z = z + z2 as isize;
                        let offset = (x2 + 1) + (y2 + 1) * 3 + (z2 + 1) * 3 * 3;
                        buffer[offset as usize] = self.get_3d(x, y, z);
                    }
                }
            }

            Neighbours(buffer)
        }

        fn get_3d(&self, x: isize, y: isize, z: isize) -> bool {
            *self
                .get(z)
                .and_then(|a| a.get(y).and_then(|a| a.get(x)))
                .unwrap_or(&false)
        }

        fn get_3d_mut(&mut self, x: isize, y: isize, z: isize) -> &mut bool {
            self.get_mut(z).get_mut(y).get_mut(x)
        }

        fn get_dimensions(&self) -> Dimensions {
            let mut dim = Dimensions::with_extremes();

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

    impl Dimensions {
        fn with_extremes() -> Self {
            Self {
                x: (isize::MAX, isize::MIN),
                y: (isize::MAX, isize::MIN),
                z: (isize::MAX, isize::MIN),
            }
        }
    }

    impl Neighbours {
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
        assert_eq!(112, parsed.clone().solve_1());
    }
}
