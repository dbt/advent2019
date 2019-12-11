
use crate::computer::{Cell, IntCode, ProgramState};
use crate::utils::{self, Result};

pub fn part1() -> Result<String> {
    run_program(1).map(|x| x.to_string())
}

pub fn part2() -> Result<String> {
    run_program(2).map(|x| x.to_string())
}

fn run_program(val: Cell) -> Result<Cell> {
    let prog = utils::load_program_cell("a09-input")?;
    let mut machine = IntCode::new(&prog);
    let (state, outputs) = machine.exec_many(&vec![val])?;
    assert_eq!(state, ProgramState::Halted);
    if outputs.len() != 1 {
        Err(format!("Expected 1 output but got {:?}", outputs))?;
    }
    Ok(outputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_relative_base() {
        struct Testcase {
            program: Vec<Cell>,
            input: Vec<Cell>,
            output: Vec<Cell>,
        }
        let testcases = vec![
            Testcase {
                program: vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
                input: vec![],
                output: vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
            },
        ];
        for case in testcases {
            let mut machine = IntCode::new(&case.program);
            let (state, actual) = machine.exec_many(&case.input).unwrap();
            assert_eq!(state, ProgramState::Halted);
            assert_eq!(actual, case.output);
        }

    }
}