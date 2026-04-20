/// leet 300

impl Solution {
    /// Patience sorting / DP with binary search. O(n log n) time, O(n) space.
    /// Maintain a `tails` vec where tails[i] is the smallest tail element
    /// for an increasing subsequence of length i+1.
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails: Vec<i32> = Vec::new();
        // O(n) outer loop — process each element once
        for &num in &nums {
            // O(log n) binary search for insertion point
            let pos = tails.partition_point(|&x| x < num);
            if pos == tails.len() {
                tails.push(num);
            } else {
                tails[pos] = num;
            }
        }
        tails.len() as i32
    }

    /// Classic DP. O(n^2) time, O(n) space.
    /// dp[i] = length of longest increasing subsequence ending at index i.
    pub fn length_of_lis_dp(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut dp = vec![1; n];
        let mut res = 1;
        // O(n) outer loop
        for i in 1..n {
            // O(n) inner loop — check all previous elements
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            res = res.max(dp[i]);
        }
        res
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_binary_search() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(Solution::length_of_lis(vec![1]), 1);
        assert_eq!(Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
    }

    #[test]
    fn test_classic_dp() {
        assert_eq!(Solution::length_of_lis_dp(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis_dp(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis_dp(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(Solution::length_of_lis_dp(vec![1]), 1);
        assert_eq!(Solution::length_of_lis_dp(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
    }
}
