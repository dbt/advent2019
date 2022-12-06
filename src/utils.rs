pub use adventools::prelude::read_lines;
pub use anyhow::Result;
use computer::Cell;
use std::fs;
use std::path::Path;

pub fn load_program<P>(filename: P) -> Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let text = fs::read_to_string(filename)?;
    let parsed: std::result::Result<Vec<_>, _> =
        text.trim().split(',').map(|s| s.parse::<i32>()).collect();
    Ok(parsed?)
}

pub fn load_program_cell<P>(filename: P) -> Result<Vec<Cell>>
where
    P: AsRef<Path>,
{
    let text = fs::read_to_string(filename)?;
    let parsed: Vec<_> = text
        .trim()
        .split(',')
        .map(|s| s.parse::<Cell>().unwrap())
        .collect();
    Ok(parsed)
}
