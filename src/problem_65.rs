//! Valid Number [link](https://leetcode.com/problems/valid-number/)
//!
//! Validate if a given string can be interpreted as a decimal number.
//!
//! Some examples:
//!
//! `"0"` => `true`
//!
//! `" 0.1 "` => `true`
//!
//! `"abc"` => `false`
//!
//! `"1 a"` => `false`
//!
//! `"2e10"` => `true`
//!
//! `" -90e3   "` => `true`
//!
//! `" 1e"` => `false`
//!
//! `"e3"` => `false`
//!
//! `" 6e-1"` => `true`
//!
//! `" 99e2.5 "` => `false`
//!
//! `"53.5e93"` => `true`
//!
//! `" --6 "` => `false`
//!
//! `"-+3"` => `false`
//!
//! `"95a54e53"` => `false`
//!
//! **Note:** It is intended for the problem statement to be ambiguous. You should gather all requirements up front before implementing one. However, here is a list of characters that can be in a valid decimal number:
//!
//! - Numbers 0-9
//! - Exponent - "e"
//! - Positive/negative sign - "+"/"-"
//! - Decimal point - "."
//!
//! Of course, the context of these characters also matters in the input.
//!
//! **Update (2015-02-10):**
//! The signature of the `C++` function had been updated. If you still see your function signature accepts a `const char *` argument, please click the reload button to reset your code definition.

pub enum State {
    Nil,
    Sign,
    Integer,
    Dot,
    JustDot,
    Decimal,
    Euler,
    ExpSign,
    Exponent,
}

pub struct Solution;
impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.trim();
        let mut state = State::Nil;
        for ch in s.chars() {
            match ch {
                '0'...'9' => match state {
                    State::Nil | State::Sign | State::Integer => state = State::Integer,
                    State::Dot | State::JustDot | State::Decimal => state = State::Decimal,
                    _ => state = State::Exponent,
                },
                '+' | '-' => match state {
                    State::Nil => state = State::Sign,
                    State::Euler => state = State::ExpSign,
                    _ => return false,
                },
                '.' => match state {
                    State::Nil | State::Sign => state = State::JustDot,
                    State::Integer => state = State::Dot,
                    _ => return false,
                },
                'e' => match state {
                    State::Integer | State::Dot | State::Decimal => state = State::Euler,
                    _ => return false,
                },
                _ => return false,
            }
        }
        match state {
            State::Integer | State::Dot | State::Decimal | State::Exponent => true,
            _ => false,
        }
    }
}
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn example_1() {
        assert_eq!(Solution::is_number("0".to_string()), true);
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::is_number(" 0.1 ".to_string()), true);
    }
    #[test]
    fn example_3() {
        assert_eq!(Solution::is_number("abc".to_string()), false);
    }
    #[test]
    fn example_4() {
        assert_eq!(Solution::is_number("1 a".to_string()), false);
    }
    #[test]
    fn example_5() {
        assert_eq!(Solution::is_number("2e10".to_string()), true);
    }
    #[test]
    fn example_6() {
        assert_eq!(Solution::is_number(" -90e3   ".to_string()), true);
    }
    #[test]
    fn example_7() {
        assert_eq!(Solution::is_number(" 1e".to_string()), false);
    }
    #[test]
    fn example_8() {
        assert_eq!(Solution::is_number("e3".to_string()), false);
    }
    #[test]
    fn example_9() {
        assert_eq!(Solution::is_number(" 6e-1".to_string()), true);
    }
    #[test]
    fn example_10() {
        assert_eq!(Solution::is_number(" 99e2.5 ".to_string()), false);
    }
    #[test]
    fn example_11() {
        assert_eq!(Solution::is_number("53.5e93".to_string()), true);
    }
    #[test]
    fn example_12() {
        assert_eq!(Solution::is_number(" --6 ".to_string()), false);
    }
    #[test]
    fn example_13() {
        assert_eq!(Solution::is_number("-+3".to_string()), false);
    }
    #[test]
    fn example_14() {
        assert_eq!(Solution::is_number("95a54e53".to_string()), false);
    }
}
