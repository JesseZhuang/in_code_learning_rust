/// LeetCode 2616 - Minimize the Maximum Difference of Pairs
///
/// Given an integer array `nums` and an integer `p`, find `p` pairs of indices
/// such that the maximum difference among all pairs is minimized.
/// Return the minimum maximum difference.

pub struct Solution;

impl Solution {
    /// Sort + Binary search on answer + Greedy.
    /// O(n log n + n log M) time where M = max - min, O(1) extra space.
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        let p = p as usize;
        if p == 0 {
            return 0;
        }

        nums.sort_unstable(); // O(n log n)

        let mut lo = 0i32;
        let mut hi = nums[nums.len() - 1] - nums[0]; // max possible difference

        // O(log M) iterations of binary search
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Self::can_form_pairs(&nums, p, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    /// Greedily count how many pairs with difference <= threshold we can form.
    /// O(n) time.
    fn can_form_pairs(nums: &[i32], p: usize, threshold: i32) -> bool {
        let mut count = 0;
        let mut i = 0;
        // O(n) greedy scan
        while i + 1 < nums.len() {
            if nums[i + 1] - nums[i] <= threshold {
                count += 1;
                if count >= p {
                    return true;
                }
                i += 2; // skip both elements in the pair
            } else {
                i += 1;
            }
        }
        count >= p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimize_max() {
        assert_eq!(Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
        assert_eq!(Solution::minimize_max(vec![4, 2, 1, 2], 1), 0);
        assert_eq!(Solution::minimize_max(vec![5, 3, 1], 0), 0);
        assert_eq!(Solution::minimize_max(vec![1, 5], 1), 4);
        assert_eq!(Solution::minimize_max(vec![3, 3, 3, 3], 2), 0);
        assert_eq!(Solution::minimize_max(vec![1, 2, 3, 4, 5, 6], 3), 1);
        assert_eq!(Solution::minimize_max(vec![0, 0], 1), 0);
    }
}
