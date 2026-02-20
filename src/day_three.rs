use crate::common::read_file_multiple_lines;
use std::fmt::{Display, Formatter};

pub fn get_banks() {
    let lines: Vec<String> = read_file_multiple_lines("resources/day3Input.txt").unwrap();
    let banks: Vec<Bank> = lines.iter().map(Bank::from_line).collect();
    let mut total_voltage: u64 = 0;
    for bank in banks {
        let new_total_voltage = bank.get_max_voltage();
        total_voltage = total_voltage + new_total_voltage;
    }
    println!("day three answer = {}", total_voltage);
}

struct Index {
    value: u8,
    index: usize,
}

#[derive(Debug)]
struct Bank {
    batteries: Vec<u8>,
}

impl Display for Bank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "batteries: {:?}", self.batteries)
    }
}

impl Bank {
    fn from_line(line: &String) -> Bank {
        let batteries = line.chars().map(Self::char_to_u8).collect();
        Bank { batteries }
    }

    fn char_to_u8(c: char) -> u8 {
        c.to_digit(10).map(|u| u as u8).unwrap()
    }

    fn get_max_voltage(&self) -> u64 {
        let mut indexes: Vec<Index> = Vec::new();
        for i in 0..12 {
            match indexes.last() {
                None => {
                    let first_index = self.get_index_of_max_digit(0, &self.batteries.len() - 11);
                    indexes.push(first_index);
                }
                Some(last_index) => {
                    let next_index = self.get_index_of_max_digit(
                        last_index.index + 1,
                        self.batteries.len() - 11 + i,
                    );
                    indexes.push(next_index);
                }
            }
        }
        let mut total_max_voltage: u64 = 0;
        for i in 0..12 {
            if let Some(index) = indexes.get(i) {
                total_max_voltage += (index.value as u64) * 10_u64.pow((11 - i) as u32);
            }
        }
        total_max_voltage
    }

    fn get_index_of_max_digit(
        &self,
        customs_starting_point: usize,
        number_of_excluded_trailing_digits: usize,
    ) -> Index {
        let mut max_so_far: u8 = 0;
        let mut index_of_max_so_far: usize = 0;
        for i in customs_starting_point..number_of_excluded_trailing_digits {
            if let Some(value) = self.batteries.get(i)
                && value > &max_so_far
            {
                max_so_far = *value;
                index_of_max_so_far = i;
            }
        }
        Index {
            value: max_so_far,
            index: index_of_max_so_far,
        }
    }
}
