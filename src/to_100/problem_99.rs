/*
Two elements of a binary search tree (BST) are swapped by mistake.

Recover the tree without changing its structure.

Example 1:

Input: [1,3,null,null,2]

   1
  /
 3
  \
   2

Output: [3,1,null,null,2]

   3
  /
 1
  \
   2
Example 2:

Input: [3,1,4,null,null,2]

  3
 / \
1   4
   /
  2

Output: [2,1,4,null,null,3]

  2
 / \
1   4
   /
  3
Follow up:

A solution using O(n) space is pretty straight forward.
Could you devise a constant space solution?
*/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::prelude::TreeNode;
use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut s = Vec::new();
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;
        let mut cur = root.clone();
        let mut first = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut second = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut found_first = false;
        while !s.is_empty() || cur.is_some() {
            if let Some(cur_inner) = cur {
                s.push(cur_inner.clone());
                cur = cur_inner.borrow().left.clone();
            } else {
                cur = s.pop();
                if let (Some(pre_inner), Some(cur_inner)) = (pre, cur.as_ref()) {
                    if pre_inner.borrow().val > cur_inner.borrow().val {
                        if !found_first {
                            first = pre_inner.clone();
                            found_first = true;
                        }
                        second = cur_inner.clone();
                    }
                }
                pre = cur;
                cur = pre.as_ref().unwrap().borrow().right.clone();
            }
        }
        swap(
            &mut first.as_ref().borrow_mut().val,
            &mut second.as_ref().borrow_mut().val,
        );
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::prelude::TreeNode;

    #[test]
    fn example_1() {
        let mut root = btree![1, 3, null, null, 2];
        Solution::recover_tree(&mut root);
        assert_eq!(root, btree![3, 1, null, null, 2]);
    }

    #[test]
    fn example_2() {
        let mut root = btree![3, 1, 4, null, null, 2];
        Solution::recover_tree(&mut root);
        assert_eq!(root, btree![2, 1, 4, null, null, 3]);
    }
}
