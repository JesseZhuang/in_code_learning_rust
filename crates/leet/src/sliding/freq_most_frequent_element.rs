/// leet 1838
/// Frequency of the Most Frequent Element

/// O(n log n) time, O(1) space — sliding window
pub struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let k = k as i64;
        let (mut l, mut sum) = (0usize, 0i64);
        let mut res = 1;

        for r in 0..nums.len() {
            sum += nums[r] as i64;
            // cost to make all elements in [l..=r] equal to nums[r]
            while (nums[r] as i64) * (r - l + 1) as i64 - sum > k {
                sum -= nums[l] as i64;
                l += 1;
            }
            res = res.max(r - l + 1);
        }

        res as i32
    }
}

/// O(n log n) time, O(n) space — binary search + prefix sum
pub struct Solution2;

impl Solution2 {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let k = k as i64;
        // prefix[i] = sum of nums[0..i]
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let mut res = 1;
        for r in 0..n {
            // binary search for the smallest l such that we can make nums[l..=r] all equal to nums[r]
            let (mut lo, mut hi) = (0, r);
            while lo < hi {
                let mid = (lo + hi) / 2;
                let window = (r - mid + 1) as i64;
                let cost = nums[r] as i64 * window - (prefix[r + 1] - prefix[mid]);
                if cost <= k {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            res = res.max(r - lo + 1);
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cases() -> Vec<(Vec<i32>, i32, i32)> {
        vec![
            (vec![1, 2, 4], 5, 3),
            (vec![1, 4, 8, 13], 5, 2),
            (vec![3, 9, 6], 2, 1),
            (vec![7], 10, 1),               // single element
            (vec![5, 5, 5, 5], 0, 4),       // all same
            (vec![1, 2, 3], 0, 1),          // k = 0
            (vec![1, 2, 3, 4, 5], 100, 5), // large k covers all
            (vec![1, 1, 1, 2, 2, 2], 2, 5),
        ]
    }

    #[test]
    fn test_sliding_window() {
        for (nums, k, expected) in cases() {
            assert_eq!(
                Solution::max_frequency(nums.clone(), k),
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
                Solution2::max_frequency(nums.clone(), k),
                expected,
                "binary_search: nums={:?}, k={}",
                nums,
                k
            );
        }
    }
}
