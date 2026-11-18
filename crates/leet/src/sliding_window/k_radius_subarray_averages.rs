/// LeetCode 2090: K Radius Subarray Averages.
pub struct Solution;

impl Solution {
    /// Rolling window: O(n) time and O(1) extra space, excluding the output.
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k == 0 {
            return nums;
        }

        let window = 2 * k + 1;
        let mut averages = vec![-1; nums.len()];
        if window > nums.len() {
            return averages;
        }

        let mut sum: i64 = nums[..window].iter().map(|&num| i64::from(num)).sum();
        averages[k] = (sum / window as i64) as i32;
        for right in window..nums.len() {
            sum += i64::from(nums[right]) - i64::from(nums[right - window]);
            averages[right - k] = (sum / window as i64) as i32;
        }
        averages
    }

    /// Prefix sums: O(n) time and O(n) extra space, excluding the output.
    pub fn get_averages_by_prefix_sum(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k == 0 {
            return nums;
        }

        let window = 2 * k + 1;
        let mut averages = vec![-1; nums.len()];
        if window > nums.len() {
            return averages;
        }

        let mut prefix = vec![0_i64; nums.len() + 1];
        for (i, &num) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + i64::from(num);
        }
        for center in k..nums.len() - k {
            let sum = prefix[center + k + 1] - prefix[center - k];
            averages[center] = (sum / window as i64) as i32;
        }
        averages
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_averages(nums: Vec<i32>, k: i32, expected: Vec<i32>) {
        assert_eq!(Solution::get_averages(nums.clone(), k), expected);
        assert_eq!(Solution::get_averages_by_prefix_sum(nums, k), expected);
    }

    #[test]
    fn official_example() {
        assert_averages(
            vec![7, 4, 3, 9, 1, 8, 5, 2, 6],
            3,
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1],
        );
    }

    #[test]
    fn zero_radius_preserves_every_element() {
        assert_averages(vec![100000], 0, vec![100000]);
        assert_averages(vec![-7, 0, 9], 0, vec![-7, 0, 9]);
    }

    #[test]
    fn oversized_window_has_no_valid_centers() {
        assert_averages(vec![8], 100000, vec![-1]);
        assert_averages(vec![1, 2, 3, 4], 2, vec![-1; 4]);
    }

    #[test]
    fn includes_first_and_last_valid_windows() {
        assert_averages(vec![1, 2, 3, 4, 5], 1, vec![-1, 2, 3, 4, -1]);
    }

    #[test]
    fn exactly_one_complete_window() {
        assert_averages(vec![2, 7, 4], 1, vec![-1, 4, -1]);
    }

    #[test]
    fn negative_averages_truncate_toward_zero() {
        assert_averages(vec![-8, 1, 0, 3, 0, -5], 1, vec![-1, -2, 1, 1, 0, -1]);
    }

    #[test]
    fn window_sum_can_exceed_i32() {
        let mut expected = vec![-1; 100000];
        expected[49999] = 100000;
        expected[50000] = 100000;
        assert_averages(vec![100000; 100000], 49999, expected);
    }
}
