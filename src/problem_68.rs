//! Text Justification [link](https://leetcode.com/problems/text-justification/)
//!
//! Given an array of words and a width *maxWidth*, format the text such that each line has exactly *maxWidth* characters and is fully (left and right) justified.
//!
//! You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces `' '` when necessary so that each line has exactly *maxWidth* characters.
//!
//! Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line do not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
//!
//! For the last line of text, it should be left justified and no **extra** space is inserted between words.
//!
//! **Note:**
//!
//! - A word is defined as a character sequence consisting of non-space characters only.
//! - Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
//! - The input array words contains at least one word.
//!
//! **Example 1:**
//! ```plain
//! Input:
//! words = ["This", "is", "an", "example", "of", "text", "justification."]
//! maxWidth = 16
//! Output:
//! [
//!    "This    is    an",
//!    "example  of text",
//!    "justification.  "
//! ]
//! ```
//!
//! **Example 2:**
//! ```plain
//! Input:
//! words = ["What","must","be","acknowledgment","shall","be"]
//! maxWidth = 16
//! Output:
//! [
//!   "What   must   be",
//!   "acknowledgment  ",
//!   "shall be        "
//! ]
//! Explanation: Note that the last line is "shall be    " instead of "shall     be",
//!              because the last line must be left-justified instead of fully-justified.
//!              Note that the second line is also left-justified becase it contains only one word.
//! ```
//!
//! **Example 3:**
//! ```plain
//! Input:
//! words = ["Science","is","what","we","understand","well","enough","to","explain",
//!          "to","a","computer.","Art","is","everything","else","we","do"]
//! maxWidth = 20
//! Output:
//! [
//!   "Science  is  what we",
//!   "understand      well",
//!   "enough to explain to",
//!   "a  computer.  Art is",
//!   "everything  else  we",
//!   "do                  "
//! ]
//! ```

pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut width = 0;
        let max_width = max_width as usize;
        let mut flag = 0;
        let mut last = 0;
        for i in 0..words.len() {
            if words[i].len() + width > max_width {
                width -= i - flag;
                result.push(words[flag].clone());
                flag += 1;
                if i == flag {
                    result[last] += &" ".repeat(max_width - width);
                }
                while flag < i {
                    let spaces = (max_width - width + i - flag - 1) / (i - flag);
                    result[last] += &" ".repeat(spaces);
                    result[last] += &words[flag];
                    flag += 1;
                    width += spaces;
                }
                width = 0;
                flag = i;
                last += 1;
            }
            width += words[i].len() + 1;
        }
        result.push(words[flag].clone());
        flag += 1;
        for i in flag..words.len() {
            result[last] += " ";
            result[last] += &words[i];
        }
        width = result[last].len();
        result[last] += &" ".repeat(max_width - width);
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::full_justify(
                string_vec!(
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ),
                16,
            ),
            string_vec!("This    is    an", "example  of text", "justification.  ")
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::full_justify(
                string_vec!("What", "must", "be", "acknowledgment", "shall", "be"),
                16,
            ),
            string_vec!("What   must   be", "acknowledgment  ", "shall be        ")
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::full_justify(
                string_vec!(
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ),
                20,
            ),
            string_vec!(
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            )
        )
    }
}
