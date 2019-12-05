
use crate::utils::{self, Result};
use crate::computer::InvalidOpcode;

fn advent02_exec(program: &mut Vec<i32>) -> Result<()> {
    let mut pc = 0;
    loop {
        let opcode = program[pc];
        if opcode == 99 {
            return Ok(());
        }
        if opcode < 1 || opcode > 2 {
            return Err(Box::new(InvalidOpcode::new(opcode)))
        }
        let r1 = program[pc+1] as usize;
        let r2 = program[pc+2] as usize;
        let rr = program[pc+3] as usize;
        match opcode {
            1 => { program[rr] = program[r1] + program[r2]; }
            2 => { program[rr] = program[r1] * program[r2]; }
            _ => ()
        }
        pc += 4;
    }
}

fn advent02_prog() -> Result<Vec<i32>> {
    utils::load_program("a02-input")
}

pub fn part1() -> Result<String> {
    let mut prog = advent02_prog()?;
    prog[1] = 12;
    prog[2] = 2;
    advent02_exec(&mut prog)?;
    Ok(prog[0].to_string())
}

pub fn part2() -> Result<String> {
    let prog = advent02_prog()?;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut copy = prog.clone();
            copy[1] = noun;
            copy[2] = verb;
            advent02_exec(&mut copy)?;
            if copy[0] == 19690720 {
                return Ok((100*noun+verb).to_string());
            }
        }
    }
    Ok("Not found".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advent02_exec() {
        #[derive(Debug)]
        struct Testcase {
            input: Vec<i32>,
            expected: Vec<i32>
        }
        let testcases: [Testcase; 5] = [
            Testcase{
                input:    vec![1,   9,10,3, 2,3,11,0,99,30,40,50], 
                expected: vec![3500,9,10,70,2,3,11,0,99,30,40,50]},
            Testcase{
                input:    vec![1,0,0,0,99], 
                expected: vec![2,0,0,0,99]
            },
            Testcase{
                input:    vec![2,3,0,3,99],
                expected: vec![2,3,0,6,99]
            },
            Testcase{
                input:    vec![2,4,4,5,99,0],
                expected: vec![2,4,4,5,99,9801]
            },
            Testcase{
                input:    vec![1, 1,1,4,99,5,6,0,99],
                expected: vec![30,1,1,4,2, 5,6,0,99],
            }
        ];
        for case in testcases.iter() {
            let mut program = case.input.clone();
            advent02_exec(&mut program).unwrap();
            assert_eq!(program, case.expected);
        }
    }
}
