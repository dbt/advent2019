use crate::utils::Result;
use std::collections::HashMap;
use std::fs;

pub fn part1() -> Result<String> {
    let all = fs::read_to_string("a06-input")?;
    let mut queue = vec!["COM"];
    let mut next_queue: Vec<&str> = Vec::new();
    let mut dist = 0;
    let mut total = 0;
    while !queue.is_empty() {
        dist += 1;
        for line in all.split("\n") {
            if line.len() < 1 {
                break;
            }
            let mut spl = line.split(")");
            let inner = spl.next().ok_or(format!("invalid {}", line))?;
            let outer = spl.next().ok_or(format!("invalid {}", line))?;
            if queue.contains(&inner) {
                total += dist;
                next_queue.push(outer);
            }
        }
        queue = next_queue;
        next_queue = vec![];
    }
    Ok(total.to_string())
}

pub fn part2() -> Result<String> {
    let all = fs::read_to_string("a06-input")?;
    let all_lines = all.split("\n");
    let mut parents: HashMap<&str, &str> = HashMap::new();
    for line in all_lines {
        if line.len() < 1 {
            break;
        }
        let mut spl = line.split(")");
        let inner = spl.next().ok_or(format!("invalid {}", line))?;
        let outer = spl.next().ok_or(format!("invalid {}", line))?;
        parents.insert(outer, inner);
    }
    let mut counter = -1;
    let mut ynext = Some(&"YOU");
    let mut snext = Some(&"SAN");
    let mut ydist: HashMap<&str, i32> = HashMap::new();
    let mut sdist: HashMap<&str, i32> = HashMap::new();
    while ynext.is_some() && snext.is_some() {
        if let Some(you) = ynext {
            ydist.insert(you, counter);
            if let Some(x) = sdist.get(you) {
                return Ok((x + counter).to_string());
            }
            ynext = parents.get(you);
        }
        if let Some(san) = snext {
            sdist.insert(san, counter);
            if let Some(x) = ydist.get(san) {
                return Ok((x + counter).to_string());
            }
            snext = parents.get(san);
        }
        counter += 1;
    }
    Err("no matches found")?
}
