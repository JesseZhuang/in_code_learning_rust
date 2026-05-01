//! LeetCode 1 similar (Salesforce variant), medium, tags: array, hash table.
//!
//! Given array `a` and target `t`, return the number of ordered pairs `(i, j)`
//! where `a[i] - a[j] == t`. Duplicates contribute with multiplicity.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Count pairs whose difference equals `target`.
    /// Time O(n), Space O(n).
    pub fn two_subtract(a: Vec<i32>, target: i32) -> i64 {
        let mut cnt: HashMap<i32, i64> = HashMap::new();
        for v in &a {
            *cnt.entry(*v).or_insert(0) += 1; // O(n)
        }
        let mut res: i64 = 0;
        for (v, c) in &cnt {
            // Look forward only (v + target) to avoid double counting.
            if let Some(other) = cnt.get(&(v + target)) {
                res += c * other;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn no_pair() {
        assert_eq!(Solution::two_subtract(vec![1, 2], 3), 0);
    }

    #[test]
    fn single_pair() {
        assert_eq!(Solution::two_subtract(vec![1, 2], 1), 1);
    }

    #[test]
    fn zero_adds() {
        assert_eq!(Solution::two_subtract(vec![1, 2, 0], 1), 2);
    }

    #[test]
    fn duplicates_multiply_counts() {
        assert_eq!(Solution::two_subtract(vec![1, 2, 1, 2, 0], 1), 6);
    }

    #[test]
    fn empty() {
        assert_eq!(Solution::two_subtract(vec![], 1), 0);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::two_subtract(vec![5], 0), 1); // (5,5) self-pair when T=0
    }

    #[test]
    fn negative_values() {
        assert_eq!(Solution::two_subtract(vec![-1, -3, -2], 1), 2);
    }

    #[test]
    fn negative_target() {
        assert_eq!(Solution::two_subtract(vec![1, 2], -1), 1);
    }

    #[test]
    fn all_same_target_zero() {
        // cnt = {5:4}; v=5 -> look 5 -> 4*4 = 16.
        assert_eq!(Solution::two_subtract(vec![5, 5, 5, 5], 0), 16);
    }

    #[test]
    fn multiple_distinct_pairs() {
        assert_eq!(Solution::two_subtract(vec![10, 7, 3, 0, 13, 17], 3), 3);
    }

    #[test]
    fn no_match_large_target() {
        assert_eq!(Solution::two_subtract(vec![1, 2, 3, 4, 5], 100), 0);
    }
}
