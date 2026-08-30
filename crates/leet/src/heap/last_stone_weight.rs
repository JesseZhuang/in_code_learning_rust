use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    /// BinaryHeap (max-heap) approach.
    /// Time: O(n log n), Space: O(n)
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones); // O(n) heapify
        while heap.len() > 1 {
            let a = heap.pop().unwrap(); // heaviest
            let b = heap.pop().unwrap(); // second heaviest
            if a != b {
                heap.push(a - b); // O(log n) insert
            }
        }
        heap.pop().unwrap_or(0)
    }

    /// Sorted Vec approach: sort, pop two largest, insort remainder.
    /// Time: O(n^2), Space: O(n)
    pub fn last_stone_weight_sorted(mut stones: Vec<i32>) -> i32 {
        stones.sort_unstable(); // O(n log n) initial sort
        while stones.len() > 1 {
            let a = stones.pop().unwrap(); // largest
            let b = stones.pop().unwrap(); // second largest
            let diff = a - b;
            if diff > 0 {
                // Binary search insert to maintain sorted order — O(n) shift
                let pos = stones.partition_point(|&x| x < diff);
                stones.insert(pos, diff);
            }
        }
        stones.pop().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cases() -> Vec<(Vec<i32>, i32)> {
        vec![
            (vec![2, 7, 4, 1, 8, 1], 1),
            (vec![1], 1),
            (vec![3, 3], 0),
            (vec![3, 7], 4),
            (vec![5, 5, 5, 5], 0),
            (vec![5, 5, 5], 5),
            (vec![10, 4, 2, 10], 2),
            (vec![42], 42),
            (vec![1000, 999], 1),
            (vec![100, 1, 1, 1, 1], 96),
        ]
    }

    #[test]
    fn test_heap() {
        for (stones, expected) in cases() {
            assert_eq!(Solution::last_stone_weight(stones), expected);
        }
    }

    #[test]
    fn test_sorted() {
        for (stones, expected) in cases() {
            assert_eq!(Solution::last_stone_weight_sorted(stones), expected);
        }
    }
}
