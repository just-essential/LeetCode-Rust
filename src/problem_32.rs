/*
Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.

Example 1:

Input: "(()"
Output: 2
Explanation: The longest valid parentheses substring is "()"
Example 2:

Input: ")()())"
Output: 4
Explanation: The longest valid parentheses substring is "()()"
*/

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.into_bytes();
        let mut stack = vec![-1];
        let mut result = 0;
        for i in 0..s.len() as i32 {
            if s[i as usize] == '(' as u8 {
                stack.push(i);
            } else {
                stack.pop();
                if let Some(&last) = stack.last() {
                    if result < i - last {
                        result = i - last;
                    }
                } else {
                    stack.push(i);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    }
}
