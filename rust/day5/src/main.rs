use std::convert::TryInto;

std::include!("../../helpers.rs");

// NOTE: could be made cleaner without overloads RFC2000 gets implementerd
// https://rust-lang.github.io/rfcs/2000-const-generics.html
fn decode_partition_10(data: &[u8; 10]) -> usize {
    data.iter()
        .enumerate()
        .map(|(i, c)| match c {
            b'B' | b'R' => 1 << (10 - 1 - i),
            b'F' | b'L' => 0,
            _ => panic!("Unknown char {}", c),
        })
        .sum()
}

fn solve_part_1(data: &[bool; KEYSPACE]) -> usize {
    data.iter()
        .enumerate()
        .rev()
        .filter(|(_, b)| **b)
        .next()
        .unwrap()
        .0
}

fn solve_part_2(data: &[bool; KEYSPACE]) -> usize {
    data.iter()
        .enumerate()
        // first part are all false
        .skip_while(|(_, b)| !**b)
        // find first false
        .skip_while(|(_, b)| **b)
        .next()
        .unwrap()
        .0
}

const KEYSPACE: usize = 2 << (10 - 1);
fn main() {
    let (input_data, time_read) = time(|| read_stdin());
    let (data, time_parse) = time(|| parse_input(split(input_data)));
    let (solution_1, time_1) = time(|| solve_part_1(&data));
    let (solution_2, time_2) = time(|| solve_part_2(&data));

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);
    println!("Time to read stdin: {:?}", time_read);
    println!("Time to parse input: {:?}", time_parse);
    println!("Time to solve 1: {:?}", time_1);
    println!("Time to solve 2: {:?}", time_2);
}

fn split(str: String) -> Vec<[u8; 10]> {
    str.lines()
        .map(|line| line.as_bytes().try_into().unwrap())
        .collect::<Vec<[u8; 10]>>()
}

fn parse_input(input_data: Vec<[u8; 10]>) -> [bool; KEYSPACE] {
    let mut data = [false; KEYSPACE];
    for f in input_data {
        let decoded = decode_partition_10(&f);
        data[decoded] = true;
    }
    data
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_binary_partition() {
        assert_eq!(357, decode_partition_10(b"FBFBBFFRLR"));
        assert_eq!(567, decode_partition_10(b"BFFFBBFRRR"));
        assert_eq!(119, decode_partition_10(b"FFFBBBFRRR"));
        assert_eq!(820, decode_partition_10(b"BBFFBBFRLL"));
    }
}
