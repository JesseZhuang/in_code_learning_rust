pub struct Solution;

impl Solution {
    /// Uses a running prefix maximum and precomputed suffix minima.
    /// Time: O(n); auxiliary space: O(n).
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut suffix_min = vec![0; n];
        suffix_min[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            suffix_min[i] = nums[i].min(suffix_min[i + 1]);
        }

        let mut prefix_max = nums[0];
        let mut sum = 0;
        for i in 1..n - 1 {
            if prefix_max < nums[i] && nums[i] < suffix_min[i + 1] {
                sum += 2;
            } else if nums[i - 1] < nums[i] && nums[i] < nums[i + 1] {
                sum += 1;
            }
            prefix_max = prefix_max.max(nums[i]);
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn length_three_counts_only_the_middle_element() {
        assert_eq!(Solution::sum_of_beauties(vec![1, 2, 3]), 2);
        assert_eq!(Solution::sum_of_beauties(vec![3, 2, 1]), 0);
        assert_eq!(Solution::sum_of_beauties(vec![1, 1, 2]), 0);
        assert_eq!(Solution::sum_of_beauties(vec![1, 2, 2]), 0);
    }

    #[test]
    fn local_increase_earns_one_when_global_condition_fails() {
        assert_eq!(Solution::sum_of_beauties(vec![2, 4, 6, 4]), 1);
    }

    #[test]
    fn global_condition_checks_every_earlier_value() {
        assert_eq!(Solution::sum_of_beauties(vec![4, 2, 3, 5]), 1);
    }

    #[test]
    fn global_condition_checks_every_later_value() {
        assert_eq!(Solution::sum_of_beauties(vec![1, 3, 4, 2]), 1);
    }

    #[test]
    fn equal_neighbors_do_not_earn_beauty() {
        assert_eq!(Solution::sum_of_beauties(vec![1, 2, 2, 3]), 0);
        assert_eq!(Solution::sum_of_beauties(vec![2, 2, 2, 2]), 0);
    }

    #[test]
    fn equal_distant_values_prevent_global_beauty() {
        assert_eq!(Solution::sum_of_beauties(vec![3, 1, 3, 4]), 1);
        assert_eq!(Solution::sum_of_beauties(vec![1, 3, 4, 3]), 1);
    }

    #[test]
    fn global_beauty_takes_precedence_over_local_beauty() {
        assert_eq!(Solution::sum_of_beauties(vec![1, 3, 4, 5]), 4);
    }

    #[test]
    fn maximum_length_increasing_input_counts_all_interior_elements() {
        assert_eq!(Solution::sum_of_beauties((1..=100_000).collect()), 199_996);
    }
}
