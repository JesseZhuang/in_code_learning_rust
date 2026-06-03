/// leet 312

impl Solution {
    /// Bottom-up interval DP. O(n^3) time, O(n^2) space.
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // Pad with 1s on both sides
        let mut val = vec![1];
        val.extend(&nums);
        val.push(1);

        // dp[i][j] = max coins from bursting all balloons between i and j (exclusive)
        let mut dp = vec![vec![0; n + 2]; n + 2];

        // length: number of balloons between i and j (exclusive)
        for length in 2..=n + 1 {
            for i in 0..=n + 1 - length {
                let j = i + length;
                // k is the last balloon to burst in range (i, j)
                for k in i + 1..j {
                    let coins = val[i] * val[k] * val[j] + dp[i][k] + dp[k][j];
                    dp[i][j] = dp[i][j].max(coins);
                }
            }
        }

        dp[0][n + 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_coins() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
        assert_eq!(Solution::max_coins(vec![1, 5]), 10);
        assert_eq!(Solution::max_coins(vec![3]), 3);
        assert_eq!(Solution::max_coins(vec![2, 2]), 6);
        assert_eq!(Solution::max_coins(vec![1, 2, 3, 4]), 40);
    }
}
