//! Binary Tree Maximum Path Sum [link](https://leetcode.com/problems/binary-tree-maximum-path-sum/)
//!
//! Given a **non-empty** binary tree, find the maximum path sum.
//!
//! For this problem, a path is defined as any sequence of nodes from some starting node to any node in the tree along the parent-child connections. The path must contain **at least one** node and does not need to go through the root.
//!
//! **Example 1:**
//! ```plain
//! Input: [1,2,3]
//!
//!        1
//!       / \
//!      2   3
//!
//! Output: 6
//! ```
//!
//! **Example 2:**
//! ```plain
//! Input: [-10,9,20,null,null,15,7]
//!
//!    -10
//!    / \
//!   9  20
//!     /  \
//!    15   7
//!
//! Output: 42
//! ```

use crate::prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::min_value();
        Self::max_sum(&root, &mut result);
        result
    }
    fn max_sum(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        match root {
            Some(r) => {
                let left = 0.max(Self::max_sum(&r.borrow().left, result));
                let right = 0.max(Self::max_sum(&r.borrow().right, result));
                *result = (*result).max(r.borrow().val + left + right);
                r.borrow().val + left.max(right)
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::prelude::TreeNode;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_path_sum(btree![1, 2, 3]), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_path_sum(btree![-10, 9, 20, null, null, 15, 7]),
            42
        );
    }
}
