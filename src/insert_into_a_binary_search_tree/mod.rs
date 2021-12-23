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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        // preorder_traversal1(root, &mut result);
        insert(&root, val);
        root
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if root.is_none() {
            return result;
        }
        inorder_traversal(root, &mut result);
        result
    }
}

fn insert(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if let Some(node) = root {
        let mut n = node.borrow_mut();

        // val大于当前节点值，往右子树查找
        // val小于当前节点值，往左子树查找
        let target = if val > n.val {
            &mut n.right
        } else {
            &mut n.left
        };
        if target.is_some() {
            return insert(target, val);
        }
        // 在找到的空节点位置插入
        *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            // 递归遍历左子树
            inorder_traversal(node.borrow().left.clone(), result);
            // 访问当前节点
            result.push(node.borrow().val);
            // 递归遍历右子树
            inorder_traversal(node.borrow().right.clone(), result);
        }
        None => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_preorder_traversal() {
        let v = vec![4, 2, 7, 1, 3];
        let t = vec![1, 2, 3, 4, 5, 7];
        let b_tree = TreeNode::from_vec(v);
        let bst = Solution::insert_into_bst(b_tree, 5);
        assert_eq!(Solution::inorder_traversal(bst), t);
    }
}
