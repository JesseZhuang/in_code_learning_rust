/// LeetCode 287 - Find the Duplicate Number
///
/// Given an array of integers nums containing n + 1 integers where each integer
/// is in the range [1, n] inclusive, find the duplicate number.
pub struct Solution;

impl Solution {
    /// Floyd's cycle detection (tortoise and hare).
    /// Time: O(n), Space: O(1)
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Phase 1: find intersection point in cycle
        let mut slow = nums[0] as usize; // O(1) space — only two pointers
        let mut fast = nums[0] as usize;
        loop {
            slow = nums[slow] as usize; // tortoise moves one step
            fast = nums[nums[fast] as usize] as usize; // hare moves two steps
            if slow == fast {
                break; // cycle detected — O(n) iterations worst case
            }
        }

        // Phase 2: find entrance to cycle (the duplicate value)
        let mut slow2 = nums[0] as usize;
        while slow2 != slow {
            slow2 = nums[slow2] as usize; // both move one step — O(n) iterations
            slow = nums[slow] as usize;
        }
        slow as i32
    }

    /// Binary search on value range [1, n].
    /// Time: O(n log n), Space: O(1)
    pub fn find_duplicate_binary_search(nums: Vec<i32>) -> i32 {
        let mut lo = 1i32;
        let mut hi = nums.len() as i32 - 1; // n = nums.len() - 1

        while lo < hi {
            // O(log n) iterations of binary search
            let mid = lo + (hi - lo) / 2;
            // Count elements <= mid — O(n) per iteration
            let count = nums.iter().filter(|&&x| x <= mid).count() as i32;
            if count > mid {
                // By pigeonhole, duplicate is in [lo, mid]
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo // Total: O(n log n) time, O(1) space
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate_floyd() {
        let cases: Vec<(Vec<i32>, i32)> = vec![
            (vec![1, 3, 4, 2, 2], 2),
            (vec![3, 1, 3, 4, 2], 3),
            (vec![1, 1, 1, 1, 1], 1),
            (vec![1, 1], 1),
            (vec![2, 2, 2, 2, 2], 2),
            (vec![1, 4, 4, 2, 3], 4),
            (vec![1, 2, 3, 4, 4], 4),
            (vec![5, 1, 2, 3, 4, 5], 5),
        ];
        for (nums, expected) in cases {
            assert_eq!(Solution::find_duplicate(nums), expected);
        }
    }

    #[test]
    fn test_find_duplicate_binary_search() {
        let cases: Vec<(Vec<i32>, i32)> = vec![
            (vec![1, 3, 4, 2, 2], 2),
            (vec![3, 1, 3, 4, 2], 3),
            (vec![1, 1, 1, 1, 1], 1),
            (vec![1, 1], 1),
            (vec![2, 2, 2, 2, 2], 2),
            (vec![1, 4, 4, 2, 3], 4),
            (vec![1, 2, 3, 4, 4], 4),
            (vec![5, 1, 2, 3, 4, 5], 5),
        ];
        for (nums, expected) in cases {
            assert_eq!(Solution::find_duplicate_binary_search(nums), expected);
        }
    }
}
