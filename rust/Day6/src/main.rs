include!("../../helpers.rs");

#[derive(Default)]
struct GroupData(u32);

impl GroupData {
    pub fn set_bit(&mut self, bit: u8) {
        self.0 |= 1 << bit;
    }

    pub fn get_answer_count(&self) -> u32 {
        self.0.count_ones()
    }
}

fn main() {
    let (input_string, time_stdin) = time(|| read_stdin());
    let (input, time_parsing) = time(|| parse_input(&input_string) as Vec<GroupData>);
    let (solution_1, time_solve_1): (u32, std::time::Duration) = time(|| input.iter().map(|x| x.get_answer_count()).sum());

    println!("solution 1: {:?}", solution_1);
}

fn parse_input(input: &str) -> Vec<GroupData> {
    let mut vec = Vec::new();
    for lines in input.split("\n\n") {
        let mut data = GroupData::default();
        for line in lines.split('\n') {
            for c in line.bytes() {
                data.set_bit(c - b'a');
            }
        }
        vec.push(data);
    }

    vec
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        //
    }
}
