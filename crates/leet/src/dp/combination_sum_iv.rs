/// leet 377

use std::collections::HashMap;

impl Solution {
    /// Bottom-up DP. O(N*target) time, O(target) space.
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let t = target as usize;
        let mut dp = vec![0i32; t + 1]; // dp[i]: number of combinations that sum to i
        dp[0] = 1;
        for i in 1..=t {
            for &num in &nums {
                let n = num as usize;
                if i >= n {
                    dp[i] += dp[i - n];
                }
            }
        }
        dp[t]
    }

    /// Top-down memoization. O(N*target) time, O(target) space.
    pub fn combination_sum4_memo(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = HashMap::new();
        memo.insert(0, 1);
        Self::dfs(&nums, target, &mut memo)
    }

    fn dfs(nums: &[i32], target: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&v) = memo.get(&target) {
            return v;
        }
        let mut count = 0;
        for &num in nums {
            if target >= num {
                count += Self::dfs(nums, target - num, memo);
            }
        }
        memo.insert(target, count);
        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_dp() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
        assert_eq!(Solution::combination_sum4(vec![1], 1), 1);
        assert_eq!(Solution::combination_sum4(vec![1, 2], 4), 5);
        assert_eq!(Solution::combination_sum4(vec![3, 1, 2, 4], 4), 8);
        assert_eq!(Solution::combination_sum4(vec![5, 1, 8], 24), 982);
    }

    #[test]
    fn test_memo() {
        assert_eq!(Solution::combination_sum4_memo(vec![1, 2, 3], 4), 7);
        assert_eq!(Solution::combination_sum4_memo(vec![9], 3), 0);
        assert_eq!(Solution::combination_sum4_memo(vec![1], 1), 1);
        assert_eq!(Solution::combination_sum4_memo(vec![1, 2], 4), 5);
        assert_eq!(Solution::combination_sum4_memo(vec![3, 1, 2, 4], 4), 8);
        assert_eq!(Solution::combination_sum4_memo(vec![5, 1, 8], 24), 982);
    }
}
