use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::error;
use std::fmt;

use crate::utils::read_lines;
use crate::utils::Result;
use adventools::prelude::*;
use anyhow::anyhow;

pub struct D {}
impl Day for D {
    fn number(&self) -> u8 {
        3
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        return Point { x: x, y: y };
    }
    pub fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        return Point::new(self.x + rhs.x, self.y + rhs.y);
    }
}

#[derive(Debug, Clone)]
struct InvalidDirection {
    c: char,
}

impl fmt::Display for InvalidDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid direction: {}", self.c)
    }
}

impl error::Error for InvalidDirection {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn points(path: &String) -> Result<Vec<Point>> {
    let mut out: Vec<Point> = Vec::new();
    let vecs = path.trim().split(",");
    let mut cur = Point::new(0, 0);
    for seg in vecs {
        let ch: char = seg.chars().nth(0).unwrap_or('-');
        let delt: std::result::Result<Point, InvalidDirection> = match ch {
            'U' => Ok(Point::new(0, -1)),
            'D' => Ok(Point::new(0, 1)),
            'L' => Ok(Point::new(-1, 0)),
            'R' => Ok(Point::new(1, 0)),
            c => Err(InvalidDirection { c }),
        };
        let delta = delt?;
        let dist = seg[1..].parse::<i32>()?;
        for _ in 0..dist {
            cur = cur + delta;
            out.push(cur);
        }
    }
    Ok(out)
}

fn intersection(lhs: Vec<Point>, rhs: Vec<Point>) -> Vec<Point> {
    let lh: HashSet<_> = lhs.iter().copied().collect();
    let rh: HashSet<_> = rhs.iter().copied().collect();
    let mut v: Vec<_> = lh.intersection(&rh).copied().collect();
    v.sort_by(|a, b| a.dist().cmp(&b.dist()));
    return v.iter().copied().collect();
}

fn advent03a_dist(path1: String, path2: String) -> Result<i32> {
    let hits = intersection(points(&path1)?, points(&path2)?);
    for h in hits {
        if h.dist() == 0 {
            continue;
        }
        return Ok(h.dist());
    }
    Err(anyhow!("not found"))
}

fn advent03b_dist(path1: String, path2: String) -> Result<usize> {
    let p1 = points(&path1)?;
    let p2 = points(&path2)?;
    let mut m1: HashMap<Point, usize> = HashMap::new();
    for (i, p) in p1.iter().enumerate() {
        m1.entry(*p).or_insert(i + 1);
    }
    let mut best = std::usize::MAX;
    for (i, p) in p2.iter().enumerate() {
        if let Entry::Occupied(o) = m1.entry(*p) {
            let dist = o.get() + i + 1;
            if dist < best {
                best = dist;
            }
        }
    }
    Ok(best)
}

pub fn part1() -> Result<String> {
    let lines = read_lines("input03.txt")?;
    let input1 = lines[0].to_string();
    let input2 = lines[1].to_string();
    advent03a_dist(input1, input2).map(|x| x.to_string())
}

pub fn part2() -> Result<String> {
    let lines = read_lines("input03.txt")?;
    let input1 = lines[0].to_string();
    let input2 = lines[1].to_string();
    advent03b_dist(input1, input2).map(|x| x.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        #[derive(Debug)]
        struct Testcase {
            input: String,
            expected: Vec<Point>,
        }
        let testcases = vec![Testcase {
            input: "R2,D2,L1,U1".to_string(),
            expected: vec![
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(2, 1),
                Point::new(2, 2),
                Point::new(1, 2),
                Point::new(1, 1),
            ],
        }];
        for case in testcases.iter() {
            let result = points(&case.input);
            assert_eq!(result.unwrap(), case.expected);
        }
    }

    #[test]
    fn test_advent03ab_dist() {
        struct Testcase<'a> {
            input1: &'a str,
            input2: &'a str,
            expected1: i32,
            expected2: usize,
        }
        let testcases = vec![
            Testcase {
                input1: "R8,U5,L5,D3",
                input2: "U7,R6,D4,L4",
                expected1: 6,
                expected2: 30,
            },
            Testcase {
                input1: "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                input2: "U62,R66,U55,R34,D71,R55,D58,R83",
                expected1: 159,
                expected2: 610,
            },
            Testcase {
                input1: "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                input2: "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                expected1: 135,
                expected2: 410,
            },
        ];
        for case in testcases.iter() {
            let result = advent03a_dist(case.input1.to_string(), case.input2.to_string());
            assert_eq!(result.unwrap(), case.expected1);
            let resb = advent03b_dist(case.input1.to_string(), case.input2.to_string());
            assert_eq!(resb.unwrap(), case.expected2);
        }
    }
}
