use std::collections::VecDeque;

/// leet 200
pub struct Solution;

impl Solution {
    /// DFS. O(m*n) time, O(m*n) space (recursion stack).
    pub fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    count += 1;
                    Self::dfs(grid, i, j, m, n);
                }
            }
        }
        count
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        if i >= m || j >= n || grid[i][j] != '1' {
            return;
        }
        grid[i][j] = '0';
        Self::dfs(grid, i + 1, j, m, n);
        if i > 0 { Self::dfs(grid, i - 1, j, m, n); }
        Self::dfs(grid, i, j + 1, m, n);
        if j > 0 { Self::dfs(grid, i, j - 1, m, n); }
    }

    /// BFS. O(m*n) time, O(min(m,n)) space (queue).
    pub fn num_islands_bfs(grid: &mut Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut count = 0;
        let mut q = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    count += 1;
                    grid[i][j] = '0';
                    q.push_back((i, j));
                    while let Some((x, y)) = q.pop_front() {
                        for (dx, dy) in [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)] {
                            let (nx, ny) = (x.wrapping_add(dx), y.wrapping_add(dy));
                            if nx < m && ny < n && grid[nx][ny] == '1' {
                                grid[nx][ny] = '0';
                                q.push_back((nx, ny));
                            }
                        }
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn grid(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn example1_one_island() {
        let base = grid(&["11110", "11010", "11000", "00000"]);
        let mut g1 = base.clone();
        assert_eq!(Solution::num_islands(&mut g1), 1);
        let mut g2 = base;
        assert_eq!(Solution::num_islands_bfs(&mut g2), 1);
    }

    #[test]
    fn example2_three_islands() {
        let base = grid(&["11000", "11000", "00100", "00011"]);
        let mut g1 = base.clone();
        assert_eq!(Solution::num_islands(&mut g1), 3);
        let mut g2 = base;
        assert_eq!(Solution::num_islands_bfs(&mut g2), 3);
    }

    #[test]
    fn single_one() {
        let base = grid(&["1"]);
        let mut g1 = base.clone();
        assert_eq!(Solution::num_islands(&mut g1), 1);
        let mut g2 = base;
        assert_eq!(Solution::num_islands_bfs(&mut g2), 1);
    }

    #[test]
    fn single_zero() {
        let base = grid(&["0"]);
        let mut g1 = base.clone();
        assert_eq!(Solution::num_islands(&mut g1), 0);
        let mut g2 = base;
        assert_eq!(Solution::num_islands_bfs(&mut g2), 0);
    }

    #[test]
    fn all_land_2x2() {
        let base = grid(&["11", "11"]);
        let mut g1 = base.clone();
        assert_eq!(Solution::num_islands(&mut g1), 1);
        let mut g2 = base;
        assert_eq!(Solution::num_islands_bfs(&mut g2), 1);
    }

    #[test]
    fn diagonal_two_islands() {
        let base = grid(&["10", "01"]);
        let mut g1 = base.clone();
        assert_eq!(Solution::num_islands(&mut g1), 2);
        let mut g2 = base;
        assert_eq!(Solution::num_islands_bfs(&mut g2), 2);
    }
}
