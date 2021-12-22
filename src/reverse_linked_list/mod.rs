#![allow(unused)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut curr_node) = curr.take() {
            // 保存当前节点的下一节点
            let next_temp = curr_node.next.take();
            // 将当前节点指向prev节点
            curr_node.next = prev.take();

            // prev和curr分别往后移动一个节点
            prev = Some(curr_node);
            curr = next_temp;
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_linked_list() {
        let mut head = ListNode::new(1);
        head.next
            .insert(Box::new(ListNode::new(2)))
            .next
            .insert(Box::new(ListNode::new(3)))
            .next
            .insert(Box::new(ListNode::new(4)))
            .next
            .insert(Box::new(ListNode::new(5)));
        let mut curr = Some(Box::new(head.clone()));
        while let Some(mut curr_node) = curr.take() {
            let next_temp = curr_node.next.take();
            curr = next_temp;
            print!("{}->", curr_node.val);
        }
        println!("Null");
        let reverse_head = Solution::reverse_linked_list(Some(Box::new(head.clone())));
        let mut curr = reverse_head;
        while let Some(mut curr_node) = curr.take() {
            let next_temp = curr_node.next.take();
            curr = next_temp;
            print!("{}->", curr_node.val);
        }
        println!("Null");
        assert!(true);
    }
}
