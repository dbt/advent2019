use crate::computer::exec;
use crate::utils::{self, Result};

fn advent02_prog() -> Result<Vec<i32>> {
    utils::load_program("a02-input")
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
