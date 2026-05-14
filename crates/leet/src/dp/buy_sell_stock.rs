use std::cmp::{max, min};

/// leet 121

impl Solution {
    /// One pass tracking min price. O(n) time, O(1) space.
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut min_price, mut profit) = (i32::MAX, 0);
        for p in prices {
            min_price = min(min_price, p);
            profit = max(profit, p - min_price);
        }
        profit
    }

    /// Kadane's algorithm on daily price changes. O(n) time, O(1) space.
    pub fn max_profit_kadane(prices: Vec<i32>) -> i32 {
        let (mut max_here, mut res) = (0, 0);
        for i in 1..prices.len() {
            max_here = max(0, max_here + prices[i] - prices[i - 1]);
            res = max(res, max_here);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![5]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(vec![2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![3, 3, 3, 3]), 0);
        assert_eq!(Solution::max_profit(vec![0, 10000]), 10000);
        assert_eq!(Solution::max_profit(vec![3, 8, 1, 9, 2]), 8);
        assert_eq!(Solution::max_profit(vec![1, 2, 1, 2, 1, 2]), 1);
    }

    #[test]
    fn test_max_profit_kadane() {
        assert_eq!(Solution::max_profit_kadane(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit_kadane(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit_kadane(vec![5]), 0);
        assert_eq!(Solution::max_profit_kadane(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit_kadane(vec![2, 1]), 0);
        assert_eq!(Solution::max_profit_kadane(vec![3, 3, 3, 3]), 0);
        assert_eq!(Solution::max_profit_kadane(vec![0, 10000]), 10000);
        assert_eq!(Solution::max_profit_kadane(vec![3, 8, 1, 9, 2]), 8);
        assert_eq!(Solution::max_profit_kadane(vec![1, 2, 1, 2, 1, 2]), 1);
    }
}
