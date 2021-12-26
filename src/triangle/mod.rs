#![allow(unused)]
struct Solution;

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 0 {
            return 0;
        }

        // 从倒数第二行开始依次向上遍历
        for i in (0..triangle.len() - 1).rev() {
            for j in 0..triangle[i].len() {
                triangle[i][j] = triangle[i][j] + triangle[i + 1][j].min(triangle[i + 1][j + 1]);
            }
        }
        println!("{:#?}", triangle);
        triangle[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_total() {
        let input = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let result = 11;
        assert_eq!(Solution::minimum_total(input), result);
    }
}
