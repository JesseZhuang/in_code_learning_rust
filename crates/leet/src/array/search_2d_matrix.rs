pub struct Solution;

impl Solution {
    /// Treat the matrix as a flattened sorted array and perform a single binary search.
    /// Time: O(log(m*n)), Space: O(1)
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut lo: i32 = 0;
        let mut hi: i32 = (m * n) as i32 - 1;

        // O(log(m*n)) iterations
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let val = matrix[mid as usize / n][mid as usize % n];
            if val == target {
                return true;
            } else if val < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        false
    }

    /// Binary search for the correct row, then binary search within that row.
    /// Time: O(log m + log n), Space: O(1)
    pub fn search_matrix_two_pass(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        // First pass: binary search for the row. O(log m) iterations.
        let mut lo = 0i32;
        let mut hi = m as i32 - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let row = &matrix[mid as usize];
            if target < row[0] {
                hi = mid - 1;
            } else if target > row[n - 1] {
                lo = mid + 1;
            } else {
                // Target is within this row; binary search inside it.
                let mut clo = 0i32;
                let mut chi = n as i32 - 1;
                // Second pass: O(log n) iterations.
                while clo <= chi {
                    let cmid = clo + (chi - clo) / 2;
                    let val = row[cmid as usize];
                    if val == target {
                        return true;
                    } else if val < target {
                        clo = cmid + 1;
                    } else {
                        chi = cmid - 1;
                    }
                }
                return false;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_basic_found() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(Solution::search_matrix(matrix.clone(), 3));
        assert!(Solution::search_matrix_two_pass(matrix, 3));
    }

    #[test]
    fn test_basic_not_found() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(!Solution::search_matrix(matrix.clone(), 13));
        assert!(!Solution::search_matrix_two_pass(matrix, 13));
    }

    #[test]
    fn test_single_element_found() {
        let matrix = vec![vec![5]];
        assert!(Solution::search_matrix(matrix.clone(), 5));
        assert!(Solution::search_matrix_two_pass(matrix, 5));
    }

    #[test]
    fn test_single_element_not_found() {
        let matrix = vec![vec![5]];
        assert!(!Solution::search_matrix(matrix.clone(), 3));
        assert!(!Solution::search_matrix_two_pass(matrix, 3));
    }

    #[test]
    fn test_first_element() {
        let matrix = vec![vec![1, 3, 5], vec![7, 9, 11]];
        assert!(Solution::search_matrix(matrix.clone(), 1));
        assert!(Solution::search_matrix_two_pass(matrix, 1));
    }

    #[test]
    fn test_last_element() {
        let matrix = vec![vec![1, 3, 5], vec![7, 9, 11]];
        assert!(Solution::search_matrix(matrix.clone(), 11));
        assert!(Solution::search_matrix_two_pass(matrix, 11));
    }

    #[test]
    fn test_single_row() {
        let matrix = vec![vec![1, 3, 5, 7, 9]];
        assert!(Solution::search_matrix(matrix.clone(), 7));
        assert!(Solution::search_matrix_two_pass(matrix.clone(), 7));
        assert!(!Solution::search_matrix(matrix.clone(), 4));
        assert!(!Solution::search_matrix_two_pass(matrix, 4));
    }

    #[test]
    fn test_single_column() {
        let matrix = vec![vec![2], vec![5], vec![8], vec![12]];
        assert!(Solution::search_matrix(matrix.clone(), 8));
        assert!(Solution::search_matrix_two_pass(matrix.clone(), 8));
        assert!(!Solution::search_matrix(matrix.clone(), 6));
        assert!(!Solution::search_matrix_two_pass(matrix, 6));
    }

    #[test]
    fn test_target_smaller_than_all() {
        let matrix = vec![vec![2, 4], vec![6, 8]];
        assert!(!Solution::search_matrix(matrix.clone(), 1));
        assert!(!Solution::search_matrix_two_pass(matrix, 1));
    }

    #[test]
    fn test_target_larger_than_all() {
        let matrix = vec![vec![2, 4], vec![6, 8]];
        assert!(!Solution::search_matrix(matrix.clone(), 10));
        assert!(!Solution::search_matrix_two_pass(matrix, 10));
    }
}
