/// leet 1004
/// Max Consecutive Ones III

/// O(n) time, O(1) space — sliding window
pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut zeros = 0;
        let mut l = 0usize;
        let mut res = 0;

        for r in 0..nums.len() {
            if nums[r] == 0 {
                zeros += 1;
            }
            while zeros > k {
                if nums[l] == 0 {
                    zeros -= 1;
                }
                l += 1;
            }
            if r >= l {
                res = res.max(r - l + 1);
            }
        }

        res as i32
    }
}

/// O(n log n) time, O(n) space — binary search + prefix sum of zeros
pub struct Solution2;

impl Solution2 {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        // prefix[i] = number of zeros in nums[0..i]
        let mut prefix = vec![0i32; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + (1 - nums[i]);
        }

        let mut res = 0;
        for r in 0..n {
            // binary search for the smallest l such that zeros in nums[l..=r] <= k
            let (mut lo, mut hi) = (0, r + 1);
            while lo < hi {
                let mid = (lo + hi) / 2;
                let zeros = prefix[r + 1] - prefix[mid];
                if zeros <= k {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            if lo <= r {
                res = res.max(r - lo + 1);
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cases() -> Vec<(Vec<i32>, i32, i32)> {
        vec![
            (vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2, 6),
            (vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3, 10),
            (vec![1, 1, 1, 1, 1], 0, 5),
            (vec![0, 0, 0, 0], 4, 4),
            (vec![0, 0, 0, 0], 2, 2),
            (vec![1, 1, 1, 0, 1, 1], 0, 3),
            (vec![1], 0, 1),
            (vec![0], 1, 1),
            (vec![0], 0, 0),
            (vec![1, 0, 1, 0, 1], 3, 5),
            (vec![0, 1, 0, 1, 0, 1], 2, 5),
        ]
    }

    #[test]
    fn test_sliding_window() {
        for (nums, k, expected) in cases() {
            assert_eq!(
                Solution::longest_ones(nums.clone(), k),
                expected,
                "sliding_window: nums={:?}, k={}",
                nums,
                k
            );
        }
    }

    #[test]
    fn test_binary_search_prefix_sum() {
        for (nums, k, expected) in cases() {
            assert_eq!(
                Solution2::longest_ones(nums.clone(), k),
                expected,
                "binary_search: nums={:?}, k={}",
                nums,
                k
            );
        }
    }
}
