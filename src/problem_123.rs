//! Best Time to Buy and Sell Stock III [link](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/)
//!
//! Say you have an array for which the *ith* element is the price of a given stock on day *i*.
//!
//! Design an algorithm to find the maximum profit. You may complete at most *two* transactions.
//!
//! **Note:** You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
//!
//! **Example 1:**
//! ```plain
//! Input: [3,3,5,0,0,3,1,4]
//! Output: 6
//! Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
//!              Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
//! ```
//!
//! **Example 2:**
//! ```plain
//! Input: [1,2,3,4,5]
//! Output: 4
//! Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
//!              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
//!              engaging multiple transactions at the same time. You must sell before buying again.
//! ```
//!
//! **Example 3:**
//! ```plain
//! Input: [7,6,4,3,1]
//! Output: 0
//! Explanation: In this case, no transaction is done, i.e. max profit = 0.
//! ```

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut one_buy = i32::min_value();
        let mut one_time_profit = 0;
        let mut two_buy = i32::min_value();
        let mut two_time_profit = 0;
        for price in prices {
            one_buy = one_buy.max(-price);
            one_time_profit = one_time_profit.max(price + one_buy);
            two_buy = two_buy.max(one_time_profit - price);
            two_time_profit = two_time_profit.max(two_buy + price);
        }
        two_time_profit
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
