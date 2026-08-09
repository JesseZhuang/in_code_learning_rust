/// LeetCode 410 - Split Array Largest Sum
///
/// Given an integer array `nums` and an integer `k`, split `nums` into `k`
/// non-empty subarrays such that the largest sum of any subarray is minimized.

pub struct Solution;

impl Solution {
    /// Binary search on answer approach.
    /// O(n * log(sum - max)) time, O(1) space.
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut lo = *nums.iter().max().unwrap() as i64; // minimum possible answer
        let mut hi = nums.iter().map(|&x| x as i64).sum::<i64>(); // maximum possible answer

        // O(log(sum - max)) iterations
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Self::can_split(&nums, k, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }

    /// Check if we can split nums into at most k subarrays
    /// each with sum <= max_sum.
    /// O(n) time.
    fn can_split(nums: &[i32], k: i32, max_sum: i64) -> bool {
        let mut count = 1;
        let mut current_sum: i64 = 0;
        // O(n) scan
        for &num in nums {
            current_sum += num as i64;
            if current_sum > max_sum {
                count += 1;
                current_sum = num as i64;
                if count > k {
                    return false;
                }
            }
        }
        true
    }

    /// DP approach.
    /// dp[i][j] = minimum largest sum to split nums[0..i] into j parts.
    /// O(n^2 * k) time, O(n * k) space.
    pub fn split_array_dp(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;

        // prefix[i] = sum of nums[0..i]
        let mut prefix = vec![0i64; n + 1];
        // O(n) prefix sum
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        // dp[i][j] = min largest subarray sum splitting first i elements into j groups
        // O(n * k) space
        let mut dp = vec![vec![i64::MAX; k + 1]; n + 1];
        dp[0][0] = 0;

        // O(n) outer loop over elements
        for i in 1..=n {
            // O(k) loop over number of splits
            for j in 1..=k.min(i) {
                // O(n) loop over split point
                for m in (j - 1)..i {
                    // split: first m elements in j-1 groups, elements m..i in last group
                    if dp[m][j - 1] != i64::MAX {
                        let last_sum = prefix[i] - prefix[m];
                        let candidate = dp[m][j - 1].max(last_sum);
                        dp[i][j] = dp[i][j].min(candidate);
                    }
                }
            }
        }

        dp[n][k] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_array() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(Solution::split_array(vec![10], 1), 10);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 5), 5);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 1), 15);
        assert_eq!(Solution::split_array(vec![3, 3, 3, 3], 2), 6);
        assert_eq!(Solution::split_array(vec![1000000, 1, 1], 2), 1000000);
    }

    #[test]
    fn test_split_array_dp() {
        assert_eq!(Solution::split_array_dp(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array_dp(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(Solution::split_array_dp(vec![10], 1), 10);
        assert_eq!(Solution::split_array_dp(vec![1, 2, 3, 4, 5], 5), 5);
        assert_eq!(Solution::split_array_dp(vec![1, 2, 3, 4, 5], 1), 15);
        assert_eq!(Solution::split_array_dp(vec![3, 3, 3, 3], 2), 6);
        assert_eq!(Solution::split_array_dp(vec![1000000, 1, 1], 2), 1000000);
    }
}
