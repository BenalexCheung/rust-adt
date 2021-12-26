#![allow(unused)]
struct Solution;

impl Solution {
    pub fn min_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len(); // 行数
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len(); // 列数
        if n == 0 {
            return 0;
        }

        // 存储中间状态，即从开始到网格任一位置matrix[i][j]处的最小路径和
        let mut states = vec![vec![0; n]; m];
        let mut sum = 0;

        // 初始化states的第一列数据，即第一列的边界路径和
        for i in 0..m {
            sum += matrix[i][0];
            states[i][0] = sum;
        }

        sum = 0;
        // 初始化states的第一行数据，即第一行的边界路径和
        for j in 0..n {
            sum += matrix[0][j];
            states[0][j] = sum;
        }

        println!("{:#?}",states);

        // 依次计算states[i][j]
        for i in 1..m {
            for j in 1..n {
                states[i][j] = matrix[i][j] + states[i - 1][j].min(states[i][j - 1]);
            }
        }
        println!("{:#?}",states);
        states[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_path_sum() {
        let input = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let result = 7;
        assert_eq!(Solution::min_path_sum(input), result);
    }
}
