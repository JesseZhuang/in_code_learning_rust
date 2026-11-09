use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Sliding window + HashMap approach.
    /// O(n) time, O(n) space.
    /// The number of subarrays with exactly k distinct values is
    /// at_most(k) - at_most(k - 1).
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        Self::at_most(&nums, k) - Self::at_most(&nums, k - 1)
    }

    fn at_most(nums: &[i32], distinct_limit: usize) -> i32 {
        if distinct_limit == 0 {
            return 0;
        }

        let mut frequencies: HashMap<i32, usize> = HashMap::new();
        let mut left = 0usize;
        let mut count = 0usize;

        for right in 0..nums.len() {
            *frequencies.entry(nums[right]).or_insert(0) += 1;

            while frequencies.len() > distinct_limit {
                let left_value = nums[left];
                let frequency = frequencies.get_mut(&left_value).unwrap();
                *frequency -= 1;
                if *frequency == 0 {
                    frequencies.remove(&left_value);
                }
                left += 1;
            }

            count += right - left + 1;
        }

        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
    }

    #[test]
    fn test_exactly_one_distinct_value() {
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 1), 5);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![7, 7, 7], 1), 6);
    }

    #[test]
    fn test_k_equals_and_exceeds_actual_distinct_count() {
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1], 2), 3);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 1, 1], 3), 0);
    }

    #[test]
    fn test_duplicate_frequencies_are_removed_when_shrinking() {
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3], 2), 4);
    }

    #[test]
    fn test_maximum_size_single_value_input() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1; 20_000], 1),
            200_010_000
        );
    }
}
