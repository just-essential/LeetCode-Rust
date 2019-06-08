//! Median of Two Sorted Arrays [link](https://leetcode.com/problems/median-of-two-sorted-arrays/)
//!
//! There are two sorted arrays nums1 and nums2 of size m and n respectively.
//!
//! Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
//!
//! You may assume nums1 and nums2 cannot be both empty.
//!
//! **Example 1:**
//!
//! nums1 = [1, 3]
//! nums2 = [2]
//!
//! The median is 2.0
//!
//! **Example 2:**
//!
//! nums1 = [1, 2]
//! nums2 = [3, 4]
//!
//! The median is (2 + 3)/2 = 2.5

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() < nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };
        let (m, n) = (a.len(), b.len());
        let (mut i_min, mut i_max, half_len) = (0, m, (m + n + 1) / 2);
        while i_min <= i_max {
            let i = (i_min + i_max) / 2;
            let j = half_len - i;
            if i < m && a[i] < b[j - 1] {
                i_min = i + 1;
            } else if i > 0 && a[i - 1] > b[j] {
                i_max = i - 1;
            } else {
                let max_left = if i != 0 && (j == 0 || a[i - 1] > b[j - 1]) {
                    a[i - 1]
                } else {
                    b[j - 1]
                };
                if (m + n) % 2 == 1 {
                    return max_left as f64;
                }
                let min_right = if i != m && (j == n || a[i] < b[j]) {
                    a[i]
                } else {
                    b[j]
                };
                return (max_left + min_right) as f64 / 2.0;
            }
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
