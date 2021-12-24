#![allow(unused)]
struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        // 检验数组是否被旋转
        if nums[right] > nums[0] {
            return nums[0];
        }

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[mid + 1] {
                return nums[mid + 1];
            }

            if nums[mid - 1] > nums[mid] {
                return nums[mid];
            }

            if nums[mid] > nums[0] {
                // 去左边搜索
                left = mid + 1
            } else {
                // 去右边搜索
                right = mid - 1;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_min1() {
        let input = vec![3, 4, 5, 1, 2];
        let result = 1;
        assert_eq!(Solution::find_min(input), result);
    }

    #[test]
    fn test_find_min2() {
        let input = vec![4, 5, 6, 7, 0, 1, 2];
        let result = 0;
        assert_eq!(Solution::find_min(input), result);
    }
}
