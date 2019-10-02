//! Distinct Subsequences [link](https://leetcode.com/problems/distinct-subsequences/)
//!
//! Given a string **S** and a string **T**, count the number of distinct subsequences of **S** which equals **T**.
//!
//! A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, `"ACE"` is a subsequence of `"ABCDE"` while `"AEC"` is not).
//!
//! **Example 1:**
//! ```plain
//! Input: S = "rabbbit", T = "rabbit"
//! Output: 3
//! Explanation:
//!
//! As shown below, there are 3 ways you can generate "rabbit" from S.
//! (The caret symbol ^ means the chosen letters)
//!
//! rabbbit
//! ^^^^ ^^
//! rabbbit
//! ^^ ^^^^
//! rabbbit
//! ^^^ ^^^
//! ```
//!
//! **Example 2:**
//! ```pain
//! Input: S = "babgbag", T = "bag"
//! Output: 5
//! Explanation:
//!
//! As shown below, there are 5 ways you can generate "bag" from S.
//! (The caret symbol ^ means the chosen letters)
//!
//! babgbag
//! ^^ ^
//! babgbag
//! ^^    ^
//! babgbag
//! ^    ^^
//! babgbag
//!   ^  ^^
//! babgbag
//!     ^^^
//! ```

pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (s_len, t_len) = (s.len(), t.len());
        if s_len < t_len {
            return 0;
        }
        let mut dp = vec![0; t_len];
        for i in 0..s_len {
            for j in (0..t_len).rev() {
                if s[i] == t[j] {
                    if j == 0 {
                        dp[j] += 1;
                    } else {
                        dp[j] += dp[j - 1]
                    }
                }
            }
        }
        dp[t_len - 1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned()),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::num_distinct("babgbag".to_owned(), "bag".to_owned()),
            5
        );
    }
}
