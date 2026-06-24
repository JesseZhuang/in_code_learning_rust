/// leet 494

use std::collections::HashMap;

impl Solution {
    /// Subset sum DP. Transform: P = (total + target) / 2, count subsets summing to P.
    /// O(n * P) time, O(P) space.
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let total: i32 = nums.iter().sum();
        if target.abs() > total || (total + target) % 2 != 0 { // early prune
            return 0;
        }
        let p = ((total + target) / 2) as usize;
        let mut dp = vec![0i32; p + 1]; // dp[j]: # ways to form sum j
        dp[0] = 1;
        for &num in &nums { // O(n)
            let n = num as usize;
            for j in (n..=p).rev() { // O(P), iterate backwards to avoid reuse
                dp[j] += dp[j - n]; // subset inclusion
            }
        }
        dp[p]
    }

    /// DFS + memoization. O(n * s) time/space, s = range of reachable sums.
    pub fn find_target_sum_ways_dfs(nums: Vec<i32>, target: i32) -> i32 {
        let total: i32 = nums.iter().sum();
        if target.abs() > total { // early prune
            return 0;
        }
        let mut memo = HashMap::new();
        Self::dfs(&nums, 0, target, &mut memo)
    }

    fn dfs(nums: &[i32], idx: usize, remain: i32, memo: &mut HashMap<(usize, i32), i32>) -> i32 {
        if idx == nums.len() {
            return if remain == 0 { 1 } else { 0 };
        }
        if let Some(&v) = memo.get(&(idx, remain)) { // O(1) lookup
            return v;
        }
        let add = Self::dfs(nums, idx + 1, remain - nums[idx], memo); // +nums[idx]
        let sub = Self::dfs(nums, idx + 1, remain + nums[idx], memo); // -nums[idx]
        let res = add + sub;
        memo.insert((idx, remain), res); // memoize
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_dp() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 0], 1), 2);
        assert_eq!(Solution::find_target_sum_ways(vec![0, 0, 0, 0, 0], 0), 32);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 2, 3], 7), 0);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], -3), 5);
        assert_eq!(Solution::find_target_sum_ways(vec![0], 0), 2);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 2, 1], 0), 2);
    }

    #[test]
    fn test_dfs() {
        assert_eq!(Solution::find_target_sum_ways_dfs(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(Solution::find_target_sum_ways_dfs(vec![1], 1), 1);
        assert_eq!(Solution::find_target_sum_ways_dfs(vec![1, 0], 1), 2);
        assert_eq!(Solution::find_target_sum_ways_dfs(vec![0, 0, 0, 0, 0], 0), 32);
        assert_eq!(Solution::find_target_sum_ways_dfs(vec![1, 2, 3], 7), 0);
        assert_eq!(Solution::find_target_sum_ways_dfs(vec![1, 1, 1, 1, 1], -3), 5);
        assert_eq!(Solution::find_target_sum_ways_dfs(vec![0], 0), 2);
        assert_eq!(Solution::find_target_sum_ways_dfs(vec![1, 2, 1], 0), 2);
    }
}
