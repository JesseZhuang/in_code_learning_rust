pub struct Solution;

impl Solution {
    /// O(n) time and O(1) auxiliary space.
    pub fn min_operations(nums: &[i32]) -> i32 {
        let mut previous = nums[0];
        let mut operations = 0;

        for &original in &nums[1..] {
            let adjusted = original.max(previous + 1);
            operations += adjusted - original;
            previous = adjusted;
        }

        operations
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn all_equal_example_requires_cumulative_increments() {
        assert_eq!(Solution::min_operations(&[1, 1, 1]), 3);
    }

    #[test]
    fn mixed_example_counts_each_adjustment() {
        assert_eq!(Solution::min_operations(&[1, 5, 2, 4, 1]), 14);
    }

    #[test]
    fn single_element_requires_no_operations() {
        assert_eq!(Solution::min_operations(&[8]), 0);
    }

    #[test]
    fn strictly_increasing_values_require_no_operations() {
        assert_eq!(Solution::min_operations(&[1, 2, 3, 4, 5]), 0);
        assert_eq!(Solution::min_operations(&[1, 4, 10, 10_000]), 0);
    }

    #[test]
    fn equal_neighbors_must_become_strictly_increasing() {
        assert_eq!(Solution::min_operations(&[5, 5]), 1);
        assert_eq!(Solution::min_operations(&[2, 2, 3]), 2);
    }

    #[test]
    fn descending_values_keep_the_original_order() {
        assert_eq!(Solution::min_operations(&[5, 4, 3, 2, 1]), 20);
    }

    #[test]
    fn adjustments_cascade_from_the_previous_adjusted_value() {
        assert_eq!(Solution::min_operations(&[3, 1, 2, 2]), 10);
        assert_eq!(Solution::min_operations(&[1, 1, 2, 3]), 3);
    }

    #[test]
    fn larger_later_value_resets_the_threshold() {
        assert_eq!(Solution::min_operations(&[3, 1, 10, 1, 1]), 24);
        assert_eq!(Solution::min_operations(&[1, 1, 10, 2]), 10);
    }

    #[test]
    fn minimum_and_maximum_values_use_the_same_greedy_rule() {
        const CASES: &[(&[i32], i32)] = &[
            (&[1], 0),
            (&[10_000], 0),
            (&[1, 10_000], 0),
            (&[10_000, 1], 10_000),
            (&[10_000, 10_000], 1),
            (&[1, 1], 1),
        ];

        for &(nums, expected) in CASES {
            assert_eq!(Solution::min_operations(nums), expected, "input: {nums:?}");
        }
    }

    #[test]
    fn maximum_length_minimum_values_accumulate_all_increments() {
        let nums = vec![1; 5_000];

        assert_eq!(Solution::min_operations(&nums), 12_497_500);
    }

    #[test]
    fn maximum_length_maximum_values_can_grow_past_the_input_bound() {
        let nums = vec![10_000; 5_000];

        assert_eq!(Solution::min_operations(&nums), 12_497_500);
    }

    #[test]
    fn maximum_operations_fit_in_i32() {
        let mut nums = vec![1; 5_000];
        nums[0] = 10_000;

        assert_eq!(Solution::min_operations(&nums), 62_482_501);
    }

    #[test]
    fn repeated_invocations_do_not_share_adjustment_state() {
        let nums = [1, 1, 1];

        assert_eq!(Solution::min_operations(&nums), 3);
        assert_eq!(Solution::min_operations(&[8]), 0);
        assert_eq!(Solution::min_operations(&nums), 3);
    }

    #[test]
    fn borrowed_subslice_uses_only_its_own_values() {
        let nums = [10_000, 1, 1, 1, 10_000];

        assert_eq!(Solution::min_operations(&nums[1..4]), 3);
    }

    #[test]
    fn caller_retains_unchanged_owned_input() {
        let mut nums = vec![1, 5, 2, 4, 1];

        assert_eq!(Solution::min_operations(&nums), 14);
        assert_eq!(nums, [1, 5, 2, 4, 1]);

        nums.push(9);
        assert_eq!(Solution::min_operations(&nums), 14);
        assert_eq!(nums, [1, 5, 2, 4, 1, 9]);
    }
}
