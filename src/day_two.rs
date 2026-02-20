use crate::common::read_file_one_line;

pub fn get_invalids_in_range() {
    let file_content = read_file_one_line("resources/day2Input.txt").unwrap();
    let range_strings = file_content.split(',');
    let ranges: Vec<IdRange> = range_strings.map(IdRange::from_line).collect();
    let mut sum: u64 = 0;
    for range in ranges {
        for id in range.ids {
            if id.is_invalid() {
                sum += id.value;
            }
        }
    }
    println!("the sum is {}", sum)
}

struct IdRange {
    ids: Box<dyn Iterator<Item = Id>>,
}

impl IdRange {
    fn from_line(line: &str) -> IdRange {
        let mut a = line.split('-');
        let part_one = a.next().unwrap().parse::<u64>().unwrap();
        let part_two = a.next().unwrap().parse::<u64>().unwrap();
        let ids = Box::new((part_one..=part_two).map(|i: u64| Id { value: i }));
        IdRange { ids }
    }
}

pub struct Id {
    pub value: u64,
}

impl Id {
    pub fn is_invalid(&self) -> bool {
        let self_as_string = self.value.to_string();
        let length = self_as_string.len();
        let divisors = get_divisors(length);
        divisors
            .iter()
            .any(|divisor| repeats(&self_as_string, &self.value.to_string()[0..*divisor]))
    }
}

fn repeats(string: &str, sub_string: &str) -> bool {
    let result = sub_string.repeat(string.len() / sub_string.len()) == string;
    result
}

pub fn get_divisors(i: usize) -> Vec<usize> {
    let half = i / 2;
    let mut vec = Vec::new();
    for j in 1..=half {
        if i % j == 0 {
            vec.push(j)
        }
    }
    vec
}
