/// leet 2352

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Expected O(n^2) time and O(n^2) space for the row-frequency map and column vectors.
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_counts = HashMap::new();
        for row in &grid {
            *row_counts.entry(row.clone()).or_insert(0) += 1;
        }

        (0..grid.len())
            .map(|column| {
                let values = grid.iter().map(|row| row[column]).collect::<Vec<_>>();
                row_counts.get(&values).copied().unwrap_or(0)
            })
            .sum()
    }

    /// O(n^3) time and O(1) extra space by comparing every row with every column directly.
    pub fn equal_pairs_brute_force(grid: Vec<Vec<i32>>) -> i32 {
        let size = grid.len();
        let mut pairs = 0;

        for row in 0..size {
            for column in 0..size {
                if (0..size).all(|index| grid[row][index] == grid[index][column]) {
                    pairs += 1;
                }
            }
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_both_approaches(grid: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(Solution::equal_pairs(grid.clone()), expected);
        assert_eq!(Solution::equal_pairs_brute_force(grid), expected);
    }

    #[test]
    fn official_examples() {
        assert_both_approaches(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]], 1);
        assert_both_approaches(
            vec![vec![3, 1, 2, 2], vec![1, 4, 4, 5], vec![2, 4, 2, 2], vec![2, 4, 2, 2]],
            3,
        );
    }

    #[test]
    fn single_cell_grid() {
        assert_both_approaches(vec![vec![42]], 1);
    }

    #[test]
    fn all_equal_rows_and_columns_count_duplicates() {
        assert_both_approaches(vec![vec![5, 5], vec![5, 5]], 4);
    }

    #[test]
    fn no_matching_pairs() {
        assert_both_approaches(vec![vec![1, 2], vec![3, 4]], 0);
    }
}
