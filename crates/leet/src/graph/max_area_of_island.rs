use std::collections::VecDeque;

/// leet 695
pub struct Solution;

impl Solution {
    /// DFS. O(m*n) time, O(m*n) space (recursion stack).
    pub fn max_area_of_island(grid: &mut Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut res = 0;
        for i in 0..m { // O(m)
            for j in 0..n { // O(n)
                if grid[i][j] == 1 {
                    res = res.max(Self::dfs(grid, i, j, m, n));
                }
            }
        }
        res
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, m: usize, n: usize) -> i32 {
        if i >= m || j >= n || grid[i][j] != 1 {
            return 0;
        }
        grid[i][j] = 0;
        let mut area = 1;
        area += Self::dfs(grid, i + 1, j, m, n);
        if i > 0 { area += Self::dfs(grid, i - 1, j, m, n); }
        area += Self::dfs(grid, i, j + 1, m, n);
        if j > 0 { area += Self::dfs(grid, i, j - 1, m, n); }
        area
    }

    /// BFS. O(m*n) time, O(min(m,n)) space (queue).
    pub fn max_area_of_island_bfs(grid: &mut Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut res = 0;
        let mut q = VecDeque::new();
        for i in 0..m { // O(m)
            for j in 0..n { // O(n)
                if grid[i][j] == 1 {
                    let mut area = 0;
                    grid[i][j] = 0;
                    q.push_back((i, j));
                    while let Some((x, y)) = q.pop_front() { // O(m*n) total
                        area += 1;
                        for (dx, dy) in [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)] {
                            let (nx, ny) = (x.wrapping_add(dx), y.wrapping_add(dy));
                            if nx < m && ny < n && grid[nx][ny] == 1 {
                                grid[nx][ny] = 0;
                                q.push_back((nx, ny));
                            }
                        }
                    }
                    res = res.max(area);
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
    fn example1() {
        let mut grid = vec![
            vec![0,0,1,0,0,0,0,1,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,1,1,0,1,0,0,0,0,0,0,0,0],
            vec![0,1,0,0,1,1,0,0,1,0,1,0,0],
            vec![0,1,0,0,1,1,0,0,1,1,1,0,0],
            vec![0,0,0,0,0,0,0,0,0,0,1,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,0,0,0,0],
        ];
        assert_eq!(6, Solution::max_area_of_island(&mut grid));
    }

    #[test]
    fn example1_bfs() {
        let mut grid = vec![
            vec![0,0,1,0,0,0,0,1,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,1,1,0,1,0,0,0,0,0,0,0,0],
            vec![0,1,0,0,1,1,0,0,1,0,1,0,0],
            vec![0,1,0,0,1,1,0,0,1,1,1,0,0],
            vec![0,0,0,0,0,0,0,0,0,0,1,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,0,0,0,0],
        ];
        assert_eq!(6, Solution::max_area_of_island_bfs(&mut grid));
    }

    #[test]
    fn all_water() {
        let mut g1 = vec![vec![0,0,0,0,0,0,0,0]];
        assert_eq!(0, Solution::max_area_of_island(&mut g1));
        let mut g2 = vec![vec![0,0,0,0,0,0,0,0]];
        assert_eq!(0, Solution::max_area_of_island_bfs(&mut g2));
    }

    #[test]
    fn single_cell() {
        let mut g1 = vec![vec![1]];
        assert_eq!(1, Solution::max_area_of_island(&mut g1));
        let mut g2 = vec![vec![1]];
        assert_eq!(1, Solution::max_area_of_island_bfs(&mut g2));
    }

    #[test]
    fn all_land() {
        let mut g1 = vec![vec![1,1],vec![1,1]];
        assert_eq!(4, Solution::max_area_of_island(&mut g1));
        let mut g2 = vec![vec![1,1],vec![1,1]];
        assert_eq!(4, Solution::max_area_of_island_bfs(&mut g2));
    }

    #[test]
    fn diagonal_not_connected() {
        let mut g1 = vec![vec![1,0],vec![0,1]];
        assert_eq!(1, Solution::max_area_of_island(&mut g1));
        let mut g2 = vec![vec![1,0],vec![0,1]];
        assert_eq!(1, Solution::max_area_of_island_bfs(&mut g2));
    }
}
