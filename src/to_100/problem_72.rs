/*
Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.

You have the following 3 operations permitted on a word:

Insert a character
Delete a character
Replace a character
Example 1:

Input: word1 = "horse", word2 = "ros"
Output: 3
Explanation:
horse -> rorse (replace 'h' with 'r')
rorse -> rose (remove 'r')
rose -> ros (remove 'e')
Example 2:

Input: word1 = "intention", word2 = "execution"
Output: 5
Explanation:
intention -> inention (remove 't')
inention -> enention (replace 'i' with 'e')
enention -> exention (replace 'n' with 'x')
exention -> exection (replace 'n' with 'c')
exection -> execution (insert 'u')
*/
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2) = (word1.into_bytes(), word2.into_bytes());
        let (len1, len2) = (word1.len(), word2.len());
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
        for i in 0..=len2 {
            dp[0][i] = i;
        }
        for i in 1..=len1 {
            dp[i][0] = i;
            for j in 1..=len2 {
                dp[i][j] = if word1[i - 1] == word2[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    min(dp[i - 1][j - 1], min(dp[i - 1][j], dp[i][j - 1])) + 1
                }
            }
        }
        dp[len1][len2] as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_distance("horse".to_owned(), "ros".to_owned()),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_distance("intention".to_owned(), "execution".to_owned()),
            5
        );
    }
}
