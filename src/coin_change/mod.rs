#![allow(unused)]
struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];

        // 初始状态
        dp[0] = 0;

        for i in 1..=amount {
            // 遍历硬币面值，金额i大于硬币面值，代表可以使用此硬币
            for &coin in coins.iter() {
                if i >= coin {
                    let i = i as usize;
                    let coin = coin as usize;
                    dp[i] = dp[i].min(dp[i - coin] + 1);
                }
            }
        }
        println!("{:#?}", dp);
        let last = *dp.last().unwrap();
        if last > amount {
            -1
        } else {
            last
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coin_change1() {
        let input = vec![1, 2, 5];
        let n = 11;
        let result = 3;
        assert_eq!(Solution::coin_change(input, n), result);
    }

    #[test]
    fn test_coin_change2() {
        let input = vec![2];
        let n = 3;
        let result = -1;
        assert_eq!(Solution::coin_change(input, n), result);
    }
}
