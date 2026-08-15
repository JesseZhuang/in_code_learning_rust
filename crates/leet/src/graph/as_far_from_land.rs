/// leet 1162

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Multi-source BFS. O(n^2) time, O(n^2) space.
    pub fn max_distance_bfs(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut queue = VecDeque::new();

        // Enqueue all land cells; mark water as -1 (unvisited).
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i, j));
                } else {
                    grid[i][j] = -1;
                }
            }
        }

        // Edge cases: no water or no land.
        if queue.is_empty() || queue.len() == n * n {
            return -1;
        }

        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut dist = 0;

        // BFS level by level, expanding from all land simultaneously.
        while !queue.is_empty() {
            dist += 1;
            let level_size = queue.len();
            for _ in 0..level_size {
                let (x, y) = queue.pop_front().unwrap();
                for &(dx, dy) in &dirs {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if grid[nx][ny] == -1 {
                            grid[nx][ny] = dist;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }

        // dist was incremented one extra time after the last level.
        dist - 1
    }

    /// DP two passes. O(n^2) time, O(1) extra space (in-place).
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let inf = (2 * n) as i32; // Upper bound on any distance in an n x n grid.

        // First pass (top-left to bottom-right): land=0, water=min(top, left)+1.
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = inf;
                    if i > 0 {
                        grid[i][j] = grid[i][j].min(grid[i - 1][j] + 1);
                    }
                    if j > 0 {
                        grid[i][j] = grid[i][j].min(grid[i][j - 1] + 1);
                    }
                }
            }
        }

        // Second pass (bottom-right to top-left): min(cur, bottom+1, right+1), track max.
        let mut ans = 0;
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if i + 1 < n {
                    grid[i][j] = grid[i][j].min(grid[i + 1][j] + 1);
                }
                if j + 1 < n {
                    grid[i][j] = grid[i][j].min(grid[i][j + 1] + 1);
                }
                ans = ans.max(grid[i][j]);
            }
        }

        // ans == 0 means all land; ans >= inf means all water.
        if ans == 0 || ans >= inf {
            -1
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let grid = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(Solution::max_distance_bfs(grid.clone()), 2);
        assert_eq!(Solution::max_distance(grid), 2);
    }

    #[test]
    fn example2() {
        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::max_distance_bfs(grid.clone()), 4);
        assert_eq!(Solution::max_distance(grid), 4);
    }

    #[test]
    fn all_land() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::max_distance_bfs(grid.clone()), -1);
        assert_eq!(Solution::max_distance(grid), -1);
    }

    #[test]
    fn all_water() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::max_distance_bfs(grid.clone()), -1);
        assert_eq!(Solution::max_distance(grid), -1);
    }

    #[test]
    fn land_center() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::max_distance_bfs(grid.clone()), 2);
        assert_eq!(Solution::max_distance(grid), 2);
    }
}
