use std::collections::HashMap;

/// leet 930

pub struct Solution;

/// Approach 1: Prefix Sum + HashMap. O(n) time, O(n) space.
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);
        let (mut prefix, mut count) = (0, 0);
        for n in nums {
            prefix += n;
            if let Some(&c) = map.get(&(prefix - goal)) {
                count += c;
            }
            *map.entry(prefix).or_insert(0) += 1;
        }
        count
    }

    /// Approach 2: Sliding Window — exactly(goal) = at_most(goal) - at_most(goal-1). O(n) time, O(1) space.
    pub fn num_subarrays_with_sum_sliding(nums: Vec<i32>, goal: i32) -> i32 {
        fn at_most(nums: &[i32], goal: i32) -> i32 {
            if goal < 0 {
                return 0;
            }
            let (mut left, mut sum, mut count) = (0, 0, 0);
            for right in 0..nums.len() {
                sum += nums[right];
                while sum > goal {
                    sum -= nums[left];
                    left += 1;
                }
                count += (right + 1 - left) as i32;
            }
            count
        }
        at_most(&nums, goal) - at_most(&nums, goal - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
        assert_eq!(Solution::num_subarrays_with_sum_sliding(vec![1, 0, 1, 0, 1], 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
        assert_eq!(Solution::num_subarrays_with_sum_sliding(vec![0, 0, 0, 0, 0], 0), 15);
    }

    #[test]
    fn single_one_goal_one() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1], 1), 1);
        assert_eq!(Solution::num_subarrays_with_sum_sliding(vec![1], 1), 1);
    }

    #[test]
    fn single_zero_goal_zero() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![0], 0), 1);
        assert_eq!(Solution::num_subarrays_with_sum_sliding(vec![0], 0), 1);
    }

    #[test]
    fn all_ones() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 1, 1, 1], 2), 3);
        assert_eq!(Solution::num_subarrays_with_sum_sliding(vec![1, 1, 1, 1], 2), 3);
    }

    #[test]
    fn goal_zero_with_mixed() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 0, 1], 0), 3);
        assert_eq!(Solution::num_subarrays_with_sum_sliding(vec![1, 0, 0, 1], 0), 3);
    }

    #[test]
    fn leading_trailing_zeros() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 1, 0, 0], 1), 9);
        assert_eq!(Solution::num_subarrays_with_sum_sliding(vec![0, 0, 1, 0, 0], 1), 9);
    }

    #[test]
    fn no_valid_subarray() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0], 1), 0);
        assert_eq!(Solution::num_subarrays_with_sum_sliding(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn large_goal() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1], 5), 0);
        assert_eq!(Solution::num_subarrays_with_sum_sliding(vec![1, 0, 1], 5), 0);
    }
}
