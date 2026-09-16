/// LeetCode 2962: Count Subarrays Where Max Element Appears at Least K Times

pub struct Solution;

impl Solution {
    /// Sliding window approach.
    /// O(n) time, O(1) space.
    /// For each right boundary, once we have >= k occurrences of the global max,
    /// shrink left until count < k. All starting positions in [0, left) are valid.
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_val = *nums.iter().max().unwrap(); // O(n) to find global max
        let k = k as usize;
        let mut count = 0usize; // occurrences of max_val in current window
        let mut left = 0usize;
        let mut res = 0i64;

        for right in 0..nums.len() {
            if nums[right] == max_val {
                count += 1;
            }
            // shrink window until count of max drops below k
            while count >= k {
                if nums[left] == max_val {
                    count -= 1;
                }
                left += 1;
            }
            // all subarrays starting at [0, left) with right endpoint are valid
            res += left as i64;
        }
        res
    }

    /// Binary search approach.
    /// O(n log n) time, O(n) space.
    /// Collect indices of max element. For each right, binary-search to check
    /// whether there are >= k max elements ending at right, and find the
    /// earliest valid left boundary.
    pub fn count_subarrays_bs(nums: Vec<i32>, k: i32) -> i64 {
        let max_val = *nums.iter().max().unwrap();
        let k = k as usize;
        // O(n) space: positions where max_val occurs
        let positions: Vec<usize> = nums
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v == max_val)
            .map(|(i, _)| i)
            .collect();

        let mut res = 0i64;
        for right in 0..nums.len() {
            // How many max positions are <= right?
            // partition_point returns the first index where pos > right, i.e. count of pos <= right
            let cnt = positions.partition_point(|&pos| pos <= right); // O(log n)
            if cnt >= k {
                // The k-th max from the right within [0..=right] is at positions[cnt - k].
                // Any left in [0, positions[cnt - k]] gives >= k maxes.
                res += (positions[cnt - k] + 1) as i64;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
        assert_eq!(Solution::count_subarrays_bs(vec![1, 3, 2, 3, 3], 2), 6);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::count_subarrays(vec![1, 4, 2, 1], 3), 0);
        assert_eq!(Solution::count_subarrays_bs(vec![1, 4, 2, 1], 3), 0);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::count_subarrays(vec![5, 5, 5, 5], 2), 6);
        assert_eq!(Solution::count_subarrays_bs(vec![5, 5, 5, 5], 2), 6);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::count_subarrays(vec![7], 1), 1);
        assert_eq!(Solution::count_subarrays_bs(vec![7], 1), 1);
    }

    #[test]
    fn test_max_at_ends() {
        assert_eq!(Solution::count_subarrays(vec![3, 1, 1, 3], 2), 1);
        assert_eq!(Solution::count_subarrays_bs(vec![3, 1, 1, 3], 2), 1);
    }

    #[test]
    fn test_alternating() {
        assert_eq!(Solution::count_subarrays(vec![2, 1, 2, 1, 2], 3), 1);
        assert_eq!(Solution::count_subarrays_bs(vec![2, 1, 2, 1, 2], 3), 1);
    }

    #[test]
    fn test_unique_max() {
        assert_eq!(Solution::count_subarrays(vec![1, 2, 3], 1), 3);
        assert_eq!(Solution::count_subarrays_bs(vec![1, 2, 3], 1), 3);
    }

    #[test]
    fn test_not_enough() {
        assert_eq!(Solution::count_subarrays(vec![5, 1, 5, 1], 3), 0);
        assert_eq!(Solution::count_subarrays_bs(vec![5, 1, 5, 1], 3), 0);
    }

    #[test]
    fn test_max_at_start() {
        assert_eq!(Solution::count_subarrays(vec![4, 4, 4, 1, 1], 2), 7);
        assert_eq!(Solution::count_subarrays_bs(vec![4, 4, 4, 1, 1], 2), 7);
    }
}
