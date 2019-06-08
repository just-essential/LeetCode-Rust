/*
Given a string s1, we may represent it as a binary tree by partitioning it to two non-empty substrings recursively.

Below is one possible representation of s1 = "great":

    great
   /    \
  gr    eat
 / \    /  \
g   r  e   at
           / \
          a   t
To scramble the string, we may choose any non-leaf node and swap its two children.

For example, if we choose the node "gr" and swap its two children, it produces a scrambled string "rgeat".

    rgeat
   /    \
  rg    eat
 / \    /  \
r   g  e   at
           / \
          a   t
We say that "rgeat" is a scrambled string of "great".

Similarly, if we continue to swap the children of nodes "eat" and "at", it produces a scrambled string "rgtae".

    rgtae
   /    \
  rg    tae
 / \    /  \
r   g  ta  e
       / \
      t   a
We say that "rgtae" is a scrambled string of "great".

Given two strings s1 and s2 of the same length, determine if s2 is a scrambled string of s1.

Example 1:

Input: s1 = "great", s2 = "rgeat"
Output: true
Example 2:

Input: s1 = "abcde", s2 = "caebd"
Output: false
*/
struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        Solution::helper(s1.as_bytes(), s2.as_bytes())
    }
    fn helper(s1: &[u8], s2: &[u8]) -> bool {
        let len = s1.len();
        if s2.len() != len {
            return false;
        }
        let mut table = [0; 128];
        for i in 0..len {
            table[s1[i] as usize] += 1;
            table[s2[i] as usize] -= 1;
        }
        for i in 65..=122 {
            if table[i] != 0 {
                return false;
            }
        }
        if len <= 3 {
            return true;
        }
        for i in 1..len {
            if (Solution::helper(&s1[0..i], &s2[0..i])
                && Solution::helper(&s1[i..len], &s2[i..len]))
                || (Solution::helper(&s1[0..i], &s2[len - i..len])
                    && Solution::helper(&s1[i..len], &s2[0..len - i]))
            {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_scramble("great".to_owned(), "rgeat".to_owned()),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::is_scramble("abcde".to_owned(), "caebd".to_owned()),
            false
        );
    }
}
