use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// O(n^2) time, O(n^2) space
    pub fn shortest_bridge(grid: &mut Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut queue = VecDeque::new();

        // Find first island cell and DFS to mark entire island as 2
        'outer: for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    Self::dfs(grid, i as i32, j as i32, n, &mut queue);
                    break 'outer;
                }
            }
        }

        // Multi-source BFS from first island until we reach second island
        let dirs = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
        let mut steps = 0;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let (x, y) = queue.pop_front().unwrap();
                for (dx, dy) in &dirs {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                        let (ni, nj) = (nx as usize, ny as usize);
                        if grid[ni][nj] == 1 {
                            return steps;
                        }
                        if grid[ni][nj] == 0 {
                            grid[ni][nj] = 2;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
            steps += 1;
        }
        steps
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32, n: usize, queue: &mut VecDeque<(i32, i32)>) {
        if x < 0 || x >= n as i32 || y < 0 || y >= n as i32 {
            return;
        }
        let (i, j) = (x as usize, y as usize);
        if grid[i][j] != 1 {
            return;
        }
        grid[i][j] = 2;
        queue.push_back((x, y));
        Self::dfs(grid, x + 1, y, n, queue);
        Self::dfs(grid, x - 1, y, n, queue);
        Self::dfs(grid, x, y + 1, n, queue);
        Self::dfs(grid, x, y - 1, n, queue);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_bridge() {
        let mut grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::shortest_bridge(&mut grid), 1);

        let mut grid = vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]];
        assert_eq!(Solution::shortest_bridge(&mut grid), 2);

        let mut grid = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(Solution::shortest_bridge(&mut grid), 1);
    }
}
