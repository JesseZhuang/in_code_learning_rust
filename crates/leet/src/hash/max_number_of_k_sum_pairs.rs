use std::collections::HashMap;

/// LeetCode 1679 - Max Number of K-Sum Pairs
pub struct Solution;

impl Solution {
    /// Solution 1: Single-pass hash map.
    /// Time O(n), Space O(n).
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new(); // value -> remaining count
        let mut ops = 0;
        for &n in &nums {
            let complement = k - n; // O(1) complement lookup
            if let Some(c) = cnt.get_mut(&complement) {
                if *c > 0 {
                    *c -= 1; // consume one complement
                    ops += 1;
                    continue;
                }
            }
            *cnt.entry(n).or_insert(0) += 1; // store for future pairing
        }
        ops
    }

    /// Solution 2: Sort + two pointers.
    /// Time O(n log n), Space O(1) extra (in-place sort).
    pub fn max_operations_two_ptr(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable(); // O(n log n)
        let (mut lo, mut hi) = (0usize, nums.len().wrapping_sub(1));
        let mut ops = 0;
        while lo < hi && hi < nums.len() {
            // hi < nums.len() guards wrapping_sub on empty input
            let sum = nums[lo] + nums[hi];
            if sum == k {
                ops += 1;
                lo += 1;
                hi = hi.wrapping_sub(1); // move both inward
            } else if sum < k {
                lo += 1; // need larger sum
            } else {
                hi = hi.wrapping_sub(1); // need smaller sum
            }
        }
        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(2, Solution::max_operations(vec![1, 2, 3, 4], 5));
        assert_eq!(2, Solution::max_operations_two_ptr(vec![1, 2, 3, 4], 5));
    }

    #[test]
    fn test_example2() {
        assert_eq!(1, Solution::max_operations(vec![3, 1, 3, 4, 3], 6));
        assert_eq!(1, Solution::max_operations_two_ptr(vec![3, 1, 3, 4, 3], 6));
    }

    #[test]
    fn test_no_pairs() {
        assert_eq!(0, Solution::max_operations(vec![1, 2, 3], 10));
        assert_eq!(0, Solution::max_operations_two_ptr(vec![1, 2, 3], 10));
    }

    #[test]
    fn test_all_pairs() {
        assert_eq!(2, Solution::max_operations(vec![1, 4, 4, 1], 5));
        assert_eq!(2, Solution::max_operations_two_ptr(vec![1, 4, 4, 1], 5));
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(2, Solution::max_operations(vec![2, 2, 2, 2], 4));
        assert_eq!(2, Solution::max_operations_two_ptr(vec![2, 2, 2, 2], 4));
    }

    #[test]
    fn test_single_element() {
        assert_eq!(0, Solution::max_operations(vec![5], 5));
        assert_eq!(0, Solution::max_operations_two_ptr(vec![5], 5));
    }

    #[test]
    fn test_odd_count() {
        assert_eq!(1, Solution::max_operations(vec![2, 2, 2], 4));
        assert_eq!(1, Solution::max_operations_two_ptr(vec![2, 2, 2], 4));
    }
}
