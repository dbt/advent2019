
use crate::computer;
use crate::utils::{self, Result};

fn diagnostic(subsystem: i32) -> Result<String> {
    let mut prog = utils::load_program("a05-input")?;
    let inputs = vec![subsystem];
    let outputs = computer::exec(&mut prog, &inputs)?;
    let val: Vec<_> = outputs.iter().skip_while(|x| **x == 0).collect();
    if val.len() == 0 {
        if outputs.len() == 0 {
            Err("no outputs")?;
        }
        return Ok("0".to_string());
    } if val.len() == 1 {
        return Ok(val[0].to_string())
    } else {
        Err(format!("some diagnostics failed: {:?}", outputs))?
    }
}

pub fn part1() -> Result<String> {
    return diagnostic(1);
}

pub fn part2() -> Result<String> {
    return diagnostic(5);
}

