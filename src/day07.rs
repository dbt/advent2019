use crate::utils::{self, Result};
use crate::computer;

pub fn part1() -> Result<String> {
    let prog = utils::load_program("a07-input")?;
    Ok(find_best(&prog, 0, start_avail())?.to_string())
}

fn start_avail() -> Vec<i32> {
    (0_i32..5_i32).collect::<Vec<i32>>()
}

fn find_best(prog: &Vec<i32>, carry: i32, avail: Vec<i32>) -> Result<i32> {
    if avail.len() == 0 {
        return Ok(carry);
    }
    let mut best = std::i32::MIN;
    for opcode in &avail {
        let inputs= vec![*opcode, carry];
        let outputs = computer::exec(&mut prog.clone(), &inputs)?;
        if outputs.len() != 1 {
            Err(format!("Expected 1 result but got {}", outputs.len()))?;
        }
        let val = find_best(prog, outputs[0], avail.iter().copied().filter(|x| x != opcode).collect())?;
        if val > best {
            best = val;
        }
    }
    return Ok(best);
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
            Testcase{
                prog: vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],
                expected: 43210,
            },
            Testcase{
                prog: vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0],
                expected: 54321,
            },
            Testcase{
                prog: vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0],
                expected: 65210,
            },
        ];
        for case in testcases {
            let best = find_best(&case.prog, 0, start_avail()).unwrap();
            assert_eq!(best, case.expected);
        }
    }
}