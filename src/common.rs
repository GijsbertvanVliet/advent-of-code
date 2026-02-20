use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_multiple_lines(file_name: &str) -> Result<Vec<String>, std::io::Error> {
    let mut result = Vec::new();
    let file: File = File::open(file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        result.push(line?);
    }
    Ok(result)
}

pub fn read_file_one_line(file_name: &str) -> Result<String, std::io::Error> {
    let file: File = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut result: String = String::new();
    reader.read_line(&mut result)?;
    Ok(result)
}

