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
    pub fn merge_two_lists(
        listnode1: Option<Box<ListNode>>,
        listnode2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (listnode1, listnode2) {
            (Some(node1), None) => Some(node1), // listnode2为空，返回listnode1的其余节点
            (None, Some(node2)) => Some(node2), // listnode1为空，返回listnode2的其余节点
            (Some(mut node1), Some(mut node2)) => {
                // 如果listnode1指向的节点值小于listnode2指向的节点值，listnode1指向的节点的下一个节点就是递归函数的返回值
                // 反之亦然
                if node1.val < node2.val {
                    let n = node1.next.take();
                    node1.next = Solution::merge_two_lists(n, Some(node2));
                    Some(node1)
                } else {
                    let n = node2.next.take();
                    node2.next = Solution::merge_two_lists(Some(node1), n);
                    Some(node2)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_two_lists() {
        let mut listnode1 = ListNode::new(1);
        listnode1
            .next
            .insert(Box::new(ListNode::new(2)))
            .next
            .insert(Box::new(ListNode::new(4)));
        let mut listnode2 = ListNode::new(1);
        listnode2
            .next
            .insert(Box::new(ListNode::new(3)))
            .next
            .insert(Box::new(ListNode::new(4)));
        let mut listnode1 = Some(Box::new(listnode1.clone()));
        let mut listnode2 = Some(Box::new(listnode2.clone()));
        let listnode = Solution::merge_two_lists(listnode1, listnode2);
        let mut curr = listnode;
        while let Some(mut curr_node) = curr.take() {
            let next_temp = curr_node.next.take();
            curr = next_temp;
            print!("{}->", curr_node.val);
        }
        println!("Null");
        assert!(true);
    }
}
