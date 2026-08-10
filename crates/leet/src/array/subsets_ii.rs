/// LeetCode 90 - Subsets II
/// Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
/// The solution set must not contain duplicate subsets.

pub struct Solution;

impl Solution {
    /// Backtracking approach: sort + skip duplicates at the same recursion level.
    /// Time: O(n * 2^n) — at most 2^n subsets, each copied in O(n).
    /// Space: O(n) — recursion depth and current subset (excluding output).
    pub fn subsets_with_dup_backtrack(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); // O(n log n) — required so duplicates are adjacent
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(&nums, 0, &mut current, &mut result);
        result
    }

    fn backtrack(nums: &[i32], start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone()); // O(n) — snapshot current subset
        for i in start..nums.len() {
            // Skip duplicate elements at the same decision level
            if i > start && nums[i] == nums[i - 1] {
                continue; // prune duplicate branch
            }
            current.push(nums[i]);
            Self::backtrack(nums, i + 1, current, result);
            current.pop(); // backtrack
        }
    }

    /// Iterative cascading approach for duplicates.
    /// When encountering a new unique element, extend ALL existing subsets.
    /// When encountering a duplicate, only extend subsets added in the previous round.
    /// Time: O(n * 2^n) — each subset is built by appending one element at a time.
    /// Space: O(n * 2^n) — storing all subsets.
    pub fn subsets_with_dup_cascade(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); // O(n log n) — duplicates must be adjacent
        let mut result: Vec<Vec<i32>> = vec![vec![]]; // start with empty subset
        let mut prev_new_start = 0; // index where last round's new subsets begin

        for i in 0..nums.len() {
            let start = if i > 0 && nums[i] == nums[i - 1] {
                prev_new_start // duplicate: only extend subsets from previous round
            } else {
                0 // new element: extend all existing subsets
            };
            let end = result.len(); // current size before adding new subsets
            for j in start..end {
                let mut subset = result[j].clone(); // O(k) per subset of size k
                subset.push(nums[i]);
                result.push(subset);
            }
            prev_new_start = end; // mark where this round's new subsets start
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Helper: normalize subsets for order-independent comparison.
    fn to_set(subsets: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
        subsets.into_iter().map(|mut v| { v.sort(); v }).collect()
    }

    fn run_both(nums: Vec<i32>, expected_count: usize) {
        let r1 = Solution::subsets_with_dup_backtrack(nums.clone());
        let r2 = Solution::subsets_with_dup_cascade(nums);

        assert_eq!(r1.len(), expected_count, "backtrack count mismatch");
        assert_eq!(r2.len(), expected_count, "cascade count mismatch");

        // Verify no duplicate subsets
        assert_eq!(to_set(r1.clone()).len(), expected_count);
        assert_eq!(to_set(r2.clone()).len(), expected_count);

        // Both methods produce same set
        assert_eq!(to_set(r1), to_set(r2));
    }

    #[test]
    fn test_122() {
        // [1,2,2] → [], [1], [2], [1,2], [2,2], [1,2,2]
        run_both(vec![1, 2, 2], 6);
    }

    #[test]
    fn test_single_zero() {
        // [0] → [], [0]
        run_both(vec![0], 2);
    }

    #[test]
    fn test_111() {
        // [1,1,1] → [], [1], [1,1], [1,1,1]
        run_both(vec![1, 1, 1], 4);
    }

    #[test]
    fn test_123_no_dups() {
        // [1,2,3] → all 2^3 = 8 subsets (no duplicates in input)
        run_both(vec![1, 2, 3], 8);
    }

    #[test]
    fn test_neg1_neg1_2() {
        // [-1,-1,2] → [], [-1], [-1,-1], [2], [-1,2], [-1,-1,2]
        run_both(vec![-1, -1, 2], 6);
    }

    #[test]
    fn test_five_pairs() {
        // [1,1,2,2,3,3,4,4,5,5] — each of 5 elements appears twice
        // Count = product of (freq_i + 1) = 3^5 = 243
        run_both(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 243);
    }
}
