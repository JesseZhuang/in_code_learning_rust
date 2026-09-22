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

pub struct Solution2;

impl Solution2 {
    /// Multi-source BFS approach. O(m*n) time, O(m*n) space.
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let m = mat.len();
        let n = mat[0].len();
        let mut queue = VecDeque::new();
        for r in 0..m {
            for c in 0..n {
                if mat[r][c] == 0 {
                    queue.push_back((r, c));
                } else {
                    mat[r][c] = -1;
                }
            }
        }
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if mat[nr][nc] == -1 {
                        mat[nr][nc] = mat[r][c] + 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
        mat
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, Solution2};

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

    #[test]
    fn test_bfs_example1() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution2::update_matrix(mat), expected);
    }

    #[test]
    fn test_bfs_example2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
        assert_eq!(Solution2::update_matrix(mat), expected);
    }

    #[test]
    fn test_bfs_all_zeros() {
        let mat = vec![vec![0, 0], vec![0, 0]];
        let expected = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution2::update_matrix(mat), expected);
    }

    #[test]
    fn test_bfs_corner_ones() {
        let mat = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        let expected = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(Solution2::update_matrix(mat), expected);
    }

    #[test]
    fn test_bfs_single_row() {
        let mat = vec![vec![1, 1, 0, 1, 1]];
        let expected = vec![vec![2, 1, 0, 1, 2]];
        assert_eq!(Solution2::update_matrix(mat), expected);
    }

    #[test]
    fn test_bfs_large_distance() {
        let mat = vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 0],
        ];
        let expected = vec![
            vec![6, 5, 4, 3],
            vec![5, 4, 3, 2],
            vec![4, 3, 2, 1],
            vec![3, 2, 1, 0],
        ];
        assert_eq!(Solution2::update_matrix(mat), expected);
    }
}
