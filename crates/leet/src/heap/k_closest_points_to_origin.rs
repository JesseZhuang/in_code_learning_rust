// lc 973 - K Closest Points to Origin

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    /// Max-heap of size k approach.
    /// Time O(n log k), Space O(k)
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        // Max-heap keyed by squared distance
        let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        for (i, p) in points.iter().enumerate() {
            let dist = (p[0] as i64) * (p[0] as i64) + (p[1] as i64) * (p[1] as i64);
            heap.push((dist, i));
            if heap.len() > k {
                heap.pop(); // evict farthest point
            }
        }
        heap.into_iter().map(|(_, i)| points[i].clone()).collect()
    }
}

pub struct Solution2;

impl Solution2 {
    /// Quickselect approach.
    /// Time O(n) average, Space O(1)
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut points = points;
        let n = points.len();
        Self::quickselect(&mut points, 0, n - 1, k);
        points.truncate(k);
        points
    }

    fn dist(p: &[i32]) -> i64 {
        (p[0] as i64) * (p[0] as i64) + (p[1] as i64) * (p[1] as i64)
    }

    fn quickselect(points: &mut Vec<Vec<i32>>, lo: usize, hi: usize, k: usize) {
        if lo >= hi {
            return;
        }
        let pivot_idx = Self::partition(points, lo, hi);
        let count = pivot_idx - lo + 1; // elements in left partition (including pivot)
        if k < count {
            Self::quickselect(points, lo, pivot_idx.saturating_sub(1), k);
        } else if k > count {
            Self::quickselect(points, pivot_idx + 1, hi, k);
        }
        // if k == count, we're done
    }

    fn partition(points: &mut Vec<Vec<i32>>, lo: usize, hi: usize) -> usize {
        let pivot_dist = Self::dist(&points[hi]);
        let mut store = lo;
        for i in lo..hi {
            if Self::dist(&points[i]) <= pivot_dist {
                points.swap(store, i);
                store += 1;
            }
        }
        points.swap(store, hi);
        store
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort();
        v
    }

    #[test]
    fn test_heap() {
        assert_eq!(
            sorted(Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1)),
            vec![vec![-2, 2]]
        );
        assert_eq!(
            sorted(Solution::k_closest(
                vec![vec![3, 3], vec![5, -1], vec![-2, 4]],
                2
            )),
            sorted(vec![vec![3, 3], vec![-2, 4]])
        );
        assert_eq!(
            sorted(Solution::k_closest(vec![vec![0, 1], vec![1, 0]], 2)),
            sorted(vec![vec![0, 1], vec![1, 0]])
        );
    }

    #[test]
    fn test_quickselect() {
        assert_eq!(
            sorted(Solution2::k_closest(vec![vec![1, 3], vec![-2, 2]], 1)),
            vec![vec![-2, 2]]
        );
        assert_eq!(
            sorted(Solution2::k_closest(
                vec![vec![3, 3], vec![5, -1], vec![-2, 4]],
                2
            )),
            sorted(vec![vec![3, 3], vec![-2, 4]])
        );
        assert_eq!(
            sorted(Solution2::k_closest(vec![vec![0, 1], vec![1, 0]], 2)),
            sorted(vec![vec![0, 1], vec![1, 0]])
        );
    }
}
