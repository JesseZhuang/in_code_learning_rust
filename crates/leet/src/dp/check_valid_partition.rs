pub struct Solution;

impl Solution {
    /// Rolling prefix DP: O(n) time and O(1) auxiliary space.
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let (mut dp0, mut dp1, mut dp2) = (true, false, nums[0] == nums[1]);

        for i in 3..=nums.len() {
            // Before this step, the flags represent dp[i-3], dp[i-2], dp[i-1].
            let equal_pair = nums[i - 2] == nums[i - 1];
            let equal_triple = nums[i - 3] == nums[i - 2] && equal_pair;
            let consecutive_triple =
                nums[i - 3] + 1 == nums[i - 2] && nums[i - 2] + 1 == nums[i - 1];
            let current = (dp1 && equal_pair) || (dp0 && (equal_triple || consecutive_triple));
            (dp0, dp1, dp2) = (dp1, dp2, current);
        }

        dp2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn official_examples() {
        assert!(Solution::valid_partition(vec![4, 4, 4, 5, 6]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 2]));
    }

    #[test]
    fn minimum_length() {
        assert!(Solution::valid_partition(vec![1, 1]));
        assert!(!Solution::valid_partition(vec![1, 2]));
    }

    #[test]
    fn equal_triple() {
        assert!(Solution::valid_partition(vec![7, 7, 7]));
    }

    #[test]
    fn consecutive_triple() {
        assert!(Solution::valid_partition(vec![1, 2, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 4]));
        assert!(!Solution::valid_partition(vec![1, 3, 4]));
        assert!(!Solution::valid_partition(vec![3, 2, 1]));
    }

    #[test]
    fn multiple_pairs() {
        assert!(Solution::valid_partition(vec![1, 1, 1, 1]));
        assert!(Solution::valid_partition(vec![1, 1, 2, 2, 3, 3]));
    }

    #[test]
    fn pair_before_consecutive_triple() {
        assert!(Solution::valid_partition(vec![1, 1, 1, 2, 3]));
    }

    #[test]
    fn valid_suffix_cannot_repair_invalid_prefix() {
        assert!(!Solution::valid_partition(vec![1, 2, 2]));
    }

    #[test]
    fn maximum_values() {
        assert!(Solution::valid_partition(vec![999_998, 999_999, 1_000_000]));
        assert!(Solution::valid_partition(vec![1_000_000, 1_000_000]));
    }

    #[test]
    fn maximum_length() {
        assert!(Solution::valid_partition(vec![1_000_000; 100_000]));

        let mut invalid = vec![1; 100_000];
        invalid[99_999] = 2;
        assert!(!Solution::valid_partition(invalid));
    }
}
