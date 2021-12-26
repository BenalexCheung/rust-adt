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
    /// 递归构造二叉树
    fn from_vec1(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = build_tree_recursion(&v, 0, v.len());
        root
    }
    /// 迭代构造二叉树
    fn from_vec2(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = v;
        let len = v.len();
        let root = build_tree_iteration(&mut v, 0, len);
        root
    }
}

/// 假设要构造的二叉树的层序遍历序列存在一个数组里
/// 注意：需要补充空节点
fn build_tree_recursion(v: &Vec<i32>, i: usize, n: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if i >= n || v[i] == 0 {
        return None;
    }
    let mut root = TreeNode::new(v[i]);
    root.left = build_tree_recursion(v, 2 * i + 1, n); // 1, 3, 5
    root.right = build_tree_recursion(v, 2 * i + 2, n); // 2, 4, 6

    return Some(Rc::new(RefCell::new(root)));
}

/// 假设要构造的二叉树的层序遍历序列存在一个数组里
/// 1. 只要数组不为空，就先入队数组首元素，并用这个值创建二叉树的root。
/// 2. 然后进入循环，队列不为空，就拿队头元素，队头再出队。队列为空，结束循环。
/// 3. 只要数组还有元素，就先给刚刚拿出的队头元素创建左孩子，然后左孩子入队。
/// 4. 同上，再创建右孩子，右孩子入队。
/// 5. 结束一次循环。回到步骤2
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if root.is_none() {
            return result;
        }
        // preorder_traversal1(root, &mut result);
        preorder_traversal2(root, &mut result);
        result
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if root.is_none() {
            return result;
        }
        // inorder_traversal1(root, &mut result);
        inorder_traversal2(root, &mut result);
        result
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if root.is_none() {
            return result;
        }
        // postorder_traversal1(root, &mut result);
        postorder_traversal2(root, &mut result);
        result
    }

    pub fn level_order_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if root.is_none() {
            return result;
        }
        level_order_traversal2(root, &mut result);
        result
    }
}

fn preorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            // 访问当前节点
            result.push(node.borrow().val);
            // 递归遍历左子树
            preorder_traversal1(node.borrow().left.clone(), result);
            // 递归遍历右子树
            preorder_traversal1(node.borrow().right.clone(), result);
        }
        None => (),
    }
}

/// 前序遍历
/// 1. 创建一个栈用来存放节点
/// 2. 若当前节点非空，访问当前节点值，再将当前节点入栈，并进入其左子树访问
/// 3. 重复步骤2，直到当前节点为空
/// 4. 将栈顶的节点出栈，并进入其右子树访问
/// 5. 重复步骤2~4，直到当前节点为空且栈为空，完成所有节点的访问
fn preorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }

    // 使用栈来保存需要返回后处理的节点
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut r = root.clone();

    // 满足当前节点非空或者栈非空时执行循环
    while r.is_some() || !stack.is_empty() {
        while let Some(node) = r {
            result.push(node.borrow().val);
            stack.push(node.clone());
            r = node.borrow().left.clone();
        }

        r = stack.pop();
        if let Some(node) = r {
            r = node.borrow().right.clone();
        }
    }
}

fn inorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            // 递归遍历左子树
            inorder_traversal1(node.borrow().left.clone(), result);
            // 访问当前节点
            result.push(node.borrow().val);
            // 递归遍历右子树
            inorder_traversal1(node.borrow().right.clone(), result);
        }
        None => (),
    }
}

/// 中序遍历
/// 1. 创建一个栈用来存放节点
/// 2. 若当前节点非空，再将当前节点入栈，并进入其左子树访问
/// 3. 重复步骤2，直到当前节点为空
/// 4. 将栈顶的节点出栈，访问当前节点值，并进入其右子树访问
/// 5. 重复步骤2~4，直到当前节点为空且栈为空，完成所有节点的访问
fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }

    // 使用栈来保存需要返回后处理的节点
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut r = root.clone();

    // 满足当前节点非空或者栈非空时执行循环
    while r.is_some() || !stack.is_empty() {
        while let Some(node) = r {
            stack.push(node.clone());
            r = node.borrow().left.clone();
        }

        r = stack.pop();
        if let Some(node) = r {
            result.push(node.borrow().val);
            r = node.borrow().right.clone();
        }
    }
}

