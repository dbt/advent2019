
extern crate advent2019;

use advent2019::utils::Result;
use advent2019::day01::{advent01a, advent01b};
use advent2019::day02::{advent02a, advent02b};
use advent2019::day03::{advent03a, advent03b};
use advent2019::day04::{advent04a, advent04b};

fn main() -> Result<()> {
    println!("advent 01a: {}", advent01a()?);
    println!("advent 01b: {}", advent01b()?);
    println!("advent 02a: {}", advent02a()?);
    println!("advent 02b: {}", advent02b()?);
    println!("advent 03a: {}", advent03a()?);
    println!("advent 03b: {}", advent03b()?);
    println!("advent 04a: {}", advent04a()?);
    println!("advent 04b: {}", advent04b()?);
    Ok(())
}
