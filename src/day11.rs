use std::collections::HashMap;

use crate::computer::{Cell, IntCode, ProgramState};
use crate::utils::load_program_cell;
use adventools::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    // 0 = left, 1 = right
    fn rotate(&self, code: u8) -> Direction {
        let idx = match self {
            Direction::UP => 0,
            Direction::RIGHT => 1,
            Direction::DOWN => 2,
            Direction::LEFT => 3,
        } + match code {
            0 => 0,
            1 => 2,
            _ => unimplemented!(),
        };
        return [
            Direction::LEFT,
            Direction::UP,
            Direction::RIGHT,
            Direction::DOWN,
            Direction::LEFT,
            Direction::UP,
        ][idx];
    }
}

struct Robot {
    panels: HashMap<(i32, i32), u8>,
    dir: Direction,
    pos_x: i32,
    pos_y: i32,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            panels: HashMap::new(),
            dir: Direction::UP,
            pos_x: 0,
            pos_y: 0,
        }
    }
    fn paint(&mut self, color: u8) {
        self.panels.insert((self.pos_x, self.pos_y), color);
    }
    fn paint_and_move(&mut self, color: u8, direction: u8) {
        self.paint(color);
        self.dir = self.dir.rotate(direction);
        match self.dir {
            Direction::UP => self.pos_y -= 1,
            Direction::RIGHT => self.pos_x += 1,
            Direction::DOWN => self.pos_y += 1,
            Direction::LEFT => self.pos_x -= 1,
        }
    }
    fn count(&self) -> usize {
        self.panels.len()
    }
    fn read(&self) -> u8 {
        *self
            .panels
            .get(&(self.pos_x, self.pos_y))
            .or(Some(&0))
            .unwrap()
    }
    fn render(&self) -> String {
        let min_x = self.panels.keys().map(|(x, _)| *x).min().unwrap();
        let max_x = self.panels.keys().map(|(x, _)| *x).max().unwrap();
        let min_y = self.panels.keys().map(|(_, y)| *y).min().unwrap();
        let max_y = self.panels.keys().map(|(_, y)| *y).max().unwrap();
        let s: Vec<String> = (min_y..=max_y)
            .map(|y| {
                (min_x..=max_x)
                    .map(move |x| match self.panels.get(&(x, y)) {
                        None => ' ',
                        Some(0) => ' ',
                        Some(1) => '█',
                        _ => unimplemented!(),
                    })
                    .collect()
            })
            .collect();
        s.join("\n")
    }
}

fn run(initial: u8) -> Result<Robot> {
    let prog = load_program_cell("input11.txt")?;
    let mut robot = Robot::new();
    robot.paint(initial);
    let mut runner = IntCode::new(&prog);
    let mut state = runner.exec_multiple()?;
    while state != ProgramState::Halted {
        if state != ProgramState::Input {
            unimplemented!();
        }
        runner.feed(robot.read() as Cell);
        if let ProgramState::Output(color) = runner.exec_multiple()? {
            if let ProgramState::Output(dir) = runner.exec_multiple()? {
                robot.paint_and_move(color as u8, dir as u8);
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!();
        }
        state = runner.exec_multiple()?;
    }
    Ok(robot)
}

pub struct D {}
impl Day for D {
    fn number(&self) -> u8 {
        11
    }

    fn part01(&self) -> Result<()> {
        let robot = run(0)?;
        println!("{}", robot.count());

        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let robot = run(1)?;
        println!("\n{}", robot.render());

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rotate() {
        assert_eq!(Direction::UP.rotate(0), Direction::LEFT);
        assert_eq!(Direction::LEFT.rotate(0), Direction::DOWN);
        assert_eq!(Direction::DOWN.rotate(0), Direction::RIGHT);
        assert_eq!(Direction::RIGHT.rotate(0), Direction::UP);
    }

    #[test]
    fn test_p1() {
        let mut r = Robot::new();
        r.paint_and_move(1, 0);
        assert_eq!(r.dir, Direction::LEFT);
        r.paint_and_move(0, 0);
        assert_eq!(r.dir, Direction::DOWN);
        r.paint_and_move(1, 0);
        assert_eq!(r.dir, Direction::RIGHT);
        r.paint_and_move(1, 0);
        assert_eq!(r.dir, Direction::UP);
        r.paint_and_move(0, 1);
        assert_eq!(r.dir, Direction::RIGHT);
        r.paint_and_move(1, 0);
        assert_eq!(r.dir, Direction::UP);
        r.paint_and_move(1, 0);
        assert_eq!(r.dir, Direction::LEFT);
        println!("{:?}", r.panels);
        assert_eq!(r.count(), 6);
    }
}
