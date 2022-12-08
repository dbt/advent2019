use std::collections::HashMap;

use adventools::prelude::*;
use utils::load_program_cell;

use crate::computer::{Cell, IntCode, ProgramState};

type Coord = (Cell, Cell);

struct Cabinet {
    minx: Cell,
    maxx: Cell,
    miny: Cell,
    maxy: Cell,
    score: Cell,
    view: HashMap<Coord, Cell>,
}

impl Cabinet {
    fn new() -> Cabinet {
        Cabinet {
            minx: 0,
            maxx: 0,
            miny: 0,
            maxy: 0,
            score: 0,
            view: HashMap::new(),
        }
    }

    fn draw(&mut self, (x, y): Coord, p: Cell) {
        if x == -1 && y == 0 {
            self.score = p;
            return;
        }
        if x < self.minx {
            self.minx = x
        }
        if x > self.maxx {
            self.maxx = x
        }
        if y < self.miny {
            self.miny = y
        }
        if y > self.maxy {
            self.maxy = y
        }
        if p == 0 {
            self.view.remove(&(x, y));
        } else {
            self.view.insert((x, y), p);
        }
    }

    fn run(&mut self, comp: &mut IntCode) -> Result<ProgramState> {
        let mut state = ProgramState::Ready;
        while state != ProgramState::Halted {
            state = comp.exec_multiple()?;
            match state {
                ProgramState::Output(x) => {
                    // draw command
                    if let ProgramState::Output(y) = comp.exec_multiple()? {
                        if let ProgramState::Output(p) = comp.exec_multiple()? {
                            self.draw((x, y), p);
                        } else {
                            unimplemented!();
                        }
                    } else {
                        unimplemented!();
                    }
                }
                x => return Ok(x),
            }
        }
        Ok(state)
    }
    fn find_tile_xpos(&self, tile: Cell) -> Vec<Cell> {
        self.view
            .iter()
            .filter(|&(&_k, &v)| v == tile)
            .map(|(&(x, _y), &_v)| x)
            .collect()
    }
    fn autoplay(&mut self, comp: &mut IntCode) -> Result<ProgramState> {
        let mut state = ProgramState::Ready;
        while state != ProgramState::Halted {
            state = self.run(comp)?;
            if state == ProgramState::Input {
                let balls = self.find_tile_xpos(4);
                let paddles = self.find_tile_xpos(3);
                if balls.iter().max().unwrap() < paddles.iter().min().unwrap() {
                    // ball to left of paddle, move left
                    comp.feed(-1);
                } else if balls.iter().min().unwrap() > paddles.iter().max().unwrap() {
                    // ball to right of paddle, move right
                    comp.feed(1);
                } else {
                    // all good, we're under the paddle
                    comp.feed(0);
                }
            }
        }

        Ok(ProgramState::Halted)
    }
}

pub struct D {}
impl Day for D {
    fn number(&self) -> u8 {
        13
    }
    fn part01(&self) -> Result<()> {
        let prog = load_program_cell("input13.txt")?;
        let mut comp = IntCode::new(&prog);
        let mut cabinet = Cabinet::new();
        if cabinet.run(&mut comp)? == ProgramState::Halted {
            println!("{}", cabinet.view.values().filter(|&&v| v == 2).count());
        } else {
            unimplemented!();
        }
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let mut prog = load_program_cell("input13.txt")?;
        prog[0] = 2; // insert quarters!
        let mut comp = IntCode::new(&prog);
        let mut cabinet = Cabinet::new();
        if cabinet.autoplay(&mut comp)? == ProgramState::Halted {
            println!("{}", cabinet.score);
        } else {
            unimplemented!();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {}
}
