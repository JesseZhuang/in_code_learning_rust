/// leet 542

pub struct Solution;

impl Solution {
    /// DP approach. O(m*n) time, O(1) space (in-place).
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let inf = (m + n) as i32;
        // top-left to bottom-right
        for r in 0..m {
            for c in 0..n {
                if mat[r][c] > 0 {
                    let top = if r > 0 { mat[r - 1][c] } else { inf };
                    let left = if c > 0 { mat[r][c - 1] } else { inf };
                    mat[r][c] = top.min(left) + 1;
                }
            }
        }
        // bottom-right to top-left
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if mat[r][c] > 0 {
                    let bottom = if r < m - 1 { mat[r + 1][c] } else { inf };
                    let right = if c < n - 1 { mat[r][c + 1] } else { inf };
                    mat[r][c] = mat[r][c].min(bottom + 1).min(right + 1);
                }
            }
        }
        mat
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_update_matrix_example1() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::update_matrix(mat), expected);
    }

    #[test]
    fn test_update_matrix_example2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
        assert_eq!(Solution::update_matrix(mat), expected);
    }
}
