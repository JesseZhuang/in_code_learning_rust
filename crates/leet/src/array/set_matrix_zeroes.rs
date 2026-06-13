use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// O(mn) time, O(1) space. Use first row/col as markers.
    ///
    /// Strategy:
    /// 1. Use first row and first column as markers to track which rows/cols need zeroing
    /// 2. Use a separate flag for col0 since matrix[0][0] is ambiguous
    /// 3. Scan forward to mark where zeros exist
    /// 4. Scan backward to set zeros (backward avoids corrupting markers)
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix[0].is_empty() {
            return;
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut col0_has_zero = false;

        // Phase 1: Use first row and col as markers
        // col0_has_zero tracks if column 0 needs to be zeroed
        for i in 0..m {
            if matrix[i][0] == 0 {
                col0_has_zero = true;
            }
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0; // mark row i
                    matrix[0][j] = 0; // mark col j
                }
            }
        }

        // Phase 2: Set zeros based on markers (scan backward to avoid corrupting markers)
        for i in (0..m).rev() {
            for j in (1..n).rev() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
            if col0_has_zero {
                matrix[i][0] = 0;
            }
        }
    }

    /// O(mn) time, O(m+n) space. Use HashSets.
    ///
    /// Strategy:
    /// 1. First pass: record all rows and columns that contain zeros
    /// 2. Second pass: set cells to zero if their row or column was marked
    pub fn set_zeroes_v2(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix[0].is_empty() {
            return;
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut zero_rows = HashSet::new();
        let mut zero_cols = HashSet::new();

        // Phase 1: Record which rows and cols have zeros
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    zero_rows.insert(i);
                    zero_cols.insert(j);
                }
            }
        }

        // Phase 2: Set zeros
        for i in 0..m {
            for j in 0..n {
                if zero_rows.contains(&i) || zero_cols.contains(&j) {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn example1_v2() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes_v2(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn example2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }

    #[test]
    fn example2_v2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes_v2(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }

    #[test]
    fn no_zeroes() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        let expected = matrix.clone();
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn no_zeroes_v2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        let expected = matrix.clone();
        Solution::set_zeroes_v2(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn single_zero() {
        let mut matrix = vec![vec![0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0]]);
    }

    #[test]
    fn single_zero_v2() {
        let mut matrix = vec![vec![0]];
        Solution::set_zeroes_v2(&mut matrix);
        assert_eq!(matrix, vec![vec![0]]);
    }

    #[test]
    fn first_row_has_zero() {
        let mut matrix = vec![vec![0, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0, 0], vec![0, 5, 6], vec![0, 8, 9]]);
    }

    #[test]
    fn first_row_has_zero_v2() {
        let mut matrix = vec![vec![0, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::set_zeroes_v2(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0, 0], vec![0, 5, 6], vec![0, 8, 9]]);
    }

    #[test]
    fn first_col_has_zero() {
        let mut matrix = vec![vec![1, 2, 3], vec![0, 5, 6], vec![7, 8, 9]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 2, 3], vec![0, 0, 0], vec![0, 8, 9]]);
    }

    #[test]
    fn first_col_has_zero_v2() {
        let mut matrix = vec![vec![1, 2, 3], vec![0, 5, 6], vec![7, 8, 9]];
        Solution::set_zeroes_v2(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 2, 3], vec![0, 0, 0], vec![0, 8, 9]]);
    }

    #[test]
    fn negative_values() {
        let mut matrix = vec![vec![-1, 0, -3], vec![-4, -5, -6], vec![-7, -8, -9]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0], vec![-4, 0, -6], vec![-7, 0, -9]]
        );
    }

    #[test]
    fn negative_values_v2() {
        let mut matrix = vec![vec![-1, 0, -3], vec![-4, -5, -6], vec![-7, -8, -9]];
        Solution::set_zeroes_v2(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0], vec![-4, 0, -6], vec![-7, 0, -9]]
        );
    }

    #[test]
    fn all_zeros() {
        let mut matrix = vec![vec![0, 0], vec![0, 0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0], vec![0, 0]]);
    }

    #[test]
    fn all_zeros_v2() {
        let mut matrix = vec![vec![0, 0], vec![0, 0]];
        Solution::set_zeroes_v2(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0], vec![0, 0]]);
    }

    #[test]
    fn corner_cases() {
        // Zero at [0][0]
        let mut matrix = vec![vec![0, 2], vec![3, 4]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0], vec![0, 4]]);
    }

    #[test]
    fn corner_cases_v2() {
        // Zero at [0][0]
        let mut matrix = vec![vec![0, 2], vec![3, 4]];
        Solution::set_zeroes_v2(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0], vec![0, 4]]);
    }
}
