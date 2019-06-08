use crate::prelude::ListNode;

struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut lists = lists;
        let mut step = 1;
        let len = lists.len();
        while step < len {
            for i in (0..len - step).step_by(step << 1) {
                lists[i] = Solution::merge_two_lists(lists[i].take(), lists[i + step].take());
            }
            step <<= 1;
        }
        lists[0].take()
    }
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut l1 = l1;
        let mut l2 = l2;
        let mut ref_head = &mut head;
        while let (Some(a), Some(b)) = (l1.as_ref(), l2.as_ref()) {
            if a.val < b.val {
                ref_head.next = l1;
                ref_head = ref_head.next.as_mut().unwrap();
                l1 = ref_head.next.take();
            } else {
                ref_head.next = l2;
                ref_head = ref_head.next.as_mut().unwrap();
                l2 = ref_head.next.take();
            }
        }
        ref_head.next = if l1.is_some() { l1 } else { l2 };
        head.next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::prelude::ListNode;

    #[test]
    fn example_1() {
        let lists = vec![linked_list![1, 2, 3], linked_list![2, 3]];
        let result = linked_list!(1, 2, 2, 3, 3);
        assert_eq!(Solution::merge_k_lists(lists), result);
    }
}