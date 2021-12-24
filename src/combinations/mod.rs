#![allow(unused)]
struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if n < k {
            return Vec::new();
        }

        let mut vecs: Vec<Vec<i32>> = Vec::new();
        let mut vec: Vec<i32> = Vec::new();
        backtrack(&mut vecs, &mut vec, n, k, 1);
        return vecs;
    }
}

pub fn backtrack(vecs: &mut Vec<Vec<i32>>, vec: &mut Vec<i32>, n: i32, k: i32, start: usize) {
    // 终止递归条件
    if vec.len() == k as usize {
        vecs.push(vec.clone());
        return;
    }

    // i表示当前层的取值，balance表示剩余空间
    // 初始balance=k，当balance=0时，代表已取到k个数
    // 规律：n = max(i) + balance -1

    let mut i = start;
    // 剪枝操作
    while i <= (n - (k - vec.len() as i32) + 1) as usize {
        vec.push(i as i32);
        backtrack(vecs, vec, n, k, i + 1);
        vec.remove(vec.len() - 1);
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_combinations() {
        let input = (4, 2);
        let result = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        assert_eq!(Solution::combine(input.0, input.1), result);
    }
}
