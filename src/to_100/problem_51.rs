/*
The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.

Given an integer n, return all distinct solutions to the n-queens puzzle.

Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space respectively.

Example:

Input: 4
Output: [
 [".Q..",  // Solution 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // Solution 2
  "Q...",
  "...Q",
  ".Q.."]
]
Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.
*/
struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        struct Env {
            n: usize,
            board: Vec<usize>,
            result: Vec<Vec<String>>,
        }
        let mut env = Env {
            n: n as usize,
            board: vec![0; n as usize],
            result: vec![],
        };
        fn set_queen(row: usize, ref_env: &mut Env) {
            if row == ref_env.n {
                let mut board = vec![vec!['.'; ref_env.n]; ref_env.n];
                for i in 0..ref_env.n {
                    board[i][ref_env.board[i]] = 'Q';
                }
                ref_env.result.push(
                    board
                        .into_iter()
                        .map(|line| line.into_iter().collect())
                        .collect(),
                );
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
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![
                    ".Q..".to_string(),
                    "...Q".to_string(),
                    "Q...".to_string(),
                    "..Q.".to_string()
                ],
                vec![
                    "..Q.".to_string(),
                    "Q...".to_string(),
                    "...Q".to_string(),
                    ".Q..".to_string()
                ]
            ]
        )
    }
}
