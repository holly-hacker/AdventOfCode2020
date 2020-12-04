use std::io::{stdin, Read};
use std::time::Instant;

#[derive(Debug, Default, Clone)]
struct Passport<'a> {
    pub byr: Option<&'a str>,
    pub iyr: Option<&'a str>,
    pub eyr: Option<&'a str>,
    pub hgt: Option<&'a str>,
    pub hcl: Option<&'a str>,
    pub ecl: Option<&'a str>,
    pub pid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    pub fn parse_line(mut self, line: &'a str) -> Self {
        for data in line.split(' ') {
            match &data[0..3] {
                "byr" => self.byr = Some(&data[4..]),
                "iyr" => self.iyr = Some(&data[4..]),
                "eyr" => self.eyr = Some(&data[4..]),
                "hgt" => self.hgt = Some(&data[4..]),
                "hcl" => self.hcl = Some(&data[4..]),
                "ecl" => self.ecl = Some(&data[4..]),
                "pid" => self.pid = Some(&data[4..]),
                "cid" => (),
                unk => panic!("Unknown data type: {}", unk),
            }
        }

        self
    }

    pub fn has_data(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid(&self) -> bool {
        if !self.has_data() {
            return false;
        }

        let byr = self.byr.unwrap().parse::<usize>().unwrap();
        if byr < 1920 || byr > 2002 {
            return false;
        }

        let iyr = self.iyr.unwrap().parse::<usize>().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        let eyr = self.eyr.unwrap().parse::<usize>().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        let hgt = self.hgt.unwrap();
        if hgt.len() <= 3 {
            return false;
        }

        let hgt = (
            &hgt[..hgt.len() - 2].parse::<usize>().unwrap(),
            &hgt[hgt.len() - 2..],
        );
        if !match hgt.1 {
            "cm" => *hgt.0 >= 150 && *hgt.0 <= 193,
            "in" => *hgt.0 >= 59 && *hgt.0 <= 76,
            _ => false,
        } {
            return false;
        }

        let hcl = self.hcl.unwrap();
        if hcl.len() != 7 || hcl.bytes().nth(0).unwrap() != '#' as u8 || !hcl[1..].bytes().all(|b| b.is_ascii_hexdigit()) {
            return false;
        }

        let ecl = self.ecl.unwrap();

        if !match ecl {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        } {
            return false;
        }

        let pid = self.pid.unwrap();
        if pid.len() != 9 || !pid.bytes().all(|c| c.is_ascii_digit()) {
            return false;
        }

        return true;
    }
}

fn parse_string(data: &str) -> Vec<Passport> {
    let mut vec = Vec::new();
    for lines in data.split("\n\n") {
        let mut flags: Passport = Passport::default();
        for line in lines.split('\n').filter(|l| l.len() > 0) {
            flags = flags.parse_line(line);
        }
        vec.push(flags);
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
        let count_correct = data.iter().filter(|x| x.has_data()).count();
        println!("took {:?} to solve 1", time_solving.elapsed());
        println!("solution 1: {}", count_correct);

        let time_solving = Instant::now();
        let count_correct = data.iter().filter(|x| x.is_valid()).count();
        println!("took {:?} to solve 2", time_solving.elapsed());
        println!("solution 2: {}", count_correct);
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
        assert_eq!(2, data.iter().filter(|x| x.has_data()).count());
        assert!(true);
    }

    #[test]
    fn test_2_valid() {
        let str = include_str!("../test_input2_valid.txt");
        let data = parse_string(str);
        assert_eq!(data.len(), data.iter().filter(|x| x.is_valid()).count());
        assert!(true);
    }

    #[test]
    fn test_2_invalid() {
        let str = include_str!("../test_input2_invalid.txt");
        let data = parse_string(str);

        assert_eq!(data.len(), data.iter().filter(|x| !x.is_valid()).count());
        assert!(true);
    }

    #[test]
    fn real_1() {
        let str = include_str!("../input.txt");
        let data = parse_string(str);

        assert_eq!(235, data.iter().filter(|x| x.has_data()).count());
        assert!(true);
    }

    #[test]
    fn real_2() {
        let str = include_str!("../input.txt");
        let data = parse_string(str);

        assert_eq!(194, data.iter().filter(|x| x.is_valid()).count());
        assert!(true);
    }
}
