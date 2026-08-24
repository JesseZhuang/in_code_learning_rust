pub struct Solution;

impl Solution {
    /// Histogram + monotonic stack approach.
    /// Build a histogram of heights row by row, then apply largest-rectangle-in-histogram per row.
    /// Time: O(m * n), Space: O(n)
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let cols = matrix[0].len();
        let mut heights = vec![0i32; cols];
        let mut max_area = 0;

        // O(m) — iterate each row
        for row in &matrix {
            // O(n) — update histogram heights
            for j in 0..cols {
                heights[j] = if row[j] == '1' { heights[j] + 1 } else { 0 };
            }
            max_area = max_area.max(Self::largest_rectangle_area(&heights));
        }
        max_area
    }

    /// Largest rectangle in histogram using monotonic stack. O(n) time and space.
    fn largest_rectangle_area(heights: &[i32]) -> i32 {
        let n = heights.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;

        // O(n) — each index pushed and popped at most once
        for i in 0..=n {
            let cur_h = if i == n { 0 } else { heights[i] };
            while let Some(&top) = stack.last() {
                if heights[top] <= cur_h {
                    break;
                }
                stack.pop();
                let h = heights[top];
                let w = if let Some(&left) = stack.last() {
                    i as i32 - left as i32 - 1
                } else {
                    i as i32
                };
                max_area = max_area.max(h * w);
            }
            stack.push(i);
        }
        max_area
    }

    /// DP approach tracking height, left boundary, and right boundary per cell.
    /// Time: O(m * n), Space: O(n)
    pub fn maximal_rectangle_dp(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut height = vec![0i32; cols];
        let mut left = vec![0i32; cols];
        let mut right = vec![cols as i32; cols];
        let mut max_area = 0;

        // O(m) — iterate each row
        for i in 0..rows {
            let mut cur_left = 0i32;
            let mut cur_right = cols as i32;

            // O(n) — update heights
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    height[j] += 1;
                } else {
                    height[j] = 0;
                }
            }

            // O(n) — update left boundaries (leftmost column where continuous 1s start)
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    left[j] = left[j].max(cur_left);
                } else {
                    left[j] = 0;
                    cur_left = j as i32 + 1;
                }
            }

            // O(n) — update right boundaries (one past rightmost column of continuous 1s)
            for j in (0..cols).rev() {
                if matrix[i][j] == '1' {
                    right[j] = right[j].min(cur_right);
                } else {
                    right[j] = cols as i32;
                    cur_right = j as i32;
                }
            }

            // O(n) — compute area for each cell
            for j in 0..cols {
                max_area = max_area.max(height[j] * (right[j] - left[j]));
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_matrix(v: &[&[char]]) -> Vec<Vec<char>> {
        v.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn test_example() {
        // Standard LeetCode example, answer = 6
        let matrix = to_matrix(&[
            &['1', '0', '1', '0', '0'],
            &['1', '0', '1', '1', '1'],
            &['1', '1', '1', '1', '1'],
            &['1', '0', '0', '1', '0'],
        ]);
        assert_eq!(Solution::maximal_rectangle(matrix.clone()), 6);
        assert_eq!(Solution::maximal_rectangle_dp(matrix), 6);
    }

    #[test]
    fn test_empty_matrix() {
        let matrix: Vec<Vec<char>> = vec![];
        assert_eq!(Solution::maximal_rectangle(matrix.clone()), 0);
        assert_eq!(Solution::maximal_rectangle_dp(matrix), 0);
    }

    #[test]
    fn test_single_zero() {
        let matrix = to_matrix(&[&['0']]);
        assert_eq!(Solution::maximal_rectangle(matrix.clone()), 0);
        assert_eq!(Solution::maximal_rectangle_dp(matrix), 0);
    }

    #[test]
    fn test_single_one() {
        let matrix = to_matrix(&[&['1']]);
        assert_eq!(Solution::maximal_rectangle(matrix.clone()), 1);
        assert_eq!(Solution::maximal_rectangle_dp(matrix), 1);
    }

    #[test]
    fn test_all_ones() {
        let matrix = to_matrix(&[
            &['1', '1', '1'],
            &['1', '1', '1'],
            &['1', '1', '1'],
        ]);
        assert_eq!(Solution::maximal_rectangle(matrix.clone()), 9);
        assert_eq!(Solution::maximal_rectangle_dp(matrix), 9);
    }

    #[test]
    fn test_all_zeros() {
        let matrix = to_matrix(&[
            &['0', '0', '0'],
            &['0', '0', '0'],
        ]);
        assert_eq!(Solution::maximal_rectangle(matrix.clone()), 0);
        assert_eq!(Solution::maximal_rectangle_dp(matrix), 0);
    }

    #[test]
    fn test_single_row() {
        let matrix = to_matrix(&[&['1', '1', '0', '1', '1', '1']]);
        assert_eq!(Solution::maximal_rectangle(matrix.clone()), 3);
        assert_eq!(Solution::maximal_rectangle_dp(matrix), 3);
    }

    #[test]
    fn test_single_column() {
        let matrix = to_matrix(&[&['1'], &['1'], &['0'], &['1']]);
        assert_eq!(Solution::maximal_rectangle(matrix.clone()), 2);
        assert_eq!(Solution::maximal_rectangle_dp(matrix), 2);
    }
}
