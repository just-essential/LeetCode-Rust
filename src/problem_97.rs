/*
Given s1, s2, s3, find whether s3 is formed by the interleaving of s1 and s2.

Example 1:

Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
Output: true
Example 2:

Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
Output: false
*/
struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let l1 = s1.len();
        let l2 = s2.len();
        let l3 = s3.len();
        if l1 + l2 != l3 {
            return false;
        }
        let mut dp = vec![vec![true; l2 + 1]; l1 + 1];
        for i in 1..=l2 {
            dp[0][i] = dp[0][i - 1] && s2[i - 1] == s3[i - 1];
        }
        for i in 1..=l1 {
            dp[i][0] = dp[i - 1][0] && s1[i - 1] == s3[i - 1];
            for j in 1..=l2 {
                dp[i][j] = (dp[i - 1][j] && s1[i - 1] == s3[i + j - 1])
                    || (dp[i][j - 1] && s2[j - 1] == s3[i + j - 1]);
            }
        }
        dp[l1][l2]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbcbcac".to_owned()
            ),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbbaccc".to_owned()
            ),
            false
        );
    }
}
