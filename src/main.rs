
use std::result::Result;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("{}", advent01a().unwrap());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn advent01a_fuel(mass: u32) -> u32 {
    return (mass/3)-2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_advent01a_fuel() {
        assert_eq!(advent01a_fuel(12), 2);
        assert_eq!(advent01a_fuel(14), 2);
        assert_eq!(advent01a_fuel(1969), 654);
        assert_eq!(advent01a_fuel(100756), 33583);
    }
}

fn advent01a() -> Result<String, Box<dyn Error>> {
    let lines = read_lines("a01-input")?;
    let mut total: u32 = 0;
    for line in lines {
        let val = line?.parse::<u32>()?;
        total += advent01a_fuel(val);
    }
    Ok(total.to_string())
}