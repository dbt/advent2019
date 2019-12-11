use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_program<P>(filename: P) -> Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let text = fs::read_to_string(filename)?;
    let parsed: std::result::Result<Vec<_>, _> =
        text.trim().split(',').map(|s| s.parse::<i32>()).collect();
    Ok(parsed?)
}

pub fn load_program_cell<P>(filename: P) -> Result<Vec<i128>>
where
    P: AsRef<Path>,
{
    let text = fs::read_to_string(filename)?;
    let parsed: std::result::Result<Vec<_>, _> =
        text.trim().split(',').map(|s| s.parse::<i128>()).collect();
    Ok(parsed?)
}
