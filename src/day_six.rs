use crate::common::read_file_multiple_lines;
use std::fmt::{Display, Formatter};

pub fn get_total_sum() {
    let input = read_file_multiple_lines("resources/day6Input.txt").unwrap();
    let functions = Function::from_input(&input);
    let mut total_sum = 0;
    for f in functions {
        total_sum += f.apply();
    }
    println!("day siz answer = {}", total_sum);
}

enum Operation {
    Addition,
    Multiplication,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Operation::Addition => "add",
            Operation::Multiplication => "times",
        };
        write!(f, "{}", result)
    }
}

impl Operation {
    fn from_entry(input: &char) -> Operation {
        match input {
            '+' => Operation::Addition,
            '*' => Operation::Multiplication,
            _ => panic!("wrong operation received"),
        }
    }
}

struct Function {
    numbers: Vec<usize>,
    operation: Operation,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ({:?})", self.operation, self.numbers)
    }
}

impl Function {
    fn from_input(input: &Vec<String>) -> Vec<Function> {
        let columns = get_columns(input);
        let blocks: Vec<Vec<&[char; 5]>> = create_blocks(&mut columns.iter());
        blocks.iter().map(|block| Function::from_block(block)).collect::<Vec<Function>>()
    }

    fn from_block(block: &Vec<&[char; 5]>) -> Function {
        let operation = Operation::from_entry(block.get(0).unwrap().get(4).unwrap());
        let mut numbers: Vec<usize> = vec!();
        for column in block {
            numbers.push(to_number(column));
        }
        Function {
            numbers,
            operation
        }
    }

    fn apply(&self) -> usize {
        match self.operation {
            Operation::Addition => self
                .numbers
                .iter()
                .fold(0, |accumulator, next| accumulator + next),
            Operation::Multiplication => self
                .numbers
                .iter()
                .fold(1, |accumulator, next| accumulator * next),
        }
    }
}

fn to_number(column: &[char; 5]) -> usize {
    let mut vector: Vec<usize> = vec!();
    for i in 0..=4 {
        let a = column.get(i);
        if a.unwrap().is_digit(10) {
            vector.push(a.unwrap().to_digit(10).unwrap() as usize);
        }
    }
    let mut result: usize = 0;
    for index in 0..vector.len() {
        let base_number = vector.get(index).unwrap();
        let power: u32 = (vector.len() - index - 1) as u32;
        let total = (base_number) * (10_usize.pow(power));
        result += total
    }
    result
}

fn create_blocks<'a, I>(iterator: &mut I) -> Vec<Vec<&'a [char; 5]>>
where
    I: Iterator<Item = &'a [char; 5]>,
{
    let mut result: Vec<Vec<&[char; 5]>> = vec!();
    let mut block: Vec<&[char; 5]> = vec!();
    loop {
        match iterator.next() {
            Some(array) => {
                if is_empty(array) {
                    result.push((*block).to_vec());
                    block = vec!();
                } else {
                    block.push(array);
                }
            },
            None => {
                result.push(block);
                return result
            },
        }
    }
}

fn is_empty(input: &[char; 5]) -> bool {
    input.iter().all(|character| *character == ' ')
}

fn get_columns(input: &Vec<String>) -> Vec<[char; 5]> {
    let total_length = input.get(0).unwrap().chars().count();
    (0..total_length).map(|column| get_column(input, column)).collect()
}

fn get_column(input: &Vec<String>, column: usize) -> [char; 5] {
    [
        get_char(input, column, 0),
        get_char(input, column, 1),
        get_char(input, column, 2),
        get_char(input, column, 3),
        get_char(input, column, 4),
    ]
}

fn get_char(input: &Vec<String>, column: usize, row: usize) -> char {
    input.get(row).unwrap().chars().nth(column).unwrap()
}
