
use std::fmt;
use std::error::{self, Error};
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    println!("advent 01b: {}", advent01a()?);
    println!("advent 02a: {}", advent02a()?);
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn advent01a_fuel(mass: i32) -> i32 {
    let val = (mass/3)-2;
    if val <= 0 {
        return 0;
    }
    if val < 9 {
        return val;
    }
    let addl = advent01a_fuel(val);
    return val + addl;
}

fn advent01a() -> Result<String> {
    let lines = read_lines("a01-input")?;
    let mut total: i32 = 0;
    for line in lines {
        let val = line?.parse::<i32>()?;
        total += advent01a_fuel(val);
    }
    Ok(total.to_string())
}

#[derive(Debug,Clone)]
struct InvalidOpcode {
    opcode: i32
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

fn advent02_exec(program: &mut Vec<i32>) -> Result<()> {
    let mut pc = 0;
    loop {
        let opcode = program[pc];
        if opcode == 99 {
            return Ok(());
        }
        if opcode < 1 || opcode > 2 {
            return Err(Box::new(InvalidOpcode{opcode}))
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
    let text = fs::read_to_string("a02-input")?;
    let parsed: std::result::Result<Vec<_>, _> = text.trim().split(',').map(|s| s.parse::<i32>()).collect();
    Ok(parsed?)
}

fn advent02a() -> Result<String> {
    let mut prog = advent02_prog()?;
    prog[1] = 12;
    prog[2] = 2;
    advent02_exec(&mut prog)?;
    Ok(prog[0].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_advent01a_fuel() {
        assert_eq!(advent01a_fuel(12), 2);
        assert_eq!(advent01a_fuel(14), 2);
        assert_eq!(advent01a_fuel(1969), 966);
        assert_eq!(advent01a_fuel(100756), 50346);
    }

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
