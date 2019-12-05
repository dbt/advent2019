
extern crate advent2019;

use advent2019::utils::Result;
use advent2019::advent01::{advent01a, advent01b};
use advent2019::advent02::{advent02a, advent02b};
use advent2019::advent03::{advent03a, advent03b};

fn main() -> Result<()> {
    println!("advent 01a: {}", advent01a()?);
    println!("advent 01b: {}", advent01b()?);
    println!("advent 02a: {}", advent02a()?);
    println!("advent 02b: {}", advent02b()?);
    println!("advent 03a: {}", advent03a()?);
    println!("advent 03b: {}", advent03b()?);
    Ok(())
}
