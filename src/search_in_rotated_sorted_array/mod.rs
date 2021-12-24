#![allow(unused)]
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[left] <= nums[mid] {
                // 前半部分有序，在前半部分查找
                if target >= nums[left] && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // 后半部分有序，在后半部分查找
                if target > nums[mid] && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search1() {
        let input = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let result = 4;
        assert_eq!(Solution::search(input, target), result);
    }

    #[test]
    fn test_search2() {
        let input = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let result = -1;
        assert_eq!(Solution::search(input, target), result);
    }
}
