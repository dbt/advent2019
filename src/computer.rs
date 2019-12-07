use std::error;
use std::fmt;

use crate::utils::Result;

#[derive(Debug,Clone)]
pub struct InvalidOpcode {
    opcode: i32
}

impl InvalidOpcode {
    pub fn new(opcode: i32) -> InvalidOpcode {
        InvalidOpcode{opcode}
    }
}

impl fmt::Display for InvalidOpcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid opcode: {}", self.opcode)
    }
}

impl error::Error for InvalidOpcode {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn reg(program: &Vec<i32>, op: i32, pc: usize, nth: usize, addr: bool) -> i32 {
    let val = program[pc + nth];
    let mask = (op as u32)  /(10_u32.pow(nth as u32+1))%10;
    assert!(!addr || mask == 0, "addr mode set but mask is 1");
    if !addr && mask == 0 {
        program[val as usize]
    } else {
        val
    }
}

fn rr(program: &Vec<i32>, op: i32, pc: usize, nth: usize) -> i32 {
    return reg(program, op, pc, nth, false);
}

fn ra(program: &Vec<i32>, op: i32, pc: usize, nth: usize) -> usize {
    return reg(program, op, pc, nth, true) as usize;
}

pub fn exec(program: &mut Vec<i32>, inputs: &Vec<i32>) -> Result<Vec<i32>> {
    let mut pc: usize = 0;
    let mut output: Vec<i32> = Vec::new();
    let mut inp_it = inputs.iter();
    loop {
        let op = program[pc];
        let opcode = op % 100;
        if opcode == 99 {
            return Ok(output);
        }
        if opcode < 1 || opcode > 8 {
            return Err(Box::new(InvalidOpcode::new(opcode)))
        }
        match opcode {
            1 => { // add
                let r1 = rr(program, op, pc, 1);
                let r2 = rr(program, op, pc, 2);
                let rv = ra(program, op, pc, 3);
                program[rv] = r1 + r2;
                pc += 4;
            }
            2 => { // multiply
                let r1 = rr(program, op, pc, 1);
                let r2 = rr(program, op, pc, 2);
                let rv = ra(program, op, pc, 3);
                program[rv] = r1 * r2;
                pc += 4;
            }
            3 => { // input
                let rv = ra(program, op, pc, 1);
                program[rv] = *inp_it.next().unwrap();
                pc += 2;
            }
            4 => { // output 
                let r1 = rr(program, op, pc, 1);
                output.push(r1);
                pc += 2;
            }
            5 => { // jump-if-nonzero
                let r1 = rr(program, op, pc, 1);
                if r1 != 0 {
                    pc = rr(program, op, pc, 2) as usize;
                } else {
                    pc += 3;
                }
            }
            6 => { // jump-if-zero
                let r1 = rr(program, op, pc, 1);
                if r1 == 0 {
                    pc = rr(program, op, pc, 2) as usize;
                } else {
                    pc += 3;
                }
            }
            7 => { // less-than
                let r1 = rr(program, op, pc, 1);
                let r2 = rr(program, op, pc, 2);
                let rv = ra(program, op, pc, 3);
                program[rv] = if r1 < r2 { 1 } else { 0 };
                pc += 4;
            }
            8 => { // equals
                let r1 = rr(program, op, pc, 1);
                let r2 = rr(program, op, pc, 2);
                let rv = ra(program, op, pc, 3);
                program[rv] = if r1 == r2 { 1 } else { 0 };
                pc += 4;
            }
            _ => ()
        }
    }
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
            exec(&mut program, &Vec::new()).unwrap();
            assert_eq!(program, case.expected);
        }
    }

    #[test]
    fn test_exec() {
        #[derive(Debug)]
        struct Testcase {
            program: Vec<i32>,
            input: Vec<i32>,
            output: Vec<i32>
        }
        let testcases = vec![
            Testcase{
                program: vec![1101, 100, -1, 4, 0],
                input:   vec![],
                output:  vec![],
            },
            Testcase{
                program: vec![3,0,4,0,99],
                input:   vec![777],
                output:  vec![777],
            },
            Testcase{
                program: vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],
                input:   vec![7],
                output:  vec![999],
            },
            Testcase{
                program: vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9],
                input:   vec![0],
                output:  vec![0],
            },
            Testcase{
                program: vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9],
                input:   vec![-5],
                output:  vec![1],
            },
        ];
        for case in testcases.iter() {
            let mut program = case.program.clone();
            let output = exec(&mut program, &case.input);
            assert!(output.is_ok());
            assert_eq!(output.unwrap(), case.output);
        }
    }
}