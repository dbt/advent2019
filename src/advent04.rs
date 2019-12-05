
use crate::utils::Result;

fn unpack(i: i32) -> (i32, i32, i32, i32, i32, i32) {
    (i / 100000 % 10, i / 10000 % 10, i / 1000 % 10, i / 100 % 10, i/10 % 10, i % 10)
}

fn advent04a_test(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> bool {
    a <= b && b <= c && c <= d && d <= e && e <= f && (
        a == b || b == c || c == d || d == e || e == f
    )
}

fn advent04b_test(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> bool {
    a <= b && b <= c && c <= d && d <= e && e <= f && (
        (a == b && b != c) || 
        (b == c && a != b && c != d) || 
        (c == d && b != c && d != e) || 
        (d == e && c != d && e != f) || 
        (e == f && d != e) 
    )
}

pub fn advent04a() -> Result<String> {
    let mut counter = 0;
    for i in 171309..=643603 {
        let (a, b, c, d, e, f) = unpack(i);
        if advent04a_test(a, b, c, d, e, f) {
            counter = counter + 1;
        }
    }
    Ok(counter.to_string())
}

pub fn advent04b() -> Result<String> {
    let mut counter = 0;
    for i in 171309..=643603 {
        let (a, b, c, d, e, f) = unpack(i);
        if advent04b_test(a, b, c, d, e, f) {
            counter = counter + 1;
        }
    }
    Ok(counter.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unpack() {
        assert_eq!(unpack(123456), (1, 2, 3, 4, 5, 6));
    }

    #[test]
    fn test_advent04a_test() {
        assert_eq!(advent04a_test(1,1,1,1,1,1), true);
        assert_eq!(advent04a_test(2,2,3,4,5,0), false);
        assert_eq!(advent04a_test(1,2,3,7,8,9), false);
    }

    fn test_advent04b_test() {
        assert_eq!(advent04b_test(1,1,2,2,3,3), true);
        assert_eq!(advent04b_test(1,2,3,4,4,4), false);
        assert_eq!(advent04b_test(1,1,1,1,2,2), true);
    }
}
