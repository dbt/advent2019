use std::collections::HashMap;
use std::error;
use std::fmt;

use crate::utils::Result;

pub type Cell = i128;

#[derive(Debug, Clone)]
pub struct InvalidOpcode {
    opcode: usize,
}

impl InvalidOpcode {
    pub fn new(opcode: usize) -> InvalidOpcode {
        InvalidOpcode { opcode }
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProgramState {
    Ready,
    Input,
    Output(Cell),
    Halted,
}

pub struct IntCode {
    program: Vec<Cell>,
    state: ProgramState,
    pc: usize,
    base_rel: i64,
    extended: HashMap<usize, Cell>,
    input: Option<Cell>,
}

impl IntCode {
    pub fn new(program: &Vec<Cell>) -> IntCode {
        return IntCode {
            program: program.iter().copied().collect(),
            state: ProgramState::Ready,
            pc: 0,
            base_rel: 0,
            extended: HashMap::new(),
            input: None,
        };
    }

    fn read(&self, addr: usize) -> Cell {
        if self.program.len() <= addr {
            *self.extended.get(&addr).unwrap_or(&0)
        } else {
            self.program[addr]
        }
    }

    fn write(&mut self, addr: usize, val: Cell) {
        if addr >= self.program.len() {
            self.extended.insert(addr, val);
        } else {
            self.program[addr] = val;
        }
    }

    fn reg(&self, nth: usize, addr: bool) -> Cell {
        let op = self.read(self.pc);
        let mut val = self.read(self.pc + nth);
        let mask = (op as u32) / (10_u32.pow(nth as u32 + 1)) % 10;
        if mask == 1 {
            assert!(!addr, "addr mode set but mask read mode is immediate");
            return val;
        }
        if mask == 2 {
            val += self.base_rel as Cell;
        }
        if addr {
            return val;
        }
        return self.read(val as usize);
    }

    fn rr(&self, nth: usize) -> Cell {
        return self.reg(nth, false);
    }

    fn ra(&self, nth: usize) -> usize {
        return self.reg(nth, true) as usize;
    }

    pub fn exec_one(&mut self) -> Result<ProgramState> {
        let op = self.read(self.pc);
        let opcode = op % 100;
        if opcode == 99 || self.state == ProgramState::Halted {
            self.state = ProgramState::Halted;
            return Ok(self.state);
        }
        if opcode < 1 || opcode > 9 {
            Err(InvalidOpcode::new(opcode as usize))?;
        }
        self.state = match opcode {
            1 => {
                // add
                let r1 = self.rr(1);
                let r2 = self.rr(2);
                let rv = self.ra(3);
                self.write(rv, r1 + r2);
                self.pc += 4;
                ProgramState::Ready
            }
            2 => {
                // multiply
                let r1 = self.rr(1);
                let r2 = self.rr(2);
                let rv = self.ra(3);
                self.write(rv, r1 * r2);
                self.pc += 4;
                ProgramState::Ready
            }
            3 => {
                // input
                if let Some(val) = self.input.take() {
                    let rv = self.ra(1);
                    self.write(rv, val);
                    self.pc += 2;
                    ProgramState::Ready
                } else {
                    ProgramState::Input
                }
            }
            4 => {
                // output
                let r1 = self.rr(1);
                self.pc += 2;
                ProgramState::Output(r1)
            }
            5 => {
                // jump-if-nonzero
                let r1 = self.rr(1);
                if r1 != 0 {
                    self.pc = self.rr(2) as usize;
                } else {
                    self.pc += 3;
                }
                ProgramState::Ready
            }
            6 => {
                // jump-if-zero
                let r1 = self.rr(1);
                if r1 == 0 {
                    self.pc = self.rr(2) as usize;
                } else {
                    self.pc += 3;
                }
                ProgramState::Ready
            }
            7 => {
                // less-than
                let r1 = self.rr(1);
                let r2 = self.rr(2);
                let rv = self.ra(3);
                self.write(rv, if r1 < r2 { 1 } else { 0 });
                self.pc += 4;
                ProgramState::Ready
            }
            8 => {
                // equals
                let r1 = self.rr(1);
                let r2 = self.rr(2);
                let rv = self.ra(3);
                self.write(rv, if r1 == r2 { 1 } else { 0 });
                self.pc += 4;
                ProgramState::Ready
            }
            9 => {
                // set-relative-base
                let r1 = self.rr(1) as i64;
                self.base_rel += r1;
                self.pc += 2;
                ProgramState::Ready
            }
            _ => ProgramState::Halted,
        };
        Ok(self.state)
    }

    pub fn feed(&mut self, input: Cell) {
        self.input = Some(input);
    }

    pub fn exec_multiple(&mut self) -> Result<ProgramState> {
        let mut state = ProgramState::Ready;
        while state == ProgramState::Ready {
            state = self.exec_one()?;
        }
        Ok(self.state)
    }

    pub fn exec_many(&mut self, inputs: &Vec<Cell>) -> Result<(ProgramState, Vec<Cell>)> {
        let mut output = vec![];
        let mut inp_it = inputs.iter();
        loop {
            match self.exec_multiple()? {
                ProgramState::Input => {
                    if let Some(x) = inp_it.next() {
                        self.feed(*x);
                    } else {
                        return Ok((self.state, output));
                    }
                }
                ProgramState::Output(x) => {
                    output.push(x);
                }
                ProgramState::Halted => {
                    return Ok((self.state, output));
                }
                _ => (),
            }
        }
    }
}

pub fn exec(program: &mut Vec<i32>, inputs: &Vec<i32>) -> Result<Vec<i32>> {
    let prog = program.iter().map(|x| *x as Cell).collect();
    let mut machine = IntCode::new(&prog);
    let mut output: Vec<i32> = Vec::new();
    let mut inp_it = inputs.iter();
    loop {
        match machine.exec_multiple()? {
            ProgramState::Ready => (),
            ProgramState::Input => {
                machine.feed(*inp_it.next().unwrap() as Cell);
            }
            ProgramState::Output(val) => {
                output.push(val as i32);
            }
            ProgramState::Halted => {
                for i in 0..program.len() {
                    program[i] = machine.program[i] as i32;
                }
                return Ok(output);
            }
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
            expected: Vec<i32>,
        }
        let testcases: [Testcase; 5] = [
            Testcase {
                input: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                expected: vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            },
            Testcase {
                input: vec![1, 0, 0, 0, 99],
                expected: vec![2, 0, 0, 0, 99],
            },
            Testcase {
                input: vec![2, 3, 0, 3, 99],
                expected: vec![2, 3, 0, 6, 99],
            },
            Testcase {
                input: vec![2, 4, 4, 5, 99, 0],
                expected: vec![2, 4, 4, 5, 99, 9801],
            },
            Testcase {
                input: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                expected: vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            },
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
            output: Vec<i32>,
        }
        let testcases = vec![
            Testcase {
                program: vec![1101, 100, -1, 4, 0],
                input: vec![],
                output: vec![],
            },
            Testcase {
                program: vec![3, 0, 4, 0, 99],
                input: vec![777],
                output: vec![777],
            },
            Testcase {
                program: vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                input: vec![7],
                output: vec![999],
            },
            Testcase {
                program: vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                input: vec![0],
                output: vec![0],
            },
            Testcase {
                program: vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                input: vec![-5],
                output: vec![1],
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
