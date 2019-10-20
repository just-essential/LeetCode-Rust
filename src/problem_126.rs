//! Word Ladder II [link](https://leetcode.com/problems/word-ladder-ii/)
//!
//! Given two words (*beginWord* and *endWord*), and a dictionary's word list, find all shortest transformation sequence(s) from *beginWord* to *endWord*, such that:
//! 1. Only one letter can be changed at a time
//! 2. Each transformed word must exist in the word list. Note that *beginWord* is *not* a transformed word.
//!
//! **Note:**
//! - Return an empty list if there is no such transformation sequence.
//! - All words have the same length.
//! - All words contain only lowercase alphabetic characters.
//! - You may assume no duplicates in the word list.
//! - You may assume *beginWord* and *endWord* are non-empty and are not the same.
//!
//! **Example 1:**
//! ```plain
//! Input:
//! beginWord = "hit",
//! endWord = "cog",
//! wordList = ["hot","dot","dog","lot","log","cog"]
//!
//! Output:
//! [
//!   ["hit","hot","dot","dog","cog"],
//!   ["hit","hot","lot","log","cog"]
//! ]
//! ```
//!
//! **Example 2:**
//! ```plain
//! Input:
//! beginWord = "hit"
//! endWord = "cog"
//! wordList = ["hot","dot","dog","lot","log"]
//!
//! Output: []
//!
//! Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
//! ```

use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut q = VecDeque::new();
        let mut words = HashSet::new();
        let mut visited = HashSet::new();
        for word in word_list {
            words.insert(word.into_bytes());
        }
        words.remove(begin_word.as_bytes());
        let mut level = 1;
        let mut min_level = usize::max_value();
        q.push_back(vec![begin_word]);
        while let Some(path) = q.front().cloned() {
            q.pop_front();
            if path.len() > level {
                level = path.len();
                if level > min_level {
                    break;
                }
                for word in &visited {
                    words.remove(word);
                }
                visited.clear();
            }
            if let Some(last) = path.last().cloned() {
                let mut last = last.into_bytes();
                for i in 0..last.len() {
                    let temp = last[i];
                    for c in b'a'..=b'z' {
                        last[i] = c;
                        if words.contains(&last) {
                            let mut new_path = path.clone();
                            visited.insert(last.clone());
                            let last = unsafe { String::from_utf8_unchecked(last.clone()) };
                            if last == end_word {
                                min_level = level;
                                new_path.push(last);
                                result.push(new_path);
                            } else {
                                new_path.push(last);
                                q.push_back(new_path);
                            }
                        }
                    }
                    last[i] = temp;
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
            Solution::find_ladders(
                "hit".to_owned(),
                "cog".to_owned(),
                string_vec!["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            vec![
                string_vec!["hit", "hot", "dot", "dog", "cog"],
                string_vec!["hit", "hot", "lot", "log", "cog"]
            ]
        )
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::find_ladders(
                "hit".to_owned(),
                "cog".to_owned(),
                string_vec!["hot", "dot", "dog", "lot", "log"]
            ),
            Vec::<Vec<String>>::new()
        )
    }
}
