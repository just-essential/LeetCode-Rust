/*
Given an array of non-negative integers, you are initially positioned at the first index of the array.

Each element in the array represents your maximum jump length at that position.

Your goal is to reach the last index in the minimum number of jumps.

Example:

Input: [2,3,1,1,4]
Output: 2
Explanation: The minimum number of jumps to reach the last index is 2.
    Jump 1 step from index 0 to 1, then 3 steps to the last index.
Note:

You can assume that you can always reach the last index.
*/
struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let rear = nums.len() - 1;
        let mut max_step;
        let mut step;
        let mut result = 0;
        let mut i = 0;
        while i < rear {
            result += 1;
            if i + nums[i] as usize >= rear {
                break;
            }
            max_step = nums[i + 1] as usize + i;
            step = 1;
            for j in 2..=nums[i] as usize {
                if nums[i + j] as usize + j > max_step {
                    max_step = nums[i + j] as usize + j;
                    step = j;
                }
            }
            i += step;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}