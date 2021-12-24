#![allow(unused)]
struct Solution;

impl Solution {
    pub fn generate_parentheses(n: i32) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        recursion(&mut vec, 0, 0, n, String::from(""));
        return vec;
    }
}

pub fn recursion(vec: &mut Vec<String>, left: i32, right: i32, n: i32, s: String) {
    // 左括号和右括号都为n时添加这个答案
    if left == n && right == n {
        vec.push(s.clone());
    }

    // 左括号个数小于n，可继续加左括号
    if left < n {
        recursion(vec, left + 1, right, n, format!("{}{}", &s, "("));
    }

    // 左括号个数大于右括号个数，可继续加右括号
    if right < left {
        recursion(vec, left, right + 1, n, format!("{}{}", &s, ")"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_parentheses() {
        let n = 3;
        let result = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(Solution::generate_parentheses(n), result);
    }
}
