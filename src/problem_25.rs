//! Reverse Nodes in k-Group [link](https://leetcode.com/problems/reverse-nodes-in-k-group/)
//!
//! Given a linked list, reverse the nodes of a linked list *k* at a time and return its modified list.
//!
//! *k* is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes in the end should remain as it is.
//!
//! **Example:**
//!
//! Given this linked list: `1->2->3->4->5`
//!
//! For *k* = 2, you should return: `2->1->4->3->5`
//!
//! For *k* = 3, you should return: `3->2->1->4->5`
//!
//! **Note:**
//!
//! - Only constant extra memory is allowed.
//! - You may not alter the values in the list's nodes, only nodes itself may be changed.
use crate::prelude::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = dummy.as_mut();
        loop {
            let mut cur = prev.as_mut().unwrap().next.take();
            if cur.is_none() {
                break;
            }
            // detect if there is k nodes in this group
            let mut rear = cur.as_mut();
            for _ in 1..k {
                rear = rear.unwrap().next.as_mut();
                if rear.is_none() {
                    prev.as_mut().unwrap().next = cur;
                    return dummy.unwrap().next;
                }
            }
            // let previous node of next group = rear's next node of this group
            // and use it as the previous node of this group
            let mut next_prev = rear.unwrap().next.take();
            // each turn let current node's next = it's prev node
            // and iterate current node
            while let Some(mut cur_inner) = cur.take() {
                cur = cur_inner.next.take();
                cur_inner.next = next_prev.take();
                next_prev = Some(cur_inner);
            }
            // connect the rear
            prev.as_mut().unwrap().next = next_prev;
            // iterate to the rear of this group
            // cannot do `prev = rear`, because there is another borrow of cur after the length detection
            // and rear (borrowed from cur) cannot live to the next iteration of loop
            for _ in 0..k {
                prev = prev.unwrap().next.as_mut();
            }
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::prelude::ListNode;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::reverse_k_group(linked_list!(1, 2, 3, 4, 5), 2),
            linked_list!(2, 1, 4, 3, 5)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::reverse_k_group(linked_list!(1, 2, 3, 4, 5), 3),
            linked_list!(3, 2, 1, 4, 5)
        );
    }
}
