use adventools::prelude::*;
use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::Ordering, str::FromStr};

use num::integer::lcm;

#[derive(Copy, Clone)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    dx: i32,
    dy: i32,
    dz: i32,
}

fn delta(s: i32, o: &i32) -> i32 {
    match s.cmp(o) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}
impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            x,
            y,
            z,
            dx: 0,
            dy: 0,
            dz: 0,
        }
    }

    fn apply_gravity(&mut self, other: &Self) {
        self.dx += delta(self.x, &other.x);
        self.dy += delta(self.y, &other.y);
        self.dz += delta(self.z, &other.z);
    }

    fn apply_deltas(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }

    fn energy(&self) -> i32 {
        let pot = self.x.abs() + self.y.abs() + self.z.abs();
        let kin = self.dx.abs() + self.dy.abs() + self.dz.abs();
        pot * kin
    }
}

impl FromStr for Moon {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
        }
        match RE.captures(&value) {
            None => Err(anyhow!("Invalid input: '{}'", value)),
            Some(c) => Ok(Moon::new(
                c["x"].parse().unwrap(),
                c["y"].parse().unwrap(),
                c["z"].parse().unwrap(),
            )),
        }
    }
}

pub struct D {}
impl Day for D {
    fn number(&self) -> u8 {
        12
    }
    fn part01(&self) -> Result<()> {
        let mut moons = read_lines("input12.txt")?
            .iter()
            .map(|s| s.parse::<Moon>())
            .collect::<Result<_>>()?;
        process(&mut moons, 1000);
        let e: i32 = moons.iter().map(|m| m.energy()).sum();
        println!("{}", e);
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let moons = read_lines("input12.txt")?
            .iter()
            .map(|s| s.parse::<Moon>())
            .collect::<Result<_>>()?;
        println!("{}", find_cycle_time(moons));
        Ok(())
    }
}

fn process(moons: &mut Vec<Moon>, iter: usize) {
    for _ in 0..iter {
        let others: Vec<_> = moons.iter().copied().collect();
        for m in moons.iter_mut() {
            for o in others.iter() {
                m.apply_gravity(o);
            }
        }
        for m in moons.iter_mut() {
            m.apply_deltas();
        }
    }
}

fn find_cycle_time(mut moons: Vec<Moon>) -> usize {
    let mut xloop = 0;
    let mut yloop = 0;
    let mut zloop = 0;
    let xstart: String = moons.iter().map(|m| format!("{},{},", m.x, m.dx)).collect();
    let ystart: String = moons.iter().map(|m| format!("{},{},", m.y, m.dy)).collect();
    let zstart: String = moons.iter().map(|m| format!("{},{},", m.z, m.dz)).collect();

    let mut counter = 0;

    while xloop == 0 || yloop == 0 || zloop == 0 {
        counter += 1;
        process(&mut moons, 1);
        if xloop == 0 {
            let xhash: String = moons.iter().map(|m| format!("{},{},", m.x, m.dx)).collect();
            if xhash == xstart {
                xloop = counter;
            }
        }
        if yloop == 0 {
            let yhash: String = moons.iter().map(|m| format!("{},{},", m.y, m.dy)).collect();
            if yhash == ystart {
                yloop = counter;
            }
        }
        if zloop == 0 {
            let zhash: String = moons.iter().map(|m| format!("{},{},", m.z, m.dz)).collect();
            if zhash == zstart {
                zloop = counter;
            }
        }
    }

    lcm(lcm(xloop, yloop), zloop)
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_moon() {
        let mut main = Moon::new(0, 2, 4);
        let other = Moon::new(2, 2, 2);
        main.apply_gravity(&other);
        assert_eq!(main.dx, 1);
        assert_eq!(main.dy, 0);
        assert_eq!(main.dz, -1);
        main.apply_deltas();
        assert_eq!(main.x, 1);
        assert_eq!(main.y, 2);
        assert_eq!(main.z, 3);
    }

    fn test_data() -> Vec<Moon> {
        vec![
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ]
        .iter()
        .map(|s| s.parse::<Moon>())
        .collect::<Result<_>>()
        .unwrap()
    }

    fn test_data2() -> Vec<Moon> {
        r"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"
            .split('\n')
            .map(|s| s.parse())
            .collect::<Result<_>>()
            .unwrap()
    }

    #[test]
    fn test_p1() {
        let mut moons = test_data();
        process(&mut moons, 10);
        let e: i32 = moons.iter().map(|m| m.energy()).sum();
        assert_eq!(e, 179);
    }

    #[test]
    fn test_p2() {
        let mut moons = test_data();
        assert_eq!(find_cycle_time(moons), 2772);
        moons = test_data2();
        assert_eq!(find_cycle_time(moons), 4686774924);
    }
}
