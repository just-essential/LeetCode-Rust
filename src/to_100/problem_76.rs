/*
Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).

Example:

Input: S = "ADOBECODEBANC", T = "ABC"
Output: "BANC"
Note:

If there is no such window in S that covers all characters in T, return the empty string "".
If there is such window, you are guaranteed that there will always be only one unique minimum window in S.
*/
struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (vec_s, vec_t) = (s.as_bytes(), t.as_bytes());
        let mut table = [0; 128];
        // count
        for &ch in vec_t {
            table[ch as usize] += 1;
        }
        // init window
        let mut mismatched = vec_t.len();
        let (mut left, mut right) = (0, 0);
        for &ch in vec_s {
            if mismatched == 0 {
                break;
            }
            if table[ch as usize] > 0 {
                mismatched -= 1;
            }
            table[ch as usize] -= 1;
            right += 1;
        }
        if mismatched != 0 {
            return String::new();
        }
        while table[vec_s[left] as usize] < 0 {
            table[vec_s[left] as usize] += 1;
            left += 1;
        }
        let (mut min_begin, mut min_len) = (left, right - left);
        // move window
        let len = vec_s.len();
        while right < len {
            // extend
            while right < len && vec_s[right] != vec_s[left] {
                table[vec_s[right] as usize] -= 1;
                right += 1;
            }
            if right == len {
                break;
            }
            table[vec_s[right] as usize] -= 1;
            right += 1;
            // narrow
            while left < len && table[vec_s[left] as usize] < 0 {
                table[vec_s[left] as usize] += 1;
                left += 1;
            }
            if right - left < min_len {
                min_begin = left;
                min_len = right - left;
            }
        }
        s[min_begin..min_len + min_begin].to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned()),
            "BANC".to_owned()
        );
    }
}
