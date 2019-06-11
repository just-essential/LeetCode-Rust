//! Trapping Rain Water [link](https://leetcode.com/problems/trapping-rain-water/)
//!
//! Given *n* non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it is able to trap after raining.
//!
//! **Example:**
//! ```plain
//! Input: [0,1,0,2,1,0,1,3,2,1,2,1]
//! Output: 6
//! ```

pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut result = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        while left < right {
            if height[left] < height[right] {
                if left_max < height[left] {
                    left_max = height[left];
                }
                result += left_max - height[left];
                left += 1;
            } else {
                if right_max < height[right] {
                    right_max = height[right];
                }
                result += right_max - height[right];
                right -= 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
