//! First Missing Positive [link](https://leetcode.com/problems/first-missing-positive/)
//!
//! Given an unsorted integer array, find the smallest missing positive integer.
//!
//! **Example 1:**
//! ```plain
//! Input: [1,2,0]
//! Output: 3
//! ```
//!
//! **Example 2:**
//! ```plain
//! Input: [3,4,-1,1]
//! Output: 2
//! ```
//!
//! **Example 3:**
//! ```plain
//! Input: [7,8,9,11,12]
//! Output: 1
//! ```
//!
//! **Note:**
//!
//! Your algorithm should run in O(n) time and uses constant extra space.

pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        for i in 0..len {
            while nums[i] > 0 && nums[i] <= len as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let temp = nums[i];
                nums[i] = nums[temp as usize - 1];
                nums[temp as usize - 1] = temp;
            }
        }
        for (index, &value) in nums.iter().enumerate() {
            if value != index as i32 + 1 {
                return index as i32 + 1;
            }
        }
        len as i32 + 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
