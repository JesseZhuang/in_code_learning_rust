/// LeetCode 33 - Search in Rotated Sorted Array
/// Single-pass modified binary search
/// Time: O(log n) — standard binary search with rotation handling
/// Space: O(1)
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            let mi = m as usize;
            if nums[mi] == target {
                return m;
            }
            if nums[l as usize] <= nums[mi] {
                // left half is sorted
                if nums[l as usize] <= target && target < nums[mi] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                // right half is sorted
                if nums[mi] < target && target <= nums[r as usize] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }
        -1
    }
}

/// Two-pass approach: find pivot, then binary search the correct half
/// Time: O(log n) — two binary searches
/// Space: O(1)
struct Solution2;

impl Solution2 {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len() as i32;
        if n == 0 {
            return -1;
        }

        // find pivot (index of smallest element)
        let mut l: i32 = 0;
        let mut r: i32 = n - 1;
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m as usize] > nums[r as usize] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        let pivot = l;

        // determine which half to search
        if target >= nums[pivot as usize] && target <= nums[(n - 1) as usize] {
            l = pivot;
            r = n - 1;
        } else {
            l = 0;
            r = pivot - 1;
        }

        // standard binary search
        while l <= r {
            let m = l + (r - l) / 2;
            let val = nums[m as usize];
            if val == target {
                return m;
            } else if val < target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to run a test case against both solutions.
    fn check(nums: Vec<i32>, target: i32, expected: i32) {
        assert_eq!(Solution::search(nums.clone(), target), expected, "Solution failed");
        assert_eq!(Solution2::search(nums, target), expected, "Solution2 failed");
    }

    #[test]
    fn example_found() {
        check(vec![4, 5, 6, 7, 0, 1, 2], 0, 4);
    }

    #[test]
    fn example_not_found() {
        check(vec![4, 5, 6, 7, 0, 1, 2], 3, -1);
    }

    #[test]
    fn single_not_found() {
        check(vec![1], 0, -1);
    }

    #[test]
    fn single_found() {
        check(vec![1], 1, 0);
    }

    #[test]
    fn two_elements_find_second() {
        check(vec![3, 1], 1, 1);
    }

    #[test]
    fn two_elements_find_first() {
        check(vec![3, 1], 3, 0);
    }

    #[test]
    fn three_elements_find_first() {
        check(vec![5, 1, 3], 5, 0);
    }

    #[test]
    fn rotated_find_last() {
        check(vec![2, 3, 4, 5, 1], 1, 4);
    }

    #[test]
    fn not_rotated() {
        check(vec![1, 2, 3, 4, 5], 3, 2);
    }

    #[test]
    fn larger_array() {
        check(vec![4, 5, 6, 7, 8, 1, 2, 3], 8, 4);
    }
}
