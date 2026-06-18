use std::cmp::max;

/// lc 55 medium

struct Solution {}

impl Solution {
    /// O(n) time, O(1) space, 8ms, 2.2Mb.
    pub fn dp1(nums: Vec<i32>) -> bool {
        let mut reach = 0;
        for (i, num) in nums.iter().enumerate() {
            if reach < i { return false; }
            reach = max(i + *num as usize, reach);
            if reach >= nums.len() - 1 { return true; }
        }
        true
    }

    /// O(n) time, O(1) space, 8ms, 2.2Mb.
    pub fn dp2(nums: Vec<i32>) -> bool {
        let mut smallest = nums.len() - 1; // smallest index can be reached
        for i in (0..nums.len() - 1).rev() {
            if i + nums[i] as usize >= smallest { smallest = i; }
        }
        smallest <= 0
    }

    /// dfs O(V+E) O(N^2) time and O(N) space, 396ms, 2.7Mb
    pub fn can_jump_dfs(nums: Vec<i32>) -> bool {
        let mut visited = vec![false; nums.len() - 1];
        Self::dfs(0, &mut visited, &nums)
    }

    fn dfs(i: usize, visited: &mut Vec<bool>, nums: &Vec<i32>) -> bool {
        if i >= nums.len() - 1 { return true; }
        if visited[i] { return false; }
        visited[i] = true;
        for j in 1..=nums[i] {
            if Self::dfs(j as usize + i, visited, nums) { return true; };
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::jump_game::Solution;

    #[test]
    fn test_can_jump() { assert!(Solution::dp2(vec![2, 3, 1, 1, 4])); }

    #[test]
    fn test_cannot_jump() { assert_eq!(false, Solution::can_jump_dfs(vec![3, 2, 1, 0, 4])); }

    #[test] // visited[1] out of bound
    fn test_can_jump1() { assert!(Solution::can_jump_dfs(vec![1, 2])); }

    #[test]
    fn test_single_zero() {
        assert!(Solution::dp1(vec![0]));
        assert!(Solution::dp2(vec![0]));
    }

    #[test]
    fn test_unreachable_short() {
        assert_eq!(false, Solution::dp1(vec![0, 1]));
        assert_eq!(false, Solution::dp2(vec![0, 1]));
    }

    #[test]
    fn test_large_first() {
        assert!(Solution::dp1(vec![10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
        assert!(Solution::dp2(vec![10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn test_stuck_at_zero() {
        assert_eq!(false, Solution::dp1(vec![1, 0, 0, 1]));
        assert_eq!(false, Solution::dp2(vec![1, 0, 0, 1]));
    }

    #[test]
    fn test_barely_reachable() {
        assert!(Solution::dp1(vec![1, 1, 1, 1, 1]));
        assert!(Solution::dp2(vec![1, 1, 1, 1, 1]));
    }
}
