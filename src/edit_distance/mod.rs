#![allow(unused)]
struct Solution;

impl Solution {
    pub fn min_distance(world1: String, world2: String) -> i32 {
        let world1_chars: Vec<char> = world1.chars().collect();
        let world2_chars: Vec<char> = world2.chars().collect();

        let n1 = world1.len();
        let n2 = world2.len();

        let mut dp = vec![vec![0; n2 + 1]; n1 + 1];

        // 初始化第一行，执行插入操作
        for j in 1..=n2 {
            dp[0][j] = dp[0][j - 1] + 1;
        }

        // 初始化第一列，执行删除操作
        for i in 1..=n1 {
            dp[i][0] = dp[i - 1][0] + 1;
        }

        for i in 1..=n1 {
            for j in 1..=n2 {
                if world1_chars[i - 1] == world2_chars[j - 1] {
                    // 相等时不需要任何操作
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    // 不相等时取替换、插入、删除3中操作的最小值加1
                    dp[i][j] = dp[i][j - 1].min(dp[i - 1][j].min(dp[i - 1][j - 1])) + 1;
                }
            }
        }
        dp[n1][n2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_distance1() {
        let world1 = String::from("horse");
        let world2 = String::from("ros");
        let result = 3;
        assert_eq!(Solution::min_distance(world1, world2), result);
    }

    #[test]
    fn test_min_distance2() {
        let world1 = String::from("intention");
        let world2 = String::from("execution");
        let result = 5;
        assert_eq!(Solution::min_distance(world1, world2), result);
    }
}
