use std::cmp::max;

/// leet 309

pub struct Solution;

impl Solution {
    /// State machine DP. O(n) time, O(1) space.
    /// States: hold (have stock), sold (just sold), rest (cooldown/idle).
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut hold, mut sold, mut rest) = (i32::MIN, 0, 0);
        for p in prices {
            let prev_hold = hold;
            hold = max(hold, rest - p);
            rest = max(rest, sold);
            sold = prev_hold + p;
        }
        max(sold, rest)
    }

    /// DP arrays. O(n) time, O(n) space.
    /// buy[i] = max(buy[i-1], sell[i-2] - prices[i])
    /// sell[i] = max(sell[i-1], buy[i-1] + prices[i])
    pub fn max_profit_dp(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        let mut buy = vec![0; n];
        let mut sell = vec![0; n];
        buy[0] = -prices[0];
        buy[1] = max(-prices[0], -prices[1]);
        sell[1] = max(0, prices[1] - prices[0]);
        for i in 2..n {
            buy[i] = max(buy[i - 1], sell[i - 2] - prices[i]);
            sell[i] = max(sell[i - 1], buy[i - 1] + prices[i]);
        }
        sell[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(vec![2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![5, 4, 3, 2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![3, 3, 3, 3]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2, 4, 0, 3]), 6);
        assert_eq!(Solution::max_profit(vec![1, 4, 2, 7]), 6);
        assert_eq!(Solution::max_profit(vec![0, 1000]), 1000);
    }

    #[test]
    fn test_max_profit_dp() {
        assert_eq!(Solution::max_profit_dp(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit_dp(vec![1]), 0);
        assert_eq!(Solution::max_profit_dp(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit_dp(vec![2, 1]), 0);
        assert_eq!(Solution::max_profit_dp(vec![5, 4, 3, 2, 1]), 0);
        assert_eq!(Solution::max_profit_dp(vec![3, 3, 3, 3]), 0);
        assert_eq!(Solution::max_profit_dp(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::max_profit_dp(vec![1, 2, 3, 0, 2, 4, 0, 3]), 6);
        assert_eq!(Solution::max_profit_dp(vec![1, 4, 2, 7]), 6);
        assert_eq!(Solution::max_profit_dp(vec![0, 1000]), 1000);
    }
}
