/// leet 1248: https://leetcode.com/problems/count-number-of-nice-subarrays/
pub struct Solution;

impl Solution {
    /// Prefix odd-count frequencies. O(n) time, O(n) space.
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut frequencies = vec![0; nums.len() + 1];
        frequencies[0] = 1;
        let (mut odds, mut count) = (0usize, 0);
        let k = k as usize;
        // Visit each value once, with O(1) frequency lookup and update.
        for num in nums {
            odds += (num & 1) as usize;
            if odds >= k {
                count += frequencies[odds - k];
            }
            frequencies[odds] += 1;
        }
        count
    }

    /// exactly(k) = at_most(k) - at_most(k - 1). O(n) time, O(1) space.
    pub fn number_of_subarrays_sliding(nums: Vec<i32>, k: i32) -> i32 {
        fn at_most(nums: &[i32], k: i32) -> i32 {
            let (mut left, mut odds, mut count) = (0, 0, 0);
            // Both pointers advance at most n times, so this pass is O(n).
            for right in 0..nums.len() {
                odds += nums[right] & 1;
                while odds > k {
                    odds -= nums[left] & 1;
                    left += 1;
                }
                count += (right + 1 - left) as i32;
            }
            count
        }
        at_most(&nums, k) - at_most(&nums, k - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_both(nums: Vec<i32>, k: i32, expected: i32) {
        assert_eq!(Solution::number_of_subarrays(nums.clone(), k), expected);
        assert_eq!(Solution::number_of_subarrays_sliding(nums, k), expected);
    }

    #[test]
    fn examples() {
        assert_both(vec![1, 1, 2, 1, 1], 3, 2);
        assert_both(vec![2, 4, 6], 1, 0);
        assert_both(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2, 16);
    }

    #[test]
    fn no_odd_values() {
        assert_both(vec![2, 4, 100_000, 6], 2, 0);
    }

    #[test]
    fn all_odds() {
        assert_both(vec![1, 3, 5, 7], 2, 3);
    }

    #[test]
    fn even_runs_around_odds() {
        assert_both(vec![2, 4, 1, 6, 8], 1, 9);
        assert_both(vec![2, 1, 2, 2, 3, 2], 1, 12);
    }

    #[test]
    fn k_equals_total_odd_count() {
        assert_both(vec![2, 1, 2, 3, 2], 2, 4);
        assert_both(vec![1, 3, 5], 3, 1);
    }

    #[test]
    fn k_greater_than_total_odd_count() {
        assert_both(vec![2, 1, 2, 3, 2], 3, 0);
    }

    #[test]
    fn single_odd() {
        assert_both(vec![99_999], 1, 1);
    }

    #[test]
    fn large_even_runs_around_one_odd() {
        let mut nums = vec![100_000; 49_999];
        nums[24_999] = 99_999;
        assert_both(nums, 1, 625_000_000);
    }
}
