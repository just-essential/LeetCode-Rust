//! Substring with Concatenation of All Words [link](https://leetcode.com/problems/substring-with-concatenation-of-all-words/)
//!
//! You are given a string, **s**, and a list of words, **words**, that are all of the same length. Find all starting indices of substring(s) in **s** that is a concatenation of each word in **words** exactly once and without any intervening characters.
//!
//! **Example 1:**
//!
//! ```plain
//! Input:
//!   s = "barfoothefoobarman",
//!   words = ["foo","bar"]
//! Output: [0,9]
//! Explanation: Substrings starting at index 0 and 9 are "barfoor" and "foobar" respectively.
//! The output order does not matter, returning [9,0] is fine too.
//! ```
//!
//! **Example 2:**
//!
//! ```plain
//! Input:
//!   s = "wordgoodgoodgoodbestword",
//!   words = ["word","good","best","word"]
//! Output: []
//! ```
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return vec![];
        }
        let (s_len, w_size, w_len) = (s.len(), words.len(), words[0].len());
        if s_len < w_len {
            return vec![];
        }
        let limit = s_len - w_len;
        let mut count_words = HashMap::new();
        let mut result = Vec::new();
        for i in 0..w_size {
            let count = count_words.entry(&words[i][..]).or_insert(1);
            *count += 1;
        }
        for i in 0..w_len {
            let mut left = i;
            let mut count = 0;
            let mut flags = HashMap::new();
            for j in (i..=limit).step_by(w_len) {
                let string = &s[j..j + w_len];
                if count_words.contains_key(string) {
                    let cnt = flags.entry(string).or_insert(1);
                    *cnt += 1;
                    count += 1;
                    if flags.get(string).unwrap() > count_words.get(string).unwrap() {
                        loop {
                            let temp = &s[left..left + w_len];
                            let cnt = flags.entry(temp).or_insert(1);
                            *cnt -= 1;
                            count -= 1;
                            left += w_len;
                            if string == temp {
                                break;
                            }
                        }
                    }
                    if count == w_size {
                        result.push(left as i32);
                        let cnt = flags.entry(&s[left..left + w_len]).or_insert(1);
                        *cnt -= 1;
                        count -= 1;
                        left += w_len;
                    }
                } else {
                    flags.clear();
                    count = 0;
                    left = j + w_len;
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
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()],
            ),
            vec![0, 9]
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ],
            ),
            vec![]
        )
    }
}
