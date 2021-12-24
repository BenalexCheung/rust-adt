#![allow(unused)]
struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 0 || num == 1 {
            return true;
        }

        let mut left = 2;
        let mut right = num / 2;
        while left <= right {
            let mid = left + (right - left) / 2;
            let guess_square = mid as i64 * mid as i64;

            if guess_square == num as i64 {
                return true;
            } else if guess_square > num as i64 {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_perfect_square1() {
        let input = 16;
        let result = true;
        assert_eq!(Solution::is_perfect_square(input), result);
    }

    #[test]
    fn test_is_perfect_square2() {
        let input = 14;
        let result = false;
        assert_eq!(Solution::is_perfect_square(input), result);
    }
}
