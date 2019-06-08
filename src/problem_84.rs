/*
Given n non-negative integers representing the histogram's bar height where the width of each bar is 1, find the area of largest rectangle in the histogram.

Above is a histogram where width of each bar is 1, given height = [2,1,5,6,2,3].

The largest rectangle is shown in the shaded area, which has area = 10 unit.

Example:

Input: [2,1,5,6,2,3]
Output: 10
*/
struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        if len == 0 {
            return 0;
        }
        let mut less_from_left = vec![-1; len];
        let mut less_from_right = vec![len as i32; len];
        let mut less;
        for i in 1..len {
            less = i as i32 - 1;
            while less >= 0 && heights[less as usize] >= heights[i] {
                less = less_from_left[less as usize];
            }
            less_from_left[i] = less;
        }
        for i in (0..len - 1).rev() {
            less = i as i32 + 1;
            while less < len as i32 && heights[less as usize] >= heights[i] {
                less = less_from_right[less as usize];
            }
            less_from_right[i] = less;
        }
        let mut result = 0;
        for i in 0..len {
            result = result.max((less_from_right[i] - less_from_left[i] - 1) * heights[i]);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}
