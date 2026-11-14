use std::collections::HashMap;

/// LeetCode 1658: Minimum Operations to Reduce X to Zero.
pub struct Solution;

impl Solution {
    /// Keep the longest window summing to total - x; nums contains positive values.
    /// O(n) time and O(1) extra space.
    pub fn min_operations(nums: &[i32], x: i32) -> i32 {
        let target = nums.iter().map(|&num| i64::from(num)).sum::<i64>() - i64::from(x);
        if target < 0 {
            return -1;
        }
        if target == 0 {
            return nums.len() as i32;
        }

        let (mut left, mut sum, mut longest) = (0, 0_i64, None);
        for (right, &num) in nums.iter().enumerate() {
            sum += i64::from(num);
            while sum > target {
                sum -= i64::from(nums[left]);
                left += 1;
            }
            if sum == target {
                longest = Some(longest.unwrap_or(0).max(right + 1 - left));
            }
        }
        longest.map_or(-1, |length| (nums.len() - length) as i32)
    }

    /// Earliest prefix indices maximize the retained subarray's length.
    /// O(n) expected time and O(n) extra space.
    pub fn min_operations_by_prefix_sum(nums: &[i32], x: i32) -> i32 {
        let target = nums.iter().map(|&num| i64::from(num)).sum::<i64>() - i64::from(x);
        if target < 0 {
            return -1;
        }

        let mut earliest = HashMap::from([(0_i64, 0_usize)]);
        let mut prefix = 0_i64;
        let mut longest = if target == 0 { Some(0) } else { None };
        for (index, &num) in nums.iter().enumerate() {
            prefix += i64::from(num);
            if let Some(&start) = earliest.get(&(prefix - target)) {
                longest = Some(longest.unwrap_or(0).max(index + 1 - start));
            }
            earliest.entry(prefix).or_insert(index + 1);
        }
        longest.map_or(-1, |length| (nums.len() - length) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn x_exceeding_total_is_impossible() {
        assert_eq!(Solution::min_operations(&[1, 2, 3], 7), -1);
        assert_eq!(Solution::min_operations_by_prefix_sum(&[1, 2, 3], 7), -1);
    }

    #[test]
    fn sliding_window_minimum_operations() {
        for (nums, x, expected) in [
            (vec![1, 1, 4, 2, 3], 5, 2),
            (vec![5, 6, 7, 8, 9], 4, -1),
            (vec![3, 2, 20, 1, 1, 3], 10, 5),
            (vec![1, 2, 3], 6, 3),
            (vec![1, 2, 3], 0, 0),
            (vec![1, 2, 3], 2, -1),
            (vec![1], 1, 1),
        ] {
            assert_eq!(
                Solution::min_operations(&nums, x),
                expected,
                "{nums:?}, x={x}"
            );
        }
    }

    #[test]
    fn prefix_sum_minimum_operations() {
        for (nums, x, expected) in [
            (vec![1, 1, 4, 2, 3], 5, 2),
            (vec![5, 6, 7, 8, 9], 4, -1),
            (vec![3, 2, 20, 1, 1, 3], 10, 5),
            (vec![1, 2, 3], 6, 3),
            (vec![1, 2, 3], 0, 0),
            (vec![1, 2, 3], 2, -1),
            (vec![1], 1, 1),
        ] {
            assert_eq!(
                Solution::min_operations_by_prefix_sum(&nums, x),
                expected,
                "{nums:?}, x={x}"
            );
        }
    }
}
