/// leet 1438
/// Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit

use std::collections::{BTreeMap, VecDeque};

/// O(n) time, O(n) space — monotonic deques
pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_dq: VecDeque<usize> = VecDeque::new(); // decreasing deque (front = max)
        let mut min_dq: VecDeque<usize> = VecDeque::new(); // increasing deque (front = min)
        let (mut l, mut res) = (0, 0);

        for r in 0..nums.len() {
            // Maintain decreasing deque for max element
            while let Some(&back) = max_dq.back() {
                if nums[back] <= nums[r] {
                    max_dq.pop_back();
                } else {
                    break;
                }
            }
            max_dq.push_back(r);

            // Maintain increasing deque for min element
            while let Some(&back) = min_dq.back() {
                if nums[back] >= nums[r] {
                    min_dq.pop_back();
                } else {
                    break;
                }
            }
            min_dq.push_back(r);

            // Shrink window until constraint satisfied
            while nums[*max_dq.front().unwrap()] - nums[*min_dq.front().unwrap()] > limit {
                l += 1;
                if *max_dq.front().unwrap() < l {
                    max_dq.pop_front(); // O(1) amortized removal
                }
                if *min_dq.front().unwrap() < l {
                    min_dq.pop_front();
                }
            }

            res = res.max(r - l + 1);
        }

        res as i32
    }
}

/// O(n log n) time, O(n) space — BTreeMap sorted counts
pub struct Solution2;

impl Solution2 {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new(); // element -> count
        let (mut l, mut res) = (0, 0);

        for r in 0..nums.len() {
            *map.entry(nums[r]).or_insert(0) += 1; // O(log n) insert

            // Shrink window: max - min > limit
            while *map.keys().next_back().unwrap() - *map.keys().next().unwrap() > limit {
                let e = map.entry(nums[l]).or_insert(0);
                *e -= 1;
                if *e == 0 {
                    map.remove(&nums[l]); // O(log n) removal
                }
                l += 1;
            }

            res = res.max(r - l + 1);
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cases() -> Vec<(Vec<i32>, i32, i32)> {
        vec![
            (vec![8, 2, 4, 7], 4, 2),
            (vec![10, 1, 2, 4, 7, 2], 5, 4),
            (vec![4, 2, 2, 2, 4, 4, 2, 2], 0, 3),
            (vec![5], 0, 1),
            (vec![3, 3, 3, 3, 3], 0, 5),
            (vec![1, 2, 1, 2, 1], 0, 1),
            (vec![1, 5, 9, 2, 7], 100, 5),
            (vec![9, 8, 7, 6, 5], 2, 3),
            (vec![1, 2, 3, 4, 5], 2, 3),
            (vec![1, 3], 2, 2),
            (vec![1, 4], 2, 1),
        ]
    }

    #[test]
    fn test_deque_solution() {
        for (nums, limit, expected) in cases() {
            assert_eq!(
                Solution::longest_subarray(nums.clone(), limit),
                expected,
                "deque: nums={:?}, limit={}",
                nums,
                limit
            );
        }
    }

    #[test]
    fn test_btreemap_solution() {
        for (nums, limit, expected) in cases() {
            assert_eq!(
                Solution2::longest_subarray(nums.clone(), limit),
                expected,
                "btreemap: nums={:?}, limit={}",
                nums,
                limit
            );
        }
    }
}
