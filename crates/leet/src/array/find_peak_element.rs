pub struct Solution;

impl Solution {
    /// Binary search approach: O(log n) time, O(1) space.
    /// At each step, move toward the side with a larger neighbor,
    /// guaranteeing convergence to a peak.
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut lo: usize = 0;
        let mut hi: usize = nums.len() - 1;
        // O(log n) iterations, O(1) space
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] < nums[mid + 1] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }

    /// Linear scan approach: O(n) time, O(1) space.
    /// Walk until we find a drop; the element before the drop is a peak.
    #[allow(dead_code)]
    pub fn find_peak_element_linear(nums: Vec<i32>) -> i32 {
        // O(n) iterations, O(1) space
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                return i as i32;
            }
        }
        (nums.len() - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Validates that the returned index is actually a peak.
    fn is_peak(nums: &[i32], idx: i32) -> bool {
        let i = idx as usize;
        if i >= nums.len() {
            return false;
        }
        let left_ok = i == 0 || nums[i] > nums[i - 1];
        let right_ok = i == nums.len() - 1 || nums[i] > nums[i + 1];
        left_ok && right_ok
    }

    #[test]
    fn test_single_element() {
        let nums = vec![1];
        assert!(is_peak(&nums, Solution::find_peak_element(nums.clone())));
        assert!(is_peak(&nums, Solution::find_peak_element_linear(nums.clone())));
    }

    #[test]
    fn test_two_elements_ascending() {
        let nums = vec![1, 2];
        assert!(is_peak(&nums, Solution::find_peak_element(nums.clone())));
        assert!(is_peak(&nums, Solution::find_peak_element_linear(nums.clone())));
    }

    #[test]
    fn test_two_elements_descending() {
        let nums = vec![2, 1];
        assert!(is_peak(&nums, Solution::find_peak_element(nums.clone())));
        assert!(is_peak(&nums, Solution::find_peak_element_linear(nums.clone())));
    }

    #[test]
    fn test_example_1() {
        // [1,2,3,1] -> peak at index 2
        let nums = vec![1, 2, 3, 1];
        assert!(is_peak(&nums, Solution::find_peak_element(nums.clone())));
        assert!(is_peak(&nums, Solution::find_peak_element_linear(nums.clone())));
    }

    #[test]
    fn test_example_2() {
        // [1,2,1,3,5,6,4] -> peak at index 1 or 5
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        assert!(is_peak(&nums, Solution::find_peak_element(nums.clone())));
        assert!(is_peak(&nums, Solution::find_peak_element_linear(nums.clone())));
    }

    #[test]
    fn test_ascending() {
        // Strictly ascending -> peak must be last element
        let nums = vec![1, 2, 3, 4, 5];
        assert!(is_peak(&nums, Solution::find_peak_element(nums.clone())));
        assert!(is_peak(&nums, Solution::find_peak_element_linear(nums.clone())));
    }

    #[test]
    fn test_descending() {
        // Strictly descending -> peak must be first element
        let nums = vec![5, 4, 3, 2, 1];
        assert!(is_peak(&nums, Solution::find_peak_element(nums.clone())));
        assert!(is_peak(&nums, Solution::find_peak_element_linear(nums.clone())));
    }

    #[test]
    fn test_multiple_peaks() {
        // [1,3,1,4,1] -> peaks at index 1 and 3
        let nums = vec![1, 3, 1, 4, 1];
        assert!(is_peak(&nums, Solution::find_peak_element(nums.clone())));
        assert!(is_peak(&nums, Solution::find_peak_element_linear(nums.clone())));
    }
}
