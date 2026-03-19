/// LeetCode 3212 - Count Submatrices With Equal Frequency of X and Y
struct Solution;

impl Solution {
    // 19ms, 66.84mb
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut res, mut col_x, mut col_y) = (0i32, vec![0i32; n], vec![0i32; n]);
        for i in 0..m {
            let (mut row_x, mut row_y) = (0i32, 0i32);
            for j in 0..n {
                match grid[i][j] {
                    'X' => col_x[j] += 1,
                    'Y' => col_y[j] += 1,
                    _ => {}
                }
                row_x += col_x[j];
                row_y += col_y[j];
                if row_x == row_y && row_x > 0 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']];
        assert_eq!(3, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn example_2() {
        let grid = vec![vec!['X', 'X'], vec!['X', 'Y']];
        assert_eq!(0, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn row_only_dots() {
        let grid = vec![vec!['.', '.']];
        assert_eq!(0, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn single_row_xy() {
        let grid = vec![vec!['X', 'Y']];
        assert_eq!(1, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn two_rows_xy_stacked() {
        let grid = vec![vec!['X'], vec!['Y']];
        assert_eq!(1, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn single_row_yx() {
        let grid = vec![vec!['Y', 'X']];
        assert_eq!(1, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn single_x() {
        let grid = vec![vec!['X']];
        assert_eq!(0, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn single_y() {
        let grid = vec![vec!['Y']];
        assert_eq!(0, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn spaced_xy() {
        let grid = vec![vec!['.', 'X', '.', 'Y']];
        assert_eq!(1, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn alternating_pattern() {
        let grid = vec![vec!['X', '.', 'Y', '.', 'X', 'Y']];
        assert_eq!(3, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn two_by_two_mixed() {
        let grid = vec![vec!['X', 'Y'], vec!['Y', 'X']];
        assert_eq!(3, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn grid_all_dots() {
        let grid = vec![vec!['.', '.'], vec!['.', '.']];
        assert_eq!(0, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn three_by_three_checker() {
        let grid = vec![
            vec!['X', 'Y', 'X'],
            vec!['Y', 'X', 'Y'],
            vec!['X', 'Y', 'X'],
        ];
        assert_eq!(5, Solution::number_of_submatrices(grid));
    }
}
