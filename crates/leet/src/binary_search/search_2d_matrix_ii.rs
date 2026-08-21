/// LeetCode 240 - Search a 2D Matrix II
///
/// Write an efficient algorithm that searches for a value target in an m x n integer matrix.
/// This matrix has the following properties:
/// - Integers in each row are sorted in ascending from left to right.
/// - Integers in each column are sorted in ascending from top to bottom.
pub struct Solution;

impl Solution {
    /// Staircase search from top-right corner.
    /// Time: O(m + n), Space: O(1)
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row = 0i32;
        let mut col = n as i32 - 1;

        while row < m as i32 && col >= 0 {
            let val = matrix[row as usize][col as usize];
            if val == target {
                return true;
            } else if val < target {
                row += 1;
            } else {
                col -= 1;
            }
        }
        false
    }

    /// Binary search per row.
    /// Time: O(m log n), Space: O(1)
    pub fn search_matrix_binary(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for row in &matrix {
            if row.is_empty() {
                continue;
            }
            // Skip row if target is out of range
            if row[0] > target || *row.last().unwrap() < target {
                continue;
            }
            if row.binary_search(&target).is_ok() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big_matrix() -> Vec<Vec<i32>> {
        vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ]
    }

    #[test]
    fn test_staircase_found() {
        assert!(Solution::search_matrix(big_matrix(), 5));
    }

    #[test]
    fn test_staircase_not_found() {
        assert!(!Solution::search_matrix(big_matrix(), 20));
    }

    #[test]
    fn test_staircase_single_element_found() {
        assert!(Solution::search_matrix(vec![vec![5]], 5));
    }

    #[test]
    fn test_staircase_single_element_not_found() {
        assert!(!Solution::search_matrix(vec![vec![5]], 3));
    }

    #[test]
    fn test_staircase_negative_values() {
        let matrix = vec![vec![-5, -3], vec![-1, 2]];
        assert!(Solution::search_matrix(matrix, -3));
    }

    #[test]
    fn test_staircase_single_row() {
        let matrix = vec![vec![1, 3, 5, 7, 9]];
        assert!(Solution::search_matrix(matrix.clone(), 5));
        assert!(!Solution::search_matrix(matrix, 4));
    }

    #[test]
    fn test_staircase_single_column() {
        let matrix = vec![vec![1], vec![3], vec![5], vec![7]];
        assert!(Solution::search_matrix(matrix.clone(), 7));
        assert!(!Solution::search_matrix(matrix, 6));
    }

    #[test]
    fn test_binary_found() {
        assert!(Solution::search_matrix_binary(big_matrix(), 5));
    }

    #[test]
    fn test_binary_not_found() {
        assert!(!Solution::search_matrix_binary(big_matrix(), 20));
    }

    #[test]
    fn test_binary_single_element_found() {
        assert!(Solution::search_matrix_binary(vec![vec![5]], 5));
    }

    #[test]
    fn test_binary_single_element_not_found() {
        assert!(!Solution::search_matrix_binary(vec![vec![5]], 3));
    }

    #[test]
    fn test_binary_negative_values() {
        let matrix = vec![vec![-5, -3], vec![-1, 2]];
        assert!(Solution::search_matrix_binary(matrix, -3));
    }

    #[test]
    fn test_binary_single_row() {
        let matrix = vec![vec![1, 3, 5, 7, 9]];
        assert!(Solution::search_matrix_binary(matrix.clone(), 5));
        assert!(!Solution::search_matrix_binary(matrix, 4));
    }

    #[test]
    fn test_binary_single_column() {
        let matrix = vec![vec![1], vec![3], vec![5], vec![7]];
        assert!(Solution::search_matrix_binary(matrix.clone(), 7));
        assert!(!Solution::search_matrix_binary(matrix, 6));
    }
}
