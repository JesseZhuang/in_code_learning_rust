/// LeetCode 540

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1); // constraints len>=1
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] != nums[m ^ 1] { r = m } else { l = m + 1 }
        }
        nums[l]
    }

    fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
        let mut l = 0;
        let mut r = nums.len().checked_sub(1)?; // return None if empty slice
        while l <= r {
            let mid = l + (r - l) / 2;
            // overflow means we can return None
            if nums[mid] == target {
                Some(mid);
            } else if target > nums[mid] {
                l = mid.checked_add(1)?;
            } else { r = mid.checked_sub(1)?; }
        }
        None
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::array::single_element_sorted_array::Solution;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]));
    }
}