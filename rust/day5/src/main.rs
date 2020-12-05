use std::{convert::TryInto, io::Read};

fn decode_partition_10(data: &[u8; 10]) -> usize {
    data.iter().enumerate().map(|(i, c)| match c {
        b'B' | b'R' => 1 << (10 - 1 - i),
        b'F' | b'L' => 0,
        _ => panic!("Unknown char {}", c),
    }).sum()
}

#[cfg(test)]
fn decode_partition_7(data: &[u8; 7]) -> usize {
    data.iter().enumerate().map(|(i, c)| match c {
        b'B' => 1 << (7 - 1 - i),
        b'F' => 0,
        _ => panic!("Unknown char {}", c),
    }).sum()
}

#[cfg(test)]
fn decode_partition_3(data: &[u8; 3]) -> usize {
    data.iter().enumerate().map(|(i, c)| match c {
        b'R' => 1 << (3 - 1 - i),
        b'L' => 0,
        _ => panic!("Unknown char {}", c),
    }).sum()
}

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).unwrap();
    let x = string.lines().map(|line| line.as_bytes().try_into().unwrap()).collect::<Vec<[u8; 10]>>();

    const KEYSPACE: usize = 2 << (10-1);
    let mut data = [false; KEYSPACE];
    for f in x {
        let decoded = decode_partition_10(&f);
        data[decoded] = true;
    }
    let max = (&data).iter().enumerate().rev().filter(|(_, b)| **b).next().unwrap().0;

    println!("max: {}", max);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_binary_partition() {
        assert_eq!(127, decode_partition_7(b"BBBBBBB"));
        assert_eq!(0, decode_partition_7(b"FFFFFFF"));
        assert_eq!(44, decode_partition_7(b"FBFBBFF"));
        assert_eq!(70, decode_partition_7(b"BFFFBBF"));
        assert_eq!(14, decode_partition_7(b"FFFBBBF"));
        assert_eq!(102, decode_partition_7(b"BBFFBBF"));

        assert_eq!(7, decode_partition_3(b"RRR"));
        assert_eq!(0, decode_partition_3(b"LLL"));
        assert_eq!(4, decode_partition_3(b"RLL"));

        assert_eq!(357, decode_partition_10(b"FBFBBFFRLR"));
        assert_eq!(567, decode_partition_10(b"BFFFBBFRRR"));
        assert_eq!(119, decode_partition_10(b"FFFBBBFRRR"));
        assert_eq!(820, decode_partition_10(b"BBFFBBFRLL"));
    }
}