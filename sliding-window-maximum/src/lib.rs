use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 数组为空或者 k 为1时，直接返回原数组
        if nums.len() == 0 || k == 1 {
            return nums;
        }

        let mut res: Vec<i32> = Vec::new();
        let mut deque: VecDeque<i32> = VecDeque::new();
        for i in 0..nums.len() {
            // 弹出队列中所有小于当前值的元素，再将当前值从队尾压入
            push(&mut deque, nums[i]);

            if (i as i32) > k - 1 {
                // 弹出队首元素，让滑动窗口内保持k个数字
                pop(&mut deque, nums[i - k as usize]);

                // 将最大值加入输出数组
                res.push(max(&deque));
            } else if (i as i32) == k - 1 {
                // 将k个元素中的最大值加入输出数组
                res.push(max(&deque));
            }
            println!("{}:{} {:?} -> {:?}", i, k, deque.len(), res);
        }
        return res;
    }
}

fn push(deque: &mut VecDeque<i32>, n: i32) {
    // 当队列不为空且队尾元素小于当前值时，弹出队尾元素，直到队列为空
    while !deque.is_empty() && *deque.back().unwrap() < n {
        deque.pop_back();
    }
    deque.push_back(n);
}

fn pop(deque: &mut VecDeque<i32>, n: i32) {
    // 当队列不为空且队首元素等于传入元素，弹出队首元素
    if !deque.is_empty() && *deque.front().unwrap() == n {
        deque.pop_front();
    }
}

fn max(deque: &VecDeque<i32>) -> i32 {
    // 返回队列中的最大值，即队首元素
    return *deque.front().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let v = vec![1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(
            Solution::max_sliding_window(v.clone(), 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(
            Solution::max_sliding_window(v.clone(), 1),
            vec![1, 3, -1, -3, 5, 3, 6, 7]
        );
    }
}
