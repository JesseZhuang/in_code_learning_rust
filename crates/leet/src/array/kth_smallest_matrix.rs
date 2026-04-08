// lc 378 / lintcode 1272

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (mut left, mut right) = (matrix[0][0], matrix[matrix.len() - 1][matrix[0].len() - 1]);
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::count_less_equal(&matrix, mid) >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    pub fn kth_smallest_heap(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = matrix.len();
        let mut heap = BinaryHeap::new();
        for (row, values) in matrix.iter().enumerate().take(rows.min(k as usize)) {
            heap.push((Reverse(values[0]), row, 0usize));
        }

        let mut value = matrix[0][0];
        for _ in 0..k {
            let (Reverse(cur), row, col) = heap.pop().unwrap();
            value = cur;
            if col + 1 < matrix[row].len() {
                heap.push((Reverse(matrix[row][col + 1]), row, col + 1));
            }
        }
        value
    }

    fn count_less_equal(matrix: &[Vec<i32>], target: i32) -> i32 {
        let mut row = matrix.len() as i32 - 1;
        let mut col = 0usize;
        let mut count = 0i32;
        while row >= 0 && col < matrix[0].len() {
            if matrix[row as usize][col] <= target {
                count += row + 1;
                col += 1;
            } else {
                row -= 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn binary_search_solution() {
        assert_eq!(
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
            13
        );
        assert_eq!(Solution::kth_smallest(vec![vec![-5]], 1), -5);
        assert_eq!(Solution::kth_smallest(vec![vec![1, 2], vec![1, 3]], 2), 1);
        assert_eq!(
            Solution::kth_smallest(vec![vec![-10, -5, 0], vec![-8, -3, 2], vec![-6, 1, 4]], 5),
            -3
        );
    }

    #[test]
    fn heap_solution() {
        assert_eq!(
            Solution::kth_smallest_heap(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
            13
        );
        assert_eq!(Solution::kth_smallest_heap(vec![vec![-5]], 1), -5);
        assert_eq!(
            Solution::kth_smallest_heap(vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]], 9),
            5
        );
        assert_eq!(
            Solution::kth_smallest_heap(vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]], 1),
            1
        );
    }
}
