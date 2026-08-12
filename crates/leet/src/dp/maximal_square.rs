pub struct Solution;

impl Solution {
    /// DP with O(n) space. dp[j] = side length of largest square with bottom-right at (i, j).
    /// Transition: dp[j] = min(dp[j-1], prev_dp[j], prev_dp[j-1]) + 1 if matrix[i][j] == '1'.
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![0i32; n];
        let mut max_side = 0i32;

        // O(m * n) time: iterate over every cell exactly once
        for i in 0..m {
            let mut prev = 0i32; // prev_dp[j-1] (top-left diagonal)
            // O(n) per row
            for j in 0..n {
                let old = dp[j]; // save prev_dp[j] before overwrite
                if matrix[i][j] == '1' {
                    if i == 0 || j == 0 {
                        dp[j] = 1;
                    } else {
                        // dp[j-1]: left, old (dp[j] before update): top, prev: top-left
                        dp[j] = dp[j - 1].min(old).min(prev) + 1;
                    }
                    max_side = max_side.max(dp[j]);
                } else {
                    dp[j] = 0;
                }
                prev = old;
            }
        }

        max_side * max_side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(Solution::maximal_square(matrix), 4);
    }

    #[test]
    fn test_example2() {
        let matrix = vec![vec!['0', '1'], vec!['1', '0']];
        assert_eq!(Solution::maximal_square(matrix), 1);
    }

    #[test]
    fn test_all_zeros() {
        let matrix = vec![vec!['0', '0'], vec!['0', '0']];
        assert_eq!(Solution::maximal_square(matrix), 0);
    }

    #[test]
    fn test_all_ones_3x3() {
        let matrix = vec![
            vec!['1', '1', '1'],
            vec!['1', '1', '1'],
            vec!['1', '1', '1'],
        ];
        assert_eq!(Solution::maximal_square(matrix), 9);
    }

    #[test]
    fn test_single_one() {
        let matrix = vec![vec!['1']];
        assert_eq!(Solution::maximal_square(matrix), 1);
    }

    #[test]
    fn test_single_row() {
        let matrix = vec![vec!['0', '1', '0', '1', '1']];
        assert_eq!(Solution::maximal_square(matrix), 1);
    }
}