fn postorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            // 递归遍历左子树
            postorder_traversal1(node.borrow().left.clone(), result);
            // 递归遍历右子树
            postorder_traversal1(node.borrow().right.clone(), result);
            // 访问当前节点
            result.push(node.borrow().val);
        }
        None => (),
    }
}

/// 后续遍历
/// 1. 创建stack1、stack2两个栈来存放节点，并先将根节点入stack1
/// 2. 让stack1栈顶的节点出栈，将该节点入stack2，同时将该节点的左右子节点入栈stack1
/// 3. 重复步骤2，直到stack1为空
/// 4. 将stack2栈顶的节点出栈，访问该节点
/// 5. 重复步骤4，直到stack2为空，完成所有节点的访问
fn postorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }

    let mut stack1: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let mut stack2: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    stack1.push(root);

    // 将stack1栈顶的节点依次出栈，并将该节点入stack2，将该节点的左右子节点入stack1
    while let Some(Some(node)) = stack1.pop() {
        if node.borrow().left.is_some() {
            stack1.push(node.borrow().left.clone());
        }
        if node.borrow().right.is_some() {
            stack1.push(node.borrow().right.clone());
        }
        stack2.push(Some(node));
    }

    while let Some(Some(node)) = stack2.pop() {
        result.push(node.borrow().val);
    }
}

// todo: Achieve the level of binary tree traversal
fn level_order_traversal1(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => (),
        None => (),
    }
}

/// 层序遍历
/// 1. 树不为空，root先入队
/// 2. 进入循环，队列不为空，则拿到队头元素，队头出队。队列为空，结束循环。
/// 3. 打印刚刚队头元素的数据。
/// 4. 它如果存在左孩子，左孩子入队。
/// 5. 它如果存在右孩子，右孩子入队。
/// 6. 结束一次循环，回到步骤2
fn level_order_traversal2(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if root.is_none() {
        return;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_preorder_traversal() {
        let v = vec![1, 0, 2, 0, 0, 3];
        let t = vec![1, 2, 3];
        let b_tree = TreeNode::from_vec1(v);
        assert_eq!(Solution::preorder_traversal(b_tree), t);
    }

    #[test]
    fn test_inorder_traversal() {
        let v = vec![1, 0, 2, 0, 0, 3];
        let t = vec![1, 3, 2];
        let b_tree = TreeNode::from_vec1(v);
        assert_eq!(Solution::inorder_traversal(b_tree), t);
    }

    #[test]
    fn test_postorder_traversal() {
        let v = vec![1, 0, 2, 0, 0, 3];
        let t = vec![3, 2, 1];
        let b_tree = TreeNode::from_vec1(v);
        assert_eq!(Solution::postorder_traversal(b_tree), t);
    }

    #[test]
    fn test_level_order_traversal() {
        let v = vec![3, 9, 20, 0, 0, 15, 7];
        let t = vec![3, 9, 20, 15, 7];
        let b_tree = TreeNode::from_vec1(v);
        assert_eq!(Solution::level_order_traversal(b_tree), t);
    }
    #[test]
    fn test_preorder_traversal2() {
        let v = vec![1, 0, 2, 3];
        let t = vec![1, 2, 3];
        let b_tree = TreeNode::from_vec2(v);
        assert_eq!(Solution::preorder_traversal(b_tree), t);
    }

    #[test]
    fn test_inorder_traversal2() {
        let v = vec![1, 0, 2, 3];
        let t = vec![1, 3, 2];
        let b_tree = TreeNode::from_vec2(v);
        assert_eq!(Solution::inorder_traversal(b_tree), t);
    }

    #[test]
    fn test_postorder_traversal2() {
        let v = vec![1, 0, 2, 3];
        let t = vec![3, 2, 1];
        let b_tree = TreeNode::from_vec2(v);
        assert_eq!(Solution::postorder_traversal(b_tree), t);
    }

    #[test]
    fn test_level_order_traversal2() {
        let v = vec![3, 9, 20, 0, 0, 15, 7];
        let t = vec![3, 9, 20, 15, 7];
        let b_tree = TreeNode::from_vec2(v);
        assert_eq!(Solution::level_order_traversal(b_tree), t);
    }
}
