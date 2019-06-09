//! Sudoku Solver [link](https://leetcode.com/problems/sudoku-solver/)
//!
//! Write a program to solve a Sudoku puzzle by filling the empty cells.
//!
//! A sudoku solution must satisfy **all of the following rules**:
//!
//! 1. Each of the digits `1-9` must occur exactly once in each row.
//! 2. Each of the digits `1-9` must occur exactly once in each column.
//! 3. Each of the the digits `1-9` must occur exactly once in each of the 9 `3x3` sub-boxes of the grid.
//!
//! Empty cells are indicated by the character `'.'`.
//!
//! **Example:**
//! ```
//! input = vec![
//!     vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
//!     vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
//!     vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
//!     vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
//!     vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
//!     vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
//!     vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
//!     vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
//!     vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
//! ];
//! output = vec![
//!     vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
//!     vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
//!     vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
//!     vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
//!     vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
//!     vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
//!     vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
//!     vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
//!     vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
//! ];
//! ```
//!
//! **Note:**
//!
//! - The given board contain only digits `1-9` and the character `'.'`.
//! - You may assume that the given Sudoku puzzle will have a single unique solution.
//! - The given board size is always `9x9`.

pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        struct Env {
            rows: [usize; 9],
            columns: [usize; 9],
            sub_boxes: [usize; 9],
            num_board: [usize; 81],
        }
        let mut env = Env {
            rows: [0; 9],
            columns: [0; 9],
            sub_boxes: [0; 9],
            num_board: [0; 81],
        };
        // init environments
        for row in 0..9 {
            for column in 0..9 {
                if board[row][column] != '.' {
                    let num = board[row][column] as usize - '0' as usize;
                    env.rows[row] |= 1 << num;
                    env.columns[column] |= 1 << num;
                    env.sub_boxes[row / 3 * 3 + column / 3] |= 1 << num;
                    env.num_board[row * 9 + column] = num;
                }
            }
        }
        fn set_cell(mut pos: usize, env: &mut Env) -> bool {
            while pos < 81 && env.num_board[pos] != 0 {
                pos += 1;
            }
            if pos == 81 {
                return true;
            }
            let (row, column) = (pos / 9, pos % 9);
            let n = (row / 3) * 3 + (column / 3);
            let bits = env.rows[row] | env.columns[column] | env.sub_boxes[n];
            for num in 1..=9 {
                if 1 << num & bits == 0 {
                    env.rows[row] |= 1 << num;
                    env.columns[column] |= 1 << num;
                    env.sub_boxes[n] |= 1 << num;
                    env.num_board[pos] = num;
                    if set_cell(pos, env) {
                        return true;
                    }
                    env.rows[row] -= 1 << num;
                    env.columns[column] -= 1 << num;
                    env.sub_boxes[n] -= 1 << num;
                    env.num_board[pos] = 0;
                }
            }
            false
        };
        // set each cell from the first cell
        set_cell(0, &mut env);
        // update board by num_board
        for row in 0..9 {
            for column in 0..9 {
                board[row][column] = (env.num_board[row * 9 + column] + '0' as usize) as u8 as char;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let output = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, output);
    }
}
