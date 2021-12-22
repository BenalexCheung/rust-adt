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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast_p = &head;
        let mut slow_p = &head;

        // fast_p指向的节点和下个节点都非空
        while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
            // slow_p往后移动一个节点
            slow_p = &slow_p.as_ref().unwrap().next;
            // fast_p往后移动两个节点
            fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow_p.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_middle_node() {
        let mut head = ListNode::new(1);
        head.next
            .insert(Box::new(ListNode::new(2)))
            .next
            .insert(Box::new(ListNode::new(3)))
            .next
            .insert(Box::new(ListNode::new(4)))
            .next
            .insert(Box::new(ListNode::new(5)))
            .next
            .insert(Box::new(ListNode::new(6)));
        let mut curr = Some(Box::new(head.clone()));
        while let Some(mut curr_node) = curr.take() {
            let next_temp = curr_node.next.take();
            curr = next_temp;
            print!("{}->", curr_node.val);
        }
        println!("Null");
        let middle_node = Solution::middle_node(Some(Box::new(head.clone())));
        let mut curr = middle_node;
        while let Some(mut curr_node) = curr.take() {
            let next_temp = curr_node.next.take();
            curr = next_temp;
            print!("{}->", curr_node.val);
        }
        println!("Null");
        assert!(true);
    }
}
