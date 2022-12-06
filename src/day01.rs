use adventools::prelude::*;

fn advent01a_fuel(mass: i32) -> i32 {
    let val = (mass / 3) - 2;
    if val <= 0 {
        return 0;
    }
    return val;
}

fn advent01b_fuel(mass: i32) -> i32 {
    let val = advent01a_fuel(mass);
    if val < 9 {
        return val;
    }
    let addl = advent01b_fuel(val);
    return val + addl;
}

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        1
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

fn part1() -> Result<String> {
    let lines = read_lines("input01.txt")?;
    let mut total: i32 = 0;
    for line in lines {
        let val = line.parse::<i32>()?;
        total += advent01a_fuel(val);
    }
    Ok(total.to_string())
}

fn part2() -> Result<String> {
    let lines = read_lines("input01.txt")?;
    let mut total: i32 = 0;
    for line in lines {
        let val = line.parse::<i32>()?;
        total += advent01b_fuel(val);
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_advent01a_fuel() {
        assert_eq!(advent01a_fuel(12), 2);
        assert_eq!(advent01a_fuel(14), 2);
        assert_eq!(advent01a_fuel(1969), 654);
        assert_eq!(advent01a_fuel(100756), 33583);
    }

    #[test]
    fn test_advent01b_fuel() {
        assert_eq!(advent01b_fuel(12), 2);
        assert_eq!(advent01b_fuel(14), 2);
        assert_eq!(advent01b_fuel(1969), 966);
        assert_eq!(advent01b_fuel(100756), 50346);
    }
}
