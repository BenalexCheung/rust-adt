#![allow(unused)]
struct Solution;

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let len = nums.len();
        let mut tmp = vec![0; len];
        return sort_count(&mut nums, &mut tmp, 0, len - 1) as i32;
    }
}

fn sort_count(nums: &mut Vec<i32>, tmp: &mut Vec<i32>, left: usize, right: usize) -> usize {
    if left >= right {
        return 0;
    }
    let middle = left + (right - left) / 2;

    // 先统计左、右子数组的重要翻转对数量
    let mut count = sort_count(nums, tmp, left, middle) + sort_count(nums, tmp, middle + 1, right);

    // 再统计左、右子数组之间的重要翻转对数量
    let mut i = left;
    let mut j = middle + 1;
    while i <= middle && j <= right {
        // 满足nums[i]>2*nums[j]，增加重要翻转对数量，并让j向后移动
        // 否则，让i向后移动
        if nums[i] as i64 > 2 * nums[j] as i64 {
            count += middle - i + 1;
            j += 1;
        } else {
            i += 1;
        }
    }

    merge(nums, tmp, left, middle, right);
    count
}

// 将两个已经有序的数组合并
fn merge(nums: &mut Vec<i32>, tmp: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
    let mut index = 0;
    let mut i = left;
    let mut j = middle + 1;

    while i <= middle && j <= right {
        if nums[i] <= nums[j] {
            tmp[index] = nums[i];
            index += 1;
            i += 1;
        } else {
            tmp[index] = nums[j];
            index += 1;
            j += 1;
        }
    }

    while i <= middle {
        tmp[index] = nums[i];
        index += 1;
        i += 1;
    }

    while j <= right {
        tmp[index] = nums[j];
        index += 1;
        j += 1;
    }

    for i in left..=right {
        nums[i] = tmp[i - left];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_pairs1() {
        let nums = vec![1, 3, 2, 3, 1];
        let result = 2;
        assert_eq!(Solution::reverse_pairs(nums), result);
    }

    #[test]
    fn test_reverse_pairs2() {
        let nums = vec![2, 4, 3, 5, 1];
        let result = 3;
        assert_eq!(Solution::reverse_pairs(nums), result);
    }
}
