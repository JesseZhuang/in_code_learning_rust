/// leet 64

impl Solution {
    /// In-place DP modifying grid. O(m*n) time, O(1) extra space.
    pub fn min_path_sum(grid: &mut Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }
                let top = if i > 0 { grid[i - 1][j] } else { i32::MAX };
                let left = if j > 0 { grid[i][j - 1] } else { i32::MAX };
                grid[i][j] += top.min(left);
            }
        }
        grid[m - 1][n - 1]
    }

    /// 1D DP without modifying input. O(m*n) time, O(n) space.
    pub fn min_path_sum_1d(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![i32::MAX; n];
        dp[0] = 0;
        for i in 0..m {
            dp[0] += grid[i][0];
            for j in 1..n {
                dp[j] = dp[j].min(dp[j - 1]) + grid[i][j];
            }
        }
        dp[n - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_in_place() {
        let cases: Vec<(Vec<Vec<i32>>, i32)> = vec![
            (vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]], 7),
            (vec![vec![5]], 5),
            (vec![vec![1, 2, 3]], 6),
            (vec![vec![1], vec![2], vec![3]], 6),
            (vec![vec![1, 2], vec![1, 1]], 3),
            (vec![vec![0, 0], vec![0, 0]], 0),
            (vec![vec![100, 100, 100], vec![100, 1, 100], vec![100, 1, 1]], 203),
            (vec![vec![1, 100], vec![1, 1]], 3),
        ];
        for (mut grid, expected) in cases {
            assert_eq!(Solution::min_path_sum(&mut grid), expected);
        }
    }

    #[test]
    fn test_1d() {
        let cases: Vec<(Vec<Vec<i32>>, i32)> = vec![
            (vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]], 7),
            (vec![vec![5]], 5),
            (vec![vec![1, 2, 3]], 6),
            (vec![vec![1], vec![2], vec![3]], 6),
            (vec![vec![1, 2], vec![1, 1]], 3),
            (vec![vec![0, 0], vec![0, 0]], 0),
            (vec![vec![100, 100, 100], vec![100, 1, 100], vec![100, 1, 1]], 203),
            (vec![vec![1, 100], vec![1, 1]], 3),
        ];
        for (grid, expected) in cases {
            assert_eq!(Solution::min_path_sum_1d(grid), expected);
        }
    }
}
