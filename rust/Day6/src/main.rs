include!("../../helpers.rs");
struct GroupData(u32, u32);

impl Default for GroupData {
    fn default() -> Self {
        GroupData(0, (1 << 27) - 1)
    }
}

impl GroupData {
    pub fn register_answers(&mut self, chars: impl Iterator<Item = u8>) {
        let mut answers = 0;
        for c in chars {
            let bit = c - b'a';
            answers |= 1 << bit;
        }

        self.0 |= answers;
        self.1 &= answers;
    }

    pub fn get_answer_count_1(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn get_answer_count_2(&self) -> u32 {
        self.1.count_ones()
    }
}

fn main() {
    let (input_string, time_stdin) = time(|| read_stdin());
    let (input, time_parsing) = time(|| parse_input(&input_string) as Vec<GroupData>);
    let (solution_1, time_solve_1): (u32, std::time::Duration) = time(|| input.iter().map(|x| x.get_answer_count_1()).sum());
    let (solution_2, time_solve_2): (u32, std::time::Duration) = time(|| input.iter().map(|x| x.get_answer_count_2()).sum());

    println!("solution 1: {:?}", solution_1);
    println!("solution 2: {:?}", solution_2);
    println!("took {:?} to read input", time_stdin);
    println!("took {:?} to parse input", time_parsing);
    println!("took {:?} to solve part 1", time_solve_1);
    println!("took {:?} to solve part 2", time_solve_2);
}

fn parse_input(input: &str) -> Vec<GroupData> {
    let mut vec = Vec::new();
    for lines in input.split("\n\n") {
        let mut data = GroupData::default();
        for line in lines.split('\n').filter(|l| *l != "") {
            data.register_answers(line.bytes());
        }
        vec.push(data);
    }

    vec
}
