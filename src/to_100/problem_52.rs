/*
The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.

Given an integer n, return the number of distinct solutions to the n-queens puzzle.

Example:

Input: 4
Output: 2
Explanation: There are two distinct solutions to the 4-queens puzzle as shown below.
[
 [".Q..",  // Solution 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // Solution 2
  "Q...",
  "...Q",
  ".Q.."]
]
*/
struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        struct Env {
            n: usize,
            board: Vec<usize>,
            result: i32,
        }
        let mut env = Env {
            n: n as usize,
            board: vec![0; n as usize],
            result: 0,
        };
        fn set_queen(row: usize, ref_env: &mut Env) {
            if row == ref_env.n {
                ref_env.result += 1;
            } else {
                let mut flags = vec![true; ref_env.n];
                for i in 0..row {
                    let j = ref_env.board[i];
                    flags[j] = false;
                    if j >= row - i {
                        flags[j - (row - i)] = false;
                    }
                    if j + row - i < ref_env.n {
                        flags[j + row - i] = false;
                    }
                }
                for i in 0..ref_env.n {
                    if flags[i] {
                        ref_env.board[row] = i;
                        set_queen(row + 1, ref_env);
                    }
                }
            }
        }
        set_queen(0, &mut env);
        env.result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn example_1() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }
}
