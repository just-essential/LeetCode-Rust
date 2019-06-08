/*
Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*'.

'?' Matches any single character.
'*' Matches any sequence of characters (including the empty sequence).
The matching should cover the entire input string (not partial).

Note:

s could be empty and contains only lowercase letters a-z.
p could be empty and contains only lowercase letters a-z, and characters like ? or *.
Example 1:

Input:
s = "aa"
p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".
Example 2:

Input:
s = "aa"
p = "*"
Output: true
Explanation: '*' matches any sequence.
Example 3:

Input:
s = "cb"
p = "?a"
Output: false
Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
Example 4:

Input:
s = "adceb"
p = "*a*b"
Output: true
Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".
Example 5:

Input:
s = "acdcb"
p = "a*c?b"
Output: false
*/

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.into_bytes(), p.into_bytes());
        let (mut i, mut j) = (0, 0);
        let (mut i_point, mut j_point) = (None, None);
        while let Some(&s_char) = s.get(i) {
            if let Some(&p_char) = p.get(j) {
                if s_char == p_char || p_char == '?' as u8 {
                    i += 1;
                    j += 1;
                    continue;
                } else if p_char == '*' as u8 {
                    i_point = Some(i);
                    j_point = Some(j);
                    j += 1;
                    continue;
                }
            }
            if let (Some(i_point_value), Some(j_point_value)) = (i_point, j_point) {
                i_point = Some(i_point_value + 1);
                i = i_point_value + 1;
                j = j_point_value + 1;
            } else {
                return false;
            }
        }
        while Some(&('*' as u8)) == p.get(j) {
            j += 1;
        }
        j == p.len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::is_match("cb".to_string(), "?a".to_string()),
            false
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::is_match("adceb".to_string(), "*a*b".to_string()),
            true
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            Solution::is_match("acdcb".to_string(), "a*c?b".to_string()),
            false
        );
    }
}