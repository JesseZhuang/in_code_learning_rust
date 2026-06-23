/// leet 518

pub struct Solution;

impl Solution {
    /// 1D DP. O(N*M) time, O(M) space. N: coins.len(), M: amount.
    /// Outer loop on coins ensures each combination is counted once (not permutations).
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let m = amount as usize;
        let mut dp = vec![0; m + 1]; // dp[i]: number of combinations for amount i
        dp[0] = 1;
        for &coin in &coins { // O(N)
            let c = coin as usize;
            for i in c..=m { // O(M)
                dp[i] += dp[i - c];
            }
        }
        dp[m]
    }

    /// 2D DP. O(N*M) time, O(N*M) space.
    /// dp[i][j] = number of combinations using first i coins for amount j.
    pub fn change_2d(amount: i32, coins: Vec<i32>) -> i32 {
        let n = coins.len();
        let m = amount as usize;
        // dp[i][j]: combinations using coins[0..i] to make amount j
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = 1; // one way to make amount 0: use no coins
        }
        for i in 1..=n { // O(N)
            let c = coins[i - 1] as usize;
            for j in 1..=m { // O(M)
                dp[i][j] = dp[i - 1][j]; // don't use coin i
                if j >= c {
                    dp[i][j] += dp[i][j - c]; // use coin i (unlimited)
                }
            }
        }
        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1d() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
        assert_eq!(Solution::change(3, vec![2]), 0);
        assert_eq!(Solution::change(10, vec![10]), 1);
        assert_eq!(Solution::change(0, vec![1, 2, 5]), 1);
        assert_eq!(Solution::change(7, vec![7]), 1);
        assert_eq!(Solution::change(7, vec![3]), 0);
        assert_eq!(Solution::change(100, vec![1, 5, 10, 25]), 242);
    }

    #[test]
    fn test_2d() {
        assert_eq!(Solution::change_2d(5, vec![1, 2, 5]), 4);
        assert_eq!(Solution::change_2d(3, vec![2]), 0);
        assert_eq!(Solution::change_2d(10, vec![10]), 1);
        assert_eq!(Solution::change_2d(0, vec![1, 2, 5]), 1);
        assert_eq!(Solution::change_2d(7, vec![7]), 1);
        assert_eq!(Solution::change_2d(7, vec![3]), 0);
        assert_eq!(Solution::change_2d(100, vec![1, 5, 10, 25]), 242);
    }
}
