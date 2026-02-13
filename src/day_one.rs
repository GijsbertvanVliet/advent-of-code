use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_password() {
    let lock_movements: Vec<LockMovement> = get_lock_movements();
    let mut lock = Lock::new();
    let mut number_of_zeroes = 0;
    for movement in lock_movements {
        // println!("applying {}", movement);
        lock.apply_movement(&movement, &mut number_of_zeroes);
        // println!("results in {}", lock.state);
    }
    println!("password = {}", number_of_zeroes);
}

fn get_lock_movements() -> Vec<LockMovement> {
    let lines_result = read_file();
    let mut lock_movements = Vec::new();
    if let Ok(result) = lines_result {
        for line in result {
            lock_movements.push(create_lock_movement(&line));
        }
    }
    lock_movements
}

fn create_lock_movement(line: &str) -> LockMovement {
    let directional_char: char = line.chars().nth(0).unwrap();
    let direction = match directional_char {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid directional character"),
    };
    let amount: u32 = line[1..].parse::<u32>().unwrap();
    LockMovement { direction, amount }
}

fn read_file() -> Result<Vec<String>, std::io::Error> {
    let mut result = Vec::new();
    let file: File = File::open("resources/day1Input.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        result.push(line?);
    }
    Ok(result)
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct LockMovement {
    direction: Direction,
    amount: u32,
}

impl Display for LockMovement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "direction: {}, amount: {}", self.direction, self.amount)
    }
}

#[derive(Debug)]
struct Lock {
    state: u8,
}

impl Lock {
    fn new() -> Lock {
        Lock { state: 50 }
    }

    fn apply_movement(&mut self, movement: &LockMovement,  number_of_zeroes: &mut i32 ) {
        match movement.direction {
            Direction::Left => for _ in 0..movement.amount {
                self.decrease_one(number_of_zeroes)
            },
            Direction::Right => for _ in 0 .. movement.amount {
                self.increase_one(number_of_zeroes)
            },
        }
    }

    fn increase_one(&mut self, number_of_zeroes: &mut i32) {
        self.state = (self.state + 1) % 100;
        if self.state == 0 {
            *number_of_zeroes += 1;
        }
    }

    fn decrease_one(&mut self, number_of_zeroes: &mut i32) {
        match self.state {
            0 => self.state = 99,
            1 => {
                self.state = 0;
                *number_of_zeroes += 1;
            }
            _ => self.state -= 1,
        }
    }
}
