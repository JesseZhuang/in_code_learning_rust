use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Time: O(m * n), Space: O(m * n) for visited cells, the queue, and borders.
    pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let original_color = grid[row as usize][col as usize];
        let mut visited = vec![vec![false; cols as usize]; rows as usize];
        let mut queue = VecDeque::new();
        let mut borders = Vec::new();
        visited[row as usize][col as usize] = true;
        queue.push_back((row, col));

        while let Some((current_row, current_col)) = queue.pop_front() {
            let mut is_border = false;
            for (delta_row, delta_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next_row = current_row + delta_row;
                let next_col = current_col + delta_col;
                if next_row < 0 || next_row >= rows || next_col < 0 || next_col >= cols {
                    is_border = true;
                    continue;
                }

                let (next_row_index, next_col_index) = (next_row as usize, next_col as usize);
                if grid[next_row_index][next_col_index] != original_color {
                    is_border = true;
                } else if !visited[next_row_index][next_col_index] {
                    visited[next_row_index][next_col_index] = true;
                    queue.push_back((next_row, next_col));
                }
            }
            if is_border {
                borders.push((current_row as usize, current_col as usize));
            }
        }

        for (border_row, border_col) in borders {
            grid[border_row][border_col] = color;
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_with_different_neighbor() {
        let grid = vec![vec![1, 1], vec![1, 2]];
        assert_eq!(
            Solution::color_border(grid, 0, 0, 3),
            vec![vec![3, 3], vec![3, 2]]
        );
    }

    #[test]
    fn center_of_uniform_region_is_not_a_border() {
        let grid = vec![vec![1; 3]; 3];
        let expected = vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]];
        assert_eq!(Solution::color_border(grid, 1, 1, 2), expected);
    }

    #[test]
    fn same_color_is_no_op() {
        let grid = vec![vec![1, 1], vec![1, 2]];
        assert_eq!(Solution::color_border(grid.clone(), 0, 0, 1), grid);
    }

    #[test]
    fn leaves_disconnected_regions_unchanged() {
        let grid = vec![vec![1, 1, 2], vec![2, 2, 2], vec![1, 1, 1]];
        let expected = vec![vec![3, 3, 2], vec![2, 2, 2], vec![1, 1, 1]];
        assert_eq!(Solution::color_border(grid, 0, 0, 3), expected);
    }

    #[test]
    fn single_cell_is_a_border() {
        assert_eq!(
            Solution::color_border(vec![vec![1]], 0, 0, 3),
            vec![vec![3]]
        );
    }

    #[test]
    fn surrounded_region_has_border_next_to_other_color() {
        let grid = vec![
            vec![2, 2, 2, 2, 2],
            vec![2, 1, 1, 1, 2],
            vec![2, 1, 1, 1, 2],
            vec![2, 1, 1, 1, 2],
            vec![2, 2, 2, 2, 2],
        ];
        let expected = vec![
            vec![2, 2, 2, 2, 2],
            vec![2, 3, 3, 3, 2],
            vec![2, 3, 1, 3, 2],
            vec![2, 3, 3, 3, 2],
            vec![2, 2, 2, 2, 2],
        ];
        assert_eq!(Solution::color_border(grid, 2, 2, 3), expected);
    }

    #[test]
    fn fifty_by_fifty_uniform_region_keeps_interior() {
        let grid = vec![vec![1; 50]; 50];
        let mut expected = grid.clone();
        for index in 0..50 {
            expected[0][index] = 3;
            expected[49][index] = 3;
            expected[index][0] = 3;
            expected[index][49] = 3;
        }
        assert_eq!(Solution::color_border(grid, 25, 25, 3), expected);
    }
}
