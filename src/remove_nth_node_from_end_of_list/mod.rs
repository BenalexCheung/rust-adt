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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow_p = &mut dummy;
        let mut fast_p = &mut slow_p.clone();

        // fast_p向后移动n+1个节点，使得fast_p和slow_p相隔n个节点
        for _ in 1..=n + 1 {
            fast_p = &mut fast_p.as_mut().unwrap().next;
        }

        while fast_p.is_some() {
            fast_p = &mut fast_p.as_mut().unwrap().next;
            slow_p = &mut slow_p.as_mut().unwrap().next;
        }

        // 将slow_p指向的节点的next指针设置为指向下下个节点
        let next = &slow_p.as_mut().unwrap().next.as_mut().unwrap().next;
        slow_p.as_mut().unwrap().next = next.clone();

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_nth_from_end() {
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
        let middle_node = Solution::remove_nth_from_end(Some(Box::new(head.clone())), 2);
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
