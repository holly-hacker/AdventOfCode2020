use std::time::Instant;
use std::io::{Read, BufRead, stdin};

#[derive(Debug, Default)]
struct DataFlags {
    pub has_byr: bool,
    pub has_iyr: bool,
    pub has_eyr: bool,
    pub has_hgt: bool,
    pub has_hcl: bool,
    pub has_ecl: bool,
    pub has_pid: bool,
}

impl DataFlags {
    pub fn parse_line(&mut self, line: &str) {
        for data in line.split(' ') {
            match &data[0..3] {
                "byr" => self.has_byr = true,
                "iyr" => self.has_iyr = true,
                "eyr" => self.has_eyr = true,
                "hgt" => self.has_hgt = true,
                "hcl" => self.has_hcl = true,
                "ecl" => self.has_ecl = true,
                "pid" => self.has_pid = true,
                "cid" => (),
                unk => panic!("Unknown data type: {}", unk),
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        self.has_byr
            && self.has_iyr
            && self.has_eyr
            && self.has_hgt
            && self.has_hcl
            && self.has_ecl
            && self.has_pid
    }
}

fn parse_string(data: &str) -> Vec<DataFlags> {
    let mut vec = Vec::new();

    let mut flags = DataFlags::default();
    for line in data.split('\n') {
        if line == "" {
            vec.push(flags);
            flags = DataFlags::default();
        } else {
            flags.parse_line(line);
        }
    }

    vec
}

fn main() {
    let time_total = Instant::now();

    {
        let time_reading = Instant::now();
        let mut str = String::new();
        stdin().lock().read_to_string(&mut str).unwrap();
        let data = parse_string(&str);
        println!("took {:?} to read input", time_reading.elapsed());

        let time_solving = Instant::now();
        let count_correct = data.iter().filter(|x| x.is_valid()).count();
        println!("took {:?} to solve 1", time_solving.elapsed());
        println!("solution 1: {}", count_correct);
    }

    println!("took {:?} in total", time_total.elapsed());
}

#[cfg(test)]
mod test {
    use crate::parse_string;

    #[test]
    fn test_1() {
        let str = include_str!("../test_input.txt");
        let data = parse_string(str);
        assert_eq!(4, data.len());
        assert_eq!(2, data.iter().filter(|x| x.is_valid()).count());
        assert!(true);
    }
}
