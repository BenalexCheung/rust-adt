#![allow(unused)]
use rand::Rng;

struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() || k > nums.len() as i32 {
            return -1;
        }
        let len = nums.len();
        return quick_select(&mut nums, 0, len - 1, len - k as usize);
    }
}

fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, k_smallest: usize) -> i32 {
    // 判断是否只剩下一个元素
    if left == right {
        return nums[left];
    }

    // 在区间[left,right]中随机算则一个元素作为基准值
    let mut rng = rand::thread_rng();
    let mut pivot_index = left + rng.gen_range(0, right - left);

    // 使用partition函数找到分区点
    let pivot_index = partition(nums, left, right, pivot_index);

    // 对分区点左子数组和右子数组进行递归操作
    return if k_smallest == pivot_index {
        nums[k_smallest]
    } else if k_smallest < pivot_index {
        // 在区间左边寻找
        quick_select(nums, left, pivot_index - 1, k_smallest)
    } else {
        // 在区间右边寻找
        quick_select(nums, pivot_index + 1, right, k_smallest)
    };
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize, pivot_index: usize) -> usize {
    // 设定基准值
    let pivot = nums[pivot_index];

    // 将分区点移至末端
    nums.swap(pivot_index, right);

    // 将所有小于基准值的元素向左移动
    let mut store_index = left;
    for j in left..right {
        if nums[j] < pivot {
            // 小于基准值的元素交换到左边
            nums.swap(store_index, j);
            store_index += 1;
        }
    }

    // 将分区点移至最终位置
    nums.swap(store_index, right);
    store_index
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_kth_largest1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let result = 5;
        assert_eq!(Solution::find_kth_largest(nums, k), result);
    }

    #[test]
    fn test_find_kth_largest2() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let result = 4;
        assert_eq!(Solution::find_kth_largest(nums, k), result);
    }
}
