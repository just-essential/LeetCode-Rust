//! Maximal Rectangle [link](https://leetcode.com/problems/maximal-rectangle/)
//!
//! Given a 2D binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
//!
//! **Example:**
//! ```plain
//! Input:
//! [
//!   ["1","0","1","0","0"],
//!   ["1","0","1","1","1"],
//!   ["1","1","1","1","1"],
//!   ["1","0","0","1","0"]
//! ]
//! Output: 6
//! ```

pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.first().map_or(true, |line| line.get(0).is_none()) {
            return 0;
        }
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut lefts = vec![0; col];
        let mut rights = vec![col; col];
        let mut heights = vec![0; col];
        let mut result = 0;
        let mut left;
        let mut right;
        for i in 0..row {
            left = 0;
            right = col;
            for j in 0..col {
                if matrix[i][j] == '1' {
                    heights[j] += 1;
                    lefts[j] = lefts[j].max(left);
                } else {
                    heights[j] = 0;
                    left = j + 1;
                    lefts[j] = 0;
                }
            }
            for j in (0..col).rev() {
                if matrix[i][j] == '1' {
                    rights[j] = rights[j].min(right);
                } else {
                    right = j;
                    rights[j] = col;
                }
            }
            for j in 0..col {
                result = result.max((rights[j] - lefts[j]) * heights[j]);
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }
}
