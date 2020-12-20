use challenge::Input;

include!("../../helpers.rs");

fn main() {
    let (stdin, time_reading) = time(|| read_stdin());
    let (input, time_parsing) = time(|| Input::parse(&stdin));
    let (solution_1, time_solving_1) = time(|| input.solve_1());
    let (solution_2, time_solving_2) = time(|| input.solve_2());

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);
    println!("took {:?} to read stdin", time_reading);
    println!("took {:?} to read input", time_parsing);
    println!("took {:?} to solve 1", time_solving_1);
    println!("took {:?} to solve 2", time_solving_2);
}

mod challenge {
    use std::{
        collections::VecDeque,
        fmt::Debug,
        marker::PhantomData,
        ops::{Add, Index, IndexMut},
    };

    use impl_3d::*;
    use impl_4d::*;

    #[derive(Clone)]
    pub struct Input {
        plane: Conway2D,
    }

    pub trait Position: Add<Self> + Copy + Clone + Sized {
        fn iterate_neighbour_positions<F>(&self, fun: F)
        where
            F: FnMut(Self);
    }

    pub trait Neighbours<TPosition>: Default {
        fn set(&mut self, index: usize, value: bool);
        // PERF: would declare these const, but const trait functions are not stabilized yet
        fn get_buffer(&self) -> &[bool];
        fn get_center_index() -> usize;
        fn to_index(position: TPosition) -> usize;

        fn get_self(&self) -> bool {
            self.get_buffer()[Self::get_center_index()]
        }

        fn get_live_neighbours(&self) -> usize {
            self.get_buffer()
                .iter()
                .enumerate()
                .map(|(i, &b)| i != Self::get_center_index() && b)
                .filter(|&b| b)
                .count()
        }
    }

    pub trait Dimensions<TPosition>
    where
        TPosition: Position,
    {
        fn with_extremes() -> Self;

        fn iterate_all_positions<F>(&self, fun: F)
        where
            F: FnMut(TPosition);
    }

    pub trait ConwayField<TPosition, TNeighbours, TDimensions>:
        Index<TPosition, Output = bool> + IndexMut<TPosition> + Clone
    where
        TPosition: Position + Add<Output = TPosition>, // TODO: is this Add really needed?
        TNeighbours: Neighbours<TPosition>,
        TDimensions: Dimensions<TPosition>,
    {
        fn create(vec_2d: Conway2D) -> Self;
        fn get_dimensions(&self) -> TDimensions;

        fn get_total_live_count(&self) -> usize {
            let mut count = 0;
            let dim = self.get_dimensions();
            dim.iterate_all_positions(|pos| {
                if self[pos] {
                    count += 1;
                }
            });
            count
        }

        fn get_neighbours(&mut self, position: TPosition) -> TNeighbours {
            let mut buffer = TNeighbours::default();

            position.iterate_neighbour_positions(|offset| {
                let position = position + offset;
                let index = TNeighbours::to_index(offset);
                buffer.set(index, self[position]);
            });

            buffer
        }
    }

    #[derive(Clone)]
    pub struct Buffer<TPosition, TConway, TNeighbours, TDimensions>
    where
        TPosition: Position + Add<Output = TPosition>,
        TConway: ConwayField<TPosition, TNeighbours, TDimensions>,
        TNeighbours: Neighbours<TPosition>,
        TDimensions: Dimensions<TPosition>,
    {
        buffer_1: TConway,
        buffer_2: TConway,
        current_buffer: bool,

        phantom_position: PhantomData<TPosition>,
        phantom_neighbours: PhantomData<TNeighbours>,
        phantom_dimensions: PhantomData<TDimensions>,
    }

    pub type Conway4D = BetterVec<Conway3D>;
    pub type Conway3D = BetterVec<Conway2D>;
    pub type Conway2D = BetterVec<Conway1D>;
    pub type Conway1D = BetterVec<bool>;

    #[derive(Clone, Default)]
    pub struct BetterVec<T> {
        data: VecDeque<T>,
        start_idx: usize,
    }

    impl Input {
        pub fn parse(input: &str) -> Self {
            let plane = Conway2D::parse(input);
            Self { plane }
        }

        pub fn solve_1(&self) -> usize {
            let mut buffer = Buffer::<Position3D, Conway3D, Neighbours3D, Dimensions3D>::create(
                self.plane.clone(),
            );
            buffer.solve();
            buffer.get_buffers().0.get_total_live_count()
        }

        pub fn solve_2(&self) -> usize {
            let mut buffer = Buffer::<Position4D, Conway4D, Neighbours4D, Dimensions4D>::create(
                self.plane.clone(),
            );
            buffer.solve();
            buffer.get_buffers().0.get_total_live_count()
        }
    }

    impl<
            TPosition: Position + Add<Output = TPosition>,
            TConway: ConwayField<TPosition, TNeighbours, TDimensions>,
            TNeighbours: Neighbours<TPosition>,
            TDimensions: Dimensions<TPosition>,
        > Buffer<TPosition, TConway, TNeighbours, TDimensions>
    {
        pub fn create(plane: Conway2D) -> Self {
            let cubes = TConway::create(plane);

            Self {
                buffer_2: cubes.clone(),
                buffer_1: cubes,
                current_buffer: false,
                phantom_position: PhantomData,
                phantom_neighbours: PhantomData,
                phantom_dimensions: PhantomData,
            }
        }

        fn get_buffers(&mut self) -> (&mut TConway, &mut TConway) {
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

                dim.iterate_all_positions(|position| {
                    let neighbours = src.get_neighbours(position);

                    let active = neighbours.get_self();
                    let active_neighbours = neighbours.get_live_neighbours();
                    debug_assert_eq!(src[position], active);

                    if active && !(active_neighbours == 2 || active_neighbours == 3) {
                        dst[position] = false;
                    } else if !active && (active_neighbours == 3) {
                        dst[position] = true;
                    } else if dst[position] != active {
                        dst[position] = active;
                    }
                });

                self.swap_buffers();
            }
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

    mod impl_3d {
        use std::{
            collections::VecDeque,
            ops::{Add, Index, IndexMut},
        };

        use super::{Conway2D, Conway3D, ConwayField, Dimensions, Neighbours, Position};

        #[derive(Debug, Clone, Copy)]
        pub struct Position3D {
            pub x: isize,
            pub y: isize,
            pub z: isize,
        }

        #[derive(Debug)]
        pub struct Dimensions3D {
            pub x: (isize, isize),
            pub y: (isize, isize),
            pub z: (isize, isize),
        }

        #[derive(Debug)]
        pub struct Neighbours3D(pub [bool; 3 * 3 * 3]);

        impl Position for Position3D {
            fn iterate_neighbour_positions<F>(&self, mut fun: F)
            where
                F: FnMut(Position3D),
            {
                for z in -1..=1isize {
                    for y in -1..=1isize {
                        for x in -1..=1isize {
                            fun(Position3D { x, y, z });
                        }
                    }
                }
            }
        }

        impl Add<Self> for Position3D {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        impl Index<Position3D> for Conway3D {
            type Output = bool;

            fn index(&self, index: Position3D) -> &Self::Output {
                self.get(index.z)
                    .and_then(|a| a.get(index.y).and_then(|a| a.get(index.x)))
                    .unwrap_or(&false)
            }
        }

        impl IndexMut<Position3D> for Conway3D {
            fn index_mut(&mut self, index: Position3D) -> &mut Self::Output {
                self.get_mut(index.z).get_mut(index.y).get_mut(index.x)
            }
        }

        impl Dimensions<Position3D> for Dimensions3D {
            fn with_extremes() -> Self {
                Self {
                    x: (isize::MAX, isize::MIN),
                    y: (isize::MAX, isize::MIN),
                    z: (isize::MAX, isize::MIN),
                }
            }

            fn iterate_all_positions<F>(&self, mut fun: F)
            where
                F: FnMut(Position3D),
            {
                for z in (self.z.0 - 1)..(self.z.1 + 1) {
                    for y in (self.y.0 - 1)..(self.y.1 + 1) {
                        for x in (self.x.0 - 1)..(self.x.1 + 1) {
                            fun(Position3D { x, y, z });
                        }
                    }
                }
            }
        }

        impl Neighbours<Position3D> for Neighbours3D {
            fn get_buffer(&self) -> &[bool] {
                &self.0
            }

            fn get_center_index() -> usize {
                1 + 1 * 3 + 1 * 3 * 3
            }

            fn to_index(position: Position3D) -> usize {
                ((position.x + 1) + (position.y + 1) * 3 + (position.z + 1) * 3 * 3) as usize
            }

            fn set(&mut self, index: usize, value: bool) {
                self.0[index] = value;
            }
        }

        impl Default for Neighbours3D {
            fn default() -> Self {
                Self([false; 3 * 3 * 3])
            }
        }

        impl ConwayField<Position3D, Neighbours3D, Dimensions3D> for Conway3D {
            fn create(vec_2d: Conway2D) -> Self {
                let vec_3d = std::iter::once(vec_2d)
                    .collect::<VecDeque<Conway2D>>()
                    .into();

                vec_3d
            }

            // TODO: change to use a function that iterates over all positions and takes highest? -> less code reuse
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
        }
    }

    mod impl_4d {
        use std::{
            collections::VecDeque,
            ops::{Add, Index, IndexMut},
        };

        use super::{Conway2D, Conway3D, Conway4D, ConwayField, Dimensions, Neighbours, Position};

        #[derive(Debug, Clone, Copy)]
        pub struct Position4D {
            pub x: isize,
            pub y: isize,
            pub z: isize,
            pub w: isize,
        }

        #[derive(Debug)]
        pub struct Dimensions4D {
            pub x: (isize, isize),
            pub y: (isize, isize),
            pub z: (isize, isize),
            pub w: (isize, isize),
        }

        #[derive(Debug)]
        pub struct Neighbours4D(pub [bool; 3 * 3 * 3 * 3]);

        impl Position for Position4D {
            fn iterate_neighbour_positions<F>(&self, mut fun: F)
            where
                F: FnMut(Position4D),
            {
                for w in -1..=1isize {
                    for z in -1..=1isize {
                        for y in -1..=1isize {
                            for x in -1..=1isize {
                                fun(Position4D { x, y, z, w });
                            }
                        }
                    }
                }
            }
        }

        impl Add<Self> for Position4D {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                    w: self.w + rhs.w,
                }
            }
        }

        impl Index<Position4D> for Conway4D {
            type Output = bool;

            fn index(&self, index: Position4D) -> &Self::Output {
                self.get(index.w)
                    .and_then(|a| {
                        a.get(index.z)
                            .and_then(|a| a.get(index.y).and_then(|a| a.get(index.x)))
                    })
                    .unwrap_or(&false)
            }
        }

        impl IndexMut<Position4D> for Conway4D {
            fn index_mut(&mut self, index: Position4D) -> &mut Self::Output {
                self.get_mut(index.w)
                    .get_mut(index.z)
                    .get_mut(index.y)
                    .get_mut(index.x)
            }
        }

        impl Dimensions<Position4D> for Dimensions4D {
            fn with_extremes() -> Self {
                Self {
                    x: (isize::MAX, isize::MIN),
                    y: (isize::MAX, isize::MIN),
                    z: (isize::MAX, isize::MIN),
                    w: (isize::MAX, isize::MIN),
                }
            }

            fn iterate_all_positions<F>(&self, mut fun: F)
            where
                F: FnMut(Position4D),
            {
                for w in (self.w.0 - 1)..(self.w.1 + 1) {
                    for z in (self.z.0 - 1)..(self.z.1 + 1) {
                        for y in (self.y.0 - 1)..(self.y.1 + 1) {
                            for x in (self.x.0 - 1)..(self.x.1 + 1) {
                                fun(Position4D { x, y, z, w });
                            }
                        }
                    }
                }
            }
        }

        impl Neighbours<Position4D> for Neighbours4D {
            fn get_buffer(&self) -> &[bool] {
                &self.0
            }

            fn get_center_index() -> usize {
                1 + 1 * 3 + 1 * 3 * 3 + 1 * 3 * 3 * 3
            }

            fn to_index(position: Position4D) -> usize {
                ((position.x + 1)
                    + (position.y + 1) * 3
                    + (position.z + 1) * 3 * 3
                    + (position.w + 1) * 3 * 3 * 3) as usize
            }

            fn set(&mut self, index: usize, value: bool) {
                self.0[index] = value;
            }
        }

        impl Default for Neighbours4D {
            fn default() -> Self {
                Self([false; 3 * 3 * 3 * 3])
            }
        }

        impl ConwayField<Position4D, Neighbours4D, Dimensions4D> for Conway4D {
            fn create(vec_2d: Conway2D) -> Self {
                let vec_3d = std::iter::once(vec_2d)
                    .collect::<VecDeque<Conway2D>>()
                    .into();

                let vec_4d = std::iter::once(vec_3d)
                    .collect::<VecDeque<Conway3D>>()
                    .into();
                vec_4d
            }

            // TODO: change to use a function that iterates over all positions and takes highest? -> less code reuse
            fn get_dimensions(&self) -> Dimensions4D {
                let mut dim = Dimensions4D::with_extremes();

                dim.w = self.get_min_max();

                for vec_3d in &self.data {
                    let dim_z = vec_3d.get_min_max();
                    dim.z.0 = isize::min(dim.z.0, dim_z.0);
                    dim.z.1 = isize::max(dim.z.1, dim_z.1);

                    for vec_2d in &vec_3d.data {
                        let dim_y = vec_2d.get_min_max();
                        dim.y.0 = isize::min(dim.y.0, dim_y.0);
                        dim.y.1 = isize::max(dim.y.1, dim_y.1);

                        for vec_1d in &vec_2d.data {
                            let dim_x = vec_1d.get_min_max();
                            dim.x.0 = isize::min(dim.x.0, dim_x.0);
                            dim.x.1 = isize::max(dim.x.1, dim_x.1);
                        }
                    }
                }

                dim
            }
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

    #[test]
    fn test_2() {
        let parsed = Input::parse(TEST_INPUT);
        assert_eq!(848, parsed.solve_2());
    }
}
