#![allow(unused)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn from_vec(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = v;
        let len = v.len();
        let root = build_tree_iteration(&mut v, 0, len);
        root
    }
}

fn build_tree_iteration(v: &mut Vec<i32>, i: usize, n: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if v.is_empty() {
        return None;
    }
    let mut root = TreeNode::new(*v.first().unwrap());
    v.remove(0);

    let root_node = Rc::new(RefCell::new(root));
    let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    deque.push_back(Some(Rc::clone(&root_node)));

    while !deque.is_empty() {
        let n = deque.pop_front();
        if let Some(Some(node)) = n {
            if let Some(i) = v.first() {
                let tmp = *i;
                v.remove(0);
                if tmp != 0 {
                    let left_child = TreeNode::new(tmp);
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(left_child)));
                    deque.push_back(node.borrow_mut().left.clone());
                }
            }
            if let Some(i) = v.first() {
                let tmp = *i;
                v.remove(0);
                if tmp != 0 {
                    let right_child = TreeNode::new(tmp);
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(right_child)));
                    deque.push_back(node.borrow_mut().right.clone());
                }
            }
        }
    }
    return Some(root_node);
}

struct Solution;

impl Solution {
    pub fn min_depth_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        // preorder_traversal1(root, &mut result);
        min_depth_dfs(root)
    }
}

fn min_depth_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            if node.borrow().left.is_none() {
                return min_depth_dfs(node.borrow().left.clone()) + 1;
            }
            if node.borrow().right.is_none() {
                return min_depth_dfs(node.borrow().right.clone()) + 1;
            }
            let left = min_depth_dfs(node.borrow().left.clone());
            let right = min_depth_dfs(node.borrow().right.clone());
            1 + left.min(right)
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_depth_dfs() {
        let v = vec![3, 9, 20, 0, 0, 15, 7];
        let t = 2;
        let b_tree = TreeNode::from_vec(v);
        assert_eq!(Solution::min_depth_dfs(b_tree), t);
    }
}
