#![allow(unused)]
struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut dp = vec![1; nums.len()];

        // res记录所有计算出dp[i]的最大值
        let mut res = 1;

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            res = res.max(dp[i]);
        }
        println!("{:#?}", dp);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_lis() {
        let input = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let result = 4;
        assert_eq!(Solution::length_of_lis(input), result);
    }
}
