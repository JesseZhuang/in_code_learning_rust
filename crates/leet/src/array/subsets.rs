/// LeetCode 78 - Subsets
/// Given an integer array nums of unique elements, return all possible subsets (the power set).

pub struct Solution;

impl Solution {
    /// Backtracking approach.
    /// Time: O(n * 2^n) — we generate 2^n subsets, each up to length n.
    /// Space: O(n) — recursion depth and current subset (excluding output).
    pub fn subsets_backtrack(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(&nums, 0, &mut current, &mut result);
        result
    }

    fn backtrack(nums: &[i32], start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone());
        for i in start..nums.len() {
            current.push(nums[i]);
            Self::backtrack(nums, i + 1, current, result);
            current.pop();
        }
    }

    /// Bit mask enumeration approach.
    /// Time: O(n * 2^n) — iterate over 2^n masks, each mask inspects n bits.
    /// Space: O(n * 2^n) — storing all subsets.
    pub fn subsets_bitmask(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let total = 1 << n; // 2^n
        let mut result = Vec::with_capacity(total);
        for mask in 0..total {
            let mut subset = Vec::new();
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    subset.push(nums[i]);
                }
            }
            result.push(subset);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Helper: convert Vec<Vec<i32>> to a set of sorted vecs for comparison.
    fn to_set(subsets: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
        subsets.into_iter().map(|mut v| { v.sort(); v }).collect()
    }

    #[test]
    fn test_subsets_123() {
        let nums = vec![1, 2, 3];
        let result_bt = Solution::subsets_backtrack(nums.clone());
        let result_bm = Solution::subsets_bitmask(nums);

        assert_eq!(result_bt.len(), 8);
        assert_eq!(result_bm.len(), 8);
        assert_eq!(to_set(result_bt.clone()), to_set(result_bm.clone()));

        // Check uniqueness
        assert_eq!(to_set(result_bt).len(), 8);
        assert_eq!(to_set(result_bm).len(), 8);
    }

    #[test]
    fn test_subsets_single_zero() {
        let nums = vec![0];
        let result_bt = Solution::subsets_backtrack(nums.clone());
        let result_bm = Solution::subsets_bitmask(nums);

        assert_eq!(result_bt.len(), 2); // [] and [0]
        assert_eq!(result_bm.len(), 2);
        assert_eq!(to_set(result_bt), to_set(result_bm));
    }

    #[test]
    fn test_subsets_empty() {
        let nums = vec![];
        let result_bt = Solution::subsets_backtrack(nums.clone());
        let result_bm = Solution::subsets_bitmask(nums);

        assert_eq!(result_bt.len(), 1); // only the empty subset
        assert_eq!(result_bm.len(), 1);
        assert_eq!(result_bt, vec![vec![] as Vec<i32>]);
        assert_eq!(result_bm, vec![vec![] as Vec<i32>]);
    }

    #[test]
    fn test_subsets_two_elements() {
        let nums = vec![1, 2];
        let result_bt = Solution::subsets_backtrack(nums.clone());
        let result_bm = Solution::subsets_bitmask(nums);

        assert_eq!(result_bt.len(), 4);
        assert_eq!(result_bm.len(), 4);
        assert_eq!(to_set(result_bt.clone()), to_set(result_bm.clone()));

        // Check uniqueness
        assert_eq!(to_set(result_bt).len(), 4);
        assert_eq!(to_set(result_bm).len(), 4);
    }

    #[test]
    fn test_subsets_ten_elements() {
        let nums: Vec<i32> = (1..=10).collect();
        let result_bt = Solution::subsets_backtrack(nums.clone());
        let result_bm = Solution::subsets_bitmask(nums);

        // 2^10 = 1024 subsets
        assert_eq!(result_bt.len(), 1024);
        assert_eq!(result_bm.len(), 1024);

        // All subsets are unique
        assert_eq!(to_set(result_bt.clone()).len(), 1024);
        assert_eq!(to_set(result_bm.clone()).len(), 1024);

        // Both methods produce the same set of subsets
        assert_eq!(to_set(result_bt), to_set(result_bm));
    }
}
