#![allow(unused)]
struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut n = n;

        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        return fast_pow(x, n);
    }
}

pub fn fast_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let half = fast_pow(x, n / 2);
    return if n % 2 == 0 {
        half * half
    } else {
        half * half * x
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_pow1() {
        let input = (2.00000, 10);
        let result = 1024.00000;
        assert!((Solution::my_pow(input.0, input.1) - result).abs() < 1e-10);
    }

    #[test]
    fn test_my_pow2() {
        let input = (2.10000, 3);
        let result = 9.26100;
        assert!((Solution::my_pow(input.0, input.1) - result).abs() < 1e-10);
    }

    #[test]
    fn test_my_pow3() {
        let input = (2.00000, -2);
        let result = 0.25000;
        assert!((Solution::my_pow(input.0, input.1)).abs() - result < 1e-10);
    }
}
