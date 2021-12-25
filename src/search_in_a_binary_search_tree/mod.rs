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
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut r = root.clone();
        while let Some(node) = r {
            if node.borrow().val == val {
                return Some(node);
            }
            if node.borrow().val > val {
                r = node.borrow().left.clone();
            } else {
                r = node.borrow().right.clone();
            }
        }
        None
    }
    pub fn level_order_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if root.is_none() {
            return result;
        }
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root);

        while !deque.is_empty() {
            let level_length = deque.len();
            for _ in 0..level_length {
                let n = deque.pop_front();
                if let Some(Some(node)) = n {
                    result.push(node.borrow().val);
                    if node.borrow().left.is_some() {
                        deque.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        deque.push_back(node.borrow().right.clone());
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_bst1() {
        let v = vec![3, 9, 20, 0, 0, 15, 7];
        let t = 2;
        let b_tree = TreeNode::from_vec(v);
        println!("{:#?}", Solution::search_bst(b_tree.clone(), t));
        assert_eq!(Solution::search_bst(b_tree, t), None);
    }

    #[test]
    fn test_search_bst2() {
        let v = vec![4, 2, 7, 1, 3];
        let t = 2;
        let target = vec![2, 1, 3];
        let b_tree = TreeNode::from_vec(v);
        let node = Solution::search_bst(b_tree, t);
        assert_eq!(Solution::level_order_traversal(node), target);
    }
}
