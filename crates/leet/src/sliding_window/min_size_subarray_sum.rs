// leetcode 209

pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        // sliding window, O(n) time, O(1) space
        let target = target as i64;
        let mut min_len = usize::MAX;
        let mut left = 0;
        let mut sum = 0i64;

        for right in 0..nums.len() {
            // O(n) — right pointer visits each element once
            sum += nums[right] as i64;

            while sum >= target {
                // O(n) amortized — left moves at most n times total
                min_len = min_len.min(right - left + 1);
                sum -= nums[left] as i64;
                left += 1;
            }
        }

        if min_len == usize::MAX {
            0
        } else {
            min_len as i32
        }
    }

    pub fn min_sub_array_len_binary_search(target: i32, nums: Vec<i32>) -> i32 {
        // prefix sum + binary search, O(n log n) time, O(n) space
        let target = target as i64;
        let n = nums.len();

        // Build prefix sum array: prefix[i] = sum of nums[0..i]
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            // O(n) — build prefix sum
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let mut min_len = usize::MAX;

        for left in 0..n {
            // O(n log n) — binary search for each starting position
            let target_sum = prefix[left] + target;

            // Binary search for smallest right where prefix[right] >= target_sum
            let mut lo = left + 1;
            let mut hi = n + 1;

            while lo < hi {
                // O(log n) per iteration
                let mid = lo + (hi - lo) / 2;
                if prefix[mid] >= target_sum {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }

            if lo <= n && prefix[lo] >= target_sum {
                min_len = min_len.min(lo - left);
            }
        }

        if min_len == usize::MAX {
            0
        } else {
            min_len as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
        assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(Solution::min_sub_array_len(3, vec![1, 1]), 0);
        assert_eq!(Solution::min_sub_array_len(5, vec![5]), 1);
        assert_eq!(Solution::min_sub_array_len(6, vec![10, 2, 3]), 1);
    }

    #[test]
    fn test_binary_search() {
        assert_eq!(Solution::min_sub_array_len_binary_search(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len_binary_search(4, vec![1, 4, 4]), 1);
        assert_eq!(Solution::min_sub_array_len_binary_search(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::min_sub_array_len_binary_search(11, vec![1, 2, 3, 4, 5]), 3);
        assert_eq!(Solution::min_sub_array_len_binary_search(15, vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(Solution::min_sub_array_len_binary_search(3, vec![1, 1]), 0);
        assert_eq!(Solution::min_sub_array_len_binary_search(5, vec![5]), 1);
        assert_eq!(Solution::min_sub_array_len_binary_search(6, vec![10, 2, 3]), 1);
    }
}
