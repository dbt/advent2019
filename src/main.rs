
extern crate advent2019;

use advent2019::utils::Result;
use advent2019::day01;
use advent2019::day02;
use advent2019::day03;
use advent2019::day04;
use advent2019::day05;
use advent2019::day06;
use advent2019::day07;
use advent2019::day08;

fn main() -> Result<()> {
    println!("day  1 part 1: {}", day01::part1()?);
    println!("day  1 part 2: {}", day01::part2()?);
    println!("day  2 part 1: {}", day02::part1()?);
    println!("day  2 part 2: {}", day02::part2()?);
    println!("day  3 part 1: {}", day03::part1()?);
    println!("day  3 part 2: {}", day03::part2()?);
    println!("day  4 part 1: {}", day04::part1()?);
    println!("day  4 part 2: {}", day04::part2()?);
    println!("day  5 part 1: {}", day05::part1()?);
    println!("day  5 part 2: {}", day05::part2()?);
    println!("day  6 part 1: {}", day06::part1()?);
    println!("day  6 part 2: {}", day06::part2()?);
    println!("day  7 part 1: {}", day07::part1()?);
    println!("day  8 part 1: {}", day08::part1()?);
    println!("day  8 part 2: \n{}", day08::part2()?);

    Ok(())
}
