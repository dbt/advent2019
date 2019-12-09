use crate::utils::Result;

use std::fs;

fn count_if(a: &[u8], m: char) -> usize {
    let u = m as u8;
    return a.iter().filter(|x| **x == u).count();
}

pub fn part1() -> Result<String> {
    let data = fs::read_to_string("a08-input")?;
    // 25x6
    let layers = data.as_bytes().chunks(150);
    let mut count = 150;
    let mut ans = 0;

    for l in layers {
        if l.len() < 150 {
            continue;
        }
        let zeroes = count_if(l, '0');
        if zeroes < count {
            count = zeroes;
            ans = count_if(l, '1') * count_if(l, '2');
        }
    }

    return Ok(ans.to_string());
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
    }
}
