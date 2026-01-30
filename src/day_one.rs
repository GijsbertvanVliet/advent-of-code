use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_password() {
    let lock_movements: Vec<LockMovement> = get_lock_movements();
    let mut lock = Lock::new();
    let mut number_of_zeroes = 0;
    for movement in lock_movements {
        println!("applying {}", movement);
        lock.apply_movement(&movement);
        println!("results in {}", lock.state);
        if lock.state == 0 {
            number_of_zeroes +=1;
        }
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
    let amount: u8 = (line[1..].parse::<u32>().unwrap() % 100) as u8;
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
    amount: u8,
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

    fn apply_movement(&mut self, movement: &LockMovement) {
        match movement.direction {
            Direction::Left => {
                if movement.amount > self.state {
                    self.state = (100 - movement.amount + self.state) % 100
                } else {
                    self.state = (self.state - movement.amount) % 100
                }
            }

            Direction::Right => self.state = (self.state + movement.amount) % 100,
        }
    }
}
