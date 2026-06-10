pub struct Solution;

impl Solution {
    // O(log n) time, O(1) space. Two binary searches.
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let left = Self::find_left_boundary(&nums, target);
        if left == -1 {
            return vec![-1, -1];
        }

        let right = Self::find_right_boundary(&nums, target);
        vec![left, right]
    }

    fn find_left_boundary(nums: &[i32], target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // Check if target exists at the found position
        if left < nums.len() && nums[left] == target {
            left as i32
        } else {
            -1
        }
    }

    fn find_right_boundary(nums: &[i32], target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // left-1 is the rightmost position where nums[i] <= target
        (left - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(vec![3, 4], Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8));
    }

    #[test]
    fn test_example2() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6));
    }

    #[test]
    fn test_empty() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 0));
    }

    #[test]
    fn test_single_element_found() {
        assert_eq!(vec![0, 0], Solution::search_range(vec![1], 1));
    }

    #[test]
    fn test_single_element_not_found() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![1], 2));
    }

    #[test]
    fn test_all_same() {
        assert_eq!(vec![0, 4], Solution::search_range(vec![2, 2, 2, 2, 2], 2));
    }

    #[test]
    fn test_at_start() {
        assert_eq!(vec![0, 1], Solution::search_range(vec![1, 1, 2, 3, 4], 1));
    }

    #[test]
    fn test_at_end() {
        assert_eq!(vec![3, 4], Solution::search_range(vec![1, 2, 3, 5, 5], 5));
    }

    #[test]
    fn test_single_occurrence() {
        assert_eq!(vec![2, 2], Solution::search_range(vec![1, 2, 3, 4, 5], 3));
    }
}
