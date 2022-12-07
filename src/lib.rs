extern crate adventools;
extern crate anyhow;

use adventools::prelude::*;

extern crate num_bigint;
extern crate num_traits;

pub mod computer;
pub mod utils;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day11;

pub fn all_days() -> Vec<Box<dyn Day>> {
    vec![
        Box::new(day01::D {}),
        Box::new(day02::D {}),
        Box::new(day03::D {}),
        Box::new(day04::D {}),
        Box::new(day05::D {}),
        Box::new(day06::D {}),
        Box::new(day07::D {}),
        Box::new(day08::D {}),
        Box::new(day09::D {}),
        Box::new(day11::D {}),
    ]
}
