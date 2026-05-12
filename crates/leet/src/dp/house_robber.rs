/// leet 198

impl Solution {
    /// Bottom-up DP with two variables. O(N) time, O(1) space.
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut rob_prev, mut n_rob_prev) = (0, 0);
        for &n in &nums { // O(N)
            let rob_cur = n_rob_prev + n;
            n_rob_prev = n_rob_prev.max(rob_prev);
            rob_prev = rob_cur;
        }
        rob_prev.max(n_rob_prev)
    }

    /// DP with array. O(N) time, O(N) space.
    pub fn rob_dp_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        if n == 1 { return nums[0]; }
        let mut dp = vec![0; n]; // dp[i]: max money robbing from houses 0..=i
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..n { // O(N)
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }
        dp[n - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![5]), 5);
        assert_eq!(Solution::rob(vec![1, 2]), 2);
        assert_eq!(Solution::rob(vec![3, 3, 3, 3]), 6);
        assert_eq!(Solution::rob(vec![100, 1, 100, 1, 100]), 300);
        assert_eq!(Solution::rob(vec![]), 0);
    }

    #[test]
    fn test_rob_dp_array() {
        assert_eq!(Solution::rob_dp_array(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob_dp_array(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob_dp_array(vec![5]), 5);
        assert_eq!(Solution::rob_dp_array(vec![1, 2]), 2);
        assert_eq!(Solution::rob_dp_array(vec![3, 3, 3, 3]), 6);
        assert_eq!(Solution::rob_dp_array(vec![100, 1, 100, 1, 100]), 300);
        assert_eq!(Solution::rob_dp_array(vec![]), 0);
    }
}
