/// leetcode 1493

pub struct Solution;

impl Solution {
    /// One-zero sliding window, O(n) time and O(1) space.
    pub fn longest_subarray(nums: &[i32]) -> i32 {
        let (mut left, mut zero_count, mut max_window) = (0usize, 0usize, 0usize);

        for (right, &value) in nums.iter().enumerate() {
            if value == 0 {
                zero_count += 1;
            }

            while zero_count > 1 {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }

            max_window = max_window.max(right - left + 1);
        }

        max_window.saturating_sub(1) as i32
    }

    /// Prefix/suffix consecutive-one counts, O(n) time and O(n) space.
    pub fn longest_subarray_prefix_suffix(nums: &[i32]) -> i32 {
        let n = nums.len();
        let mut prefix_ones = vec![0usize; n + 1];
        let mut suffix_ones = vec![0usize; n + 1];

        for i in 0..n {
            if nums[i] == 1 {
                prefix_ones[i + 1] = prefix_ones[i] + 1;
            }
        }

        for i in (0..n).rev() {
            if nums[i] == 1 {
                suffix_ones[i] = suffix_ones[i + 1] + 1;
            }
        }

        let mut longest = 0usize;
        for i in 0..n {
            longest = longest.max(prefix_ones[i] + suffix_ones[i + 1]);
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_both_methods(nums: &[i32], expected: i32) {
        assert_eq!(Solution::longest_subarray(nums), expected);
        assert_eq!(Solution::longest_subarray_prefix_suffix(nums), expected);
    }

    #[test]
    fn examples() {
        assert_both_methods(&[1, 1, 0, 1], 3);
        assert_both_methods(&[0, 1, 1, 1, 0, 1, 1, 0, 1], 5);
    }

    #[test]
    fn all_ones_deletes_one_element() {
        assert_both_methods(&[1, 1, 1, 1], 3);
    }

    #[test]
    fn all_zeros_returns_zero() {
        assert_both_methods(&[0, 0, 0], 0);
    }

    #[test]
    fn singleton_deletes_its_only_element() {
        assert_both_methods(&[1], 0);
        assert_both_methods(&[0], 0);
    }

    #[test]
    fn multiple_zeros_allow_only_one_deletion() {
        assert_both_methods(&[1, 0, 1, 1, 0, 1], 3);
    }
}
