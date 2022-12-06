use crate::computer::exec;
use crate::utils::{self, Result};
use adventools::prelude::*;

fn advent02_prog() -> Result<Vec<i32>> {
    utils::load_program("input02.txt")
}

pub struct D {}
impl Day for D {
    fn number(&self) -> u8 {
        2
    }
    fn part01(&self) -> Result<()> {
        println!("{}", part1()?);
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        println!("{}", part2()?);
        Ok(())
    }
}

pub fn part1() -> Result<String> {
    let mut prog = advent02_prog()?;
    prog[1] = 12;
    prog[2] = 2;
    exec(&mut prog, &Vec::new())?;
    Ok(prog[0].to_string())
}

pub fn part2() -> Result<String> {
    let prog = advent02_prog()?;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut copy = prog.clone();
            copy[1] = noun;
            copy[2] = verb;
            exec(&mut copy, &Vec::new())?;
            if copy[0] == 19690720 {
                return Ok((100 * noun + verb).to_string());
            }
        }
    }
    Ok("Not found".to_string())
}
