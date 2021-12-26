#![allow(unused)]
struct Solution;

impl Solution {
    pub fn climbing_stairs(n: i32) -> i32 {
        let mut memo: Vec<i32> = vec![0; n as usize];
        return recursion(n as usize, &mut memo);
    }
}

// 动态规划
fn climbing_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..(n + 1) as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    return dp[n as usize];
}

// 递归
fn recursion(n: usize, memo: &mut Vec<i32>) -> i32 {
    if n <= 2 {
        return n as i32;
    }

    if memo[n - 1] == 0 {
        memo[n - 1] = recursion(n - 1, memo);
    }

    if memo[n - 2] == 0 {
        memo[n - 2] = recursion(n - 2, memo);
    }

    return memo[n - 1] + memo[n - 2];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_climbing_stairs1() {
        let input = 2;
        let result = 2;
        assert_eq!(Solution::climbing_stairs(input), result);
    }

    #[test]
    fn test_climbing_stairs2() {
        let input = 4;
        let result = 5;
        assert_eq!(Solution::climbing_stairs(input), result);
    }

    #[test]
    fn test_climbing_stairs3() {
        let input = 2;
        let result = 2;
        assert_eq!(climbing_stairs(input), result);
    }

    #[test]
    fn test_climbing_stairs4() {
        let input = 4;
        let result = 5;
        assert_eq!(climbing_stairs(input), result);
    }
}
