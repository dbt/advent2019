use crate::computer::{self, Cell};
use crate::utils::{self};
use adventools::prelude::*;
use anyhow::anyhow;

pub struct D {}
impl Day for D {
    fn number(&self) -> u8 {
        7
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
    let prog = utils::load_program("input07.txt")?;
    Ok(find_best(&prog, 0, start_avail())?.to_string())
}

pub fn part2() -> Result<String> {
    let prog = utils::load_program_cell("input07.txt")?;
    find_best_chain(&prog, &vec![], &start_chain()).map(|x| x.to_string())
}

fn start_avail() -> Vec<i32> {
    (0..5).collect::<Vec<i32>>()
}

fn find_best(prog: &Vec<i32>, carry: i32, avail: Vec<i32>) -> Result<i32> {
    if avail.len() == 0 {
        return Ok(carry);
    }
    let mut best = std::i32::MIN;
    for opcode in &avail {
        let inputs = vec![*opcode, carry];

        let outputs = computer::exec(&mut prog.clone(), &inputs)?;
        if outputs.len() != 1 {
            Err(anyhow!("Expected 1 result but got {}", outputs.len()))?;
        }
        let val = find_best(
            prog,
            outputs[0],
            avail.iter().copied().filter(|x| x != opcode).collect(),
        )?;
        if val > best {
            best = val;
        }
    }
    return Ok(best);
}

fn start_chain() -> Vec<Cell> {
    (5..=9).collect::<Vec<Cell>>()
}

fn find_best_chain(prog: &Vec<Cell>, codes: &Vec<Cell>, avail: &Vec<Cell>) -> Result<Cell> {
    if avail.len() == 0 {
        return exec_chain(prog, codes);
    }
    let mut best = std::i32::MIN as Cell;
    for opcode in avail {
        let new_codes: Vec<Cell> = codes.iter().chain(&[*opcode]).copied().collect();
        let new_avail: Vec<Cell> = avail.iter().copied().filter(|x| x != opcode).collect();
        let val = find_best_chain(&prog, &new_codes, &new_avail)?;
        if val > best {
            best = val;
        }
    }
    Ok(best)
}

fn exec_chain(prog: &Vec<Cell>, codes: &Vec<Cell>) -> Result<Cell> {
    let mut carry = Some(0);
    let mut computers: Vec<computer::IntCode> = Vec::new();
    for i in 0..codes.len() {
        computers.push(computer::IntCode::new(prog));
        computers[i].feed(codes[i]);
        assert!(computers[i].exec_multiple()? == computer::ProgramState::Input);
    }
    loop {
        for i in 0..codes.len() {
            // println!("feeding {} into amp {}", carry.unwrap(), i);
            computers[i].feed(carry.take().unwrap());
            let mut blocked = false;
            while !blocked {
                match computers[i].exec_multiple()? {
                    computer::ProgramState::Output(val) => {
                        if carry.replace(val).is_some() {
                            panic!("multiple values in flight!");
                        }
                        // println!("captured {} from amp {}", carry.unwrap(), i);
                    }
                    computer::ProgramState::Input => {
                        blocked = true;
                    }
                    computer::ProgramState::Halted => {
                        blocked = true;
                        if i + 1 == codes.len() {
                            return Ok(carry.unwrap());
                        }
                    }
                    computer::ProgramState::Ready => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_best() {
        struct Testcase {
            prog: Vec<i32>,
            expected: i32,
        }
        let testcases = vec![
            Testcase {
                prog: vec![
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
                ],
                expected: 43210,
            },
            Testcase {
                prog: vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0,
                ],
                expected: 54321,
            },
            Testcase {
                prog: vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
                ],
                expected: 65210,
            },
        ];
        for case in testcases {
            let best = find_best(&case.prog, 0, start_avail()).unwrap();
            assert_eq!(best, case.expected);
        }
    }

    #[test]
    fn test_find_best_chain() {
        struct Testcase {
            prog: Vec<Cell>,
            expected: Cell,
        }
        let testcases = vec![
            Testcase {
                prog: vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
                ],
                expected: 139629729,
            },
            Testcase {
                prog: vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
                ],
                expected: 18216,
            },
        ];
        for case in testcases {
            let best = find_best_chain(&case.prog, &vec![], &start_chain()).unwrap();
            assert_eq!(best, case.expected);
        }
    }
}
