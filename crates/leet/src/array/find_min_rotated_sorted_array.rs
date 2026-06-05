/// LeetCode 153 - Find Minimum in Rotated Sorted Array
///
/// Time: O(log n), Space: O(1)

pub struct Solution;

impl Solution {
    /// Binary search comparing mid with hi.
    /// If nums[mid] > nums[hi], minimum is in the right half.
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0usize, nums.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] > nums[hi] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        nums[lo]
    }

    /// Alternative: compare mid with nums[0].
    /// If array is not rotated (nums[0] <= nums[n-1]), return nums[0] early.
    /// Otherwise, if nums[mid] >= nums[0], mid is in the left sorted portion,
    /// so minimum is to the right.
    pub fn find_min_v2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 || nums[0] <= nums[n - 1] {
            return nums[0];
        }
        let (mut lo, mut hi) = (0usize, n - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] >= nums[0] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        nums[lo]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_min() {
        let cases: Vec<(Vec<i32>, i32)> = vec![
            (vec![3, 4, 5, 1, 2], 1),
            (vec![4, 5, 6, 7, 0, 1, 2], 0),
            (vec![11, 13, 15, 17], 11),
            (vec![1], 1),
            (vec![2, 1], 1),
            (vec![1, 2], 1),
            (vec![7, 1, 2, 3, 4, 5, 6], 1),
            (vec![1, 2, 3, -5, -4, -3, -2, -1, 0], -5),
        ];
        for (nums, expected) in cases.clone() {
            assert_eq!(Solution::find_min(nums), expected);
        }
        for (nums, expected) in cases {
            assert_eq!(Solution::find_min_v2(nums), expected);
        }
    }
}
