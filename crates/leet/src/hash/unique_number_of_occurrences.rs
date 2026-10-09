use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    // Expected O(n) time and O(n) space.
    pub fn unique_occurrences(nums: Vec<i32>) -> bool {
        let mut counts = HashMap::new();
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut frequencies = HashSet::new();
        counts.values().all(|&count| frequencies.insert(count))
    }

    // O(n + U) time and space, where U is the bounded value range.
    pub fn unique_occurrences_by_frequency_array(nums: Vec<i32>) -> bool {
        let mut counts = [0; 2001];
        for num in nums.iter() {
            counts[(num + 1000) as usize] += 1;
        }

        let mut seen_frequencies = vec![false; nums.len() + 1];
        for count in counts {
            if count > 0 {
                if seen_frequencies[count] {
                    return false;
                }
                seen_frequencies[count] = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_both_methods(nums: Vec<i32>, expected: bool) {
        assert_eq!(Solution::unique_occurrences(nums.clone()), expected);
        assert_eq!(
            Solution::unique_occurrences_by_frequency_array(nums),
            expected
        );
    }

    #[test]
    fn test_examples() {
        assert_both_methods(vec![1, 2, 2, 1, 1, 3], true);
        assert_both_methods(vec![1, 2], false);
        assert_both_methods(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0], true);
    }

    #[test]
    fn test_singleton_and_all_equal() {
        assert_both_methods(vec![7], true);
        assert_both_methods(vec![4, 4, 4, 4], true);
    }

    #[test]
    fn test_repeated_frequency_is_not_unique() {
        assert_both_methods(vec![1, 1, 2, 2], false);
    }

    #[test]
    fn test_inclusive_value_bounds_with_unique_frequencies() {
        assert_both_methods(vec![-1000, -1000, 1000], true);
    }

    #[test]
    fn test_inclusive_value_bounds_with_colliding_frequencies() {
        assert_both_methods(vec![-1000, -1000, 1000, 1000], false);
    }
}
