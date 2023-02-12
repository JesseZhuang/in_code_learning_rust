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
        return true;
    }

    /// O(n) time, O(1) space, 8ms, 2.2Mb.
    pub fn dp2(nums: Vec<i32>) -> bool {
        let mut smallest = nums.len() - 1; // smallest index can be reached
        for i in (0..nums.len() - 1).rev() {
            if i + nums[i] as usize >= smallest { smallest = i; }
        }
        return smallest <= 0;
    }

    /// dfs O(V+E) O(N^2) time and O(N) space, 396ms, 2.7Mb
    pub fn can_jump_dfs(nums: Vec<i32>) -> bool {
        let mut visited = vec![false; nums.len() - 1];
        return Self::dfs(0, &mut visited, &nums);
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
}
