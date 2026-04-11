// lc 215 - Kth Largest Element in an Array

use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    /// Min-heap approach: maintain a min-heap of size k.
    /// Time O(n log k), Space O(k)
    pub fn find_kth_largest_heap(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k + 1);
        for &num in &nums {
            heap.push(Reverse(num));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.peek().unwrap().0
    }

    /// Quickselect approach: partition-based selection.
    /// Time O(n) average, O(n²) worst, Space O(1)
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let target = n - k as usize; // index in sorted order
        Self::quickselect(&mut nums, 0, n - 1, target)
    }

    fn quickselect(nums: &mut [i32], lo: usize, hi: usize, target: usize) -> i32 {
        if lo == hi {
            return nums[lo];
        }
        // Random pivot using simple LCG to avoid rand in hot path
        let pivot_idx = lo + (nums[lo] as usize ^ nums[hi] as usize ^ lo ^ hi) % (hi - lo + 1);
        nums.swap(pivot_idx, hi);
        let pivot = nums[hi];

        let mut store = lo;
        for i in lo..hi {
            if nums[i] < pivot {
                nums.swap(i, store);
                store += 1;
            }
        }
        nums.swap(store, hi);

        if store == target {
            nums[store]
        } else if target < store {
            Self::quickselect(nums, lo, store.saturating_sub(1), target)
        } else {
            Self::quickselect(nums, store + 1, hi, target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cases() -> Vec<(Vec<i32>, i32, i32)> {
        vec![
            (vec![3, 2, 1, 5, 6, 4], 2, 5),
            (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4),
            (vec![1], 1, 1),
            (vec![7, 7, 7, 7], 2, 7),
            (vec![1, 2, 3, 4, 5], 1, 5),
            (vec![5, 4, 3, 2, 1], 5, 1),
            (vec![-1, -2, -3, -4], 2, -2),
            (vec![-1, 2, 0], 1, 2),
            (vec![3, 1, 2], 3, 1),
        ]
    }

    #[test]
    fn test_heap() {
        for (nums, k, expected) in cases() {
            assert_eq!(Solution::find_kth_largest_heap(nums, k), expected);
        }
    }

    #[test]
    fn test_quickselect() {
        for (nums, k, expected) in cases() {
            assert_eq!(Solution::find_kth_largest(nums, k), expected);
        }
    }
}
