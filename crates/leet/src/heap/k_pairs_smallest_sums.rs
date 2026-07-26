use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub struct Solution;

impl Solution {
    /// Solution 1: Min-heap approach.
    /// Push (sum, i, j) for the first min(k, nums1.len()) rows with j=0,
    /// then pop the smallest and advance the column index.
    /// Time: O(k log k), Space: O(k)
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        if nums1.is_empty() || nums2.is_empty() {
            return vec![];
        }

        let mut heap: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new(); // O(k) space

        // Initialize heap with first column of each row (up to k rows)
        for i in 0..nums1.len().min(k) {
            heap.push(Reverse((nums1[i] as i64 + nums2[0] as i64, i, 0)));
        }

        let mut result = Vec::with_capacity(k);

        while let Some(Reverse((_, i, j))) = heap.pop() {
            result.push(vec![nums1[i], nums2[j]]);
            if result.len() == k {
                break;
            }
            // Advance column: push (nums1[i], nums2[j+1]) if within bounds
            if j + 1 < nums2.len() {
                heap.push(Reverse((nums1[i] as i64 + nums2[j + 1] as i64, i, j + 1)));
            }
        }

        result
    }

    /// Solution 2: BFS-like expansion with HashSet for visited.
    /// Start from (0,0), expand right (i, j+1) and down (i+1, j).
    /// Time: O(k log k), Space: O(k)
    pub fn k_smallest_pairs_bfs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        if nums1.is_empty() || nums2.is_empty() {
            return vec![];
        }

        let mut heap: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new(); // O(k) space

        heap.push(Reverse((nums1[0] as i64 + nums2[0] as i64, 0, 0)));
        visited.insert((0, 0));

        let mut result = Vec::with_capacity(k);

        while let Some(Reverse((_, i, j))) = heap.pop() {
            result.push(vec![nums1[i], nums2[j]]);
            if result.len() == k {
                break;
            }
            // Expand right: (i, j+1)
            if j + 1 < nums2.len() && visited.insert((i, j + 1)) {
                heap.push(Reverse((nums1[i] as i64 + nums2[j + 1] as i64, i, j + 1)));
            }
            // Expand down: (i+1, j)
            if i + 1 < nums1.len() && visited.insert((i + 1, j)) {
                heap.push(Reverse((nums1[i + 1] as i64 + nums2[j] as i64, i + 1, j)));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_both(nums1: Vec<i32>, nums2: Vec<i32>, k: i32, expected: Vec<Vec<i32>>) {
        let r1 = Solution::k_smallest_pairs(nums1.clone(), nums2.clone(), k);
        let r2 = Solution::k_smallest_pairs_bfs(nums1, nums2, k);
        assert_eq!(r1, expected, "min-heap solution mismatch");
        assert_eq!(r2, expected, "bfs solution mismatch");
    }

    #[test]
    fn test_example1() {
        run_both(
            vec![1, 7, 11],
            vec![2, 4, 6],
            3,
            vec![vec![1, 2], vec![1, 4], vec![1, 6]],
        );
    }

    #[test]
    fn test_example2() {
        run_both(
            vec![1, 1, 2],
            vec![1, 2, 3],
            2,
            vec![vec![1, 1], vec![1, 1]],
        );
    }

    #[test]
    fn test_example3() {
        run_both(vec![1, 2], vec![3], 3, vec![vec![1, 3], vec![2, 3]]);
    }

    #[test]
    fn test_k_larger_than_total() {
        // Total pairs = 2*2 = 4, k = 10
        run_both(
            vec![1, 2],
            vec![3, 4],
            10,
            vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]],
        );
    }

    #[test]
    fn test_single_element() {
        run_both(vec![5], vec![7], 1, vec![vec![5, 7]]);
    }

    #[test]
    fn test_empty_arrays() {
        run_both(vec![], vec![1, 2], 3, vec![]);
        run_both(vec![1, 2], vec![], 3, vec![]);
    }

    #[test]
    fn test_negative_numbers() {
        run_both(
            vec![-5, -3, 0],
            vec![-2, 0, 4],
            4,
            vec![vec![-5, -2], vec![-5, 0], vec![-3, -2], vec![-3, 0]],
        );
    }
}
