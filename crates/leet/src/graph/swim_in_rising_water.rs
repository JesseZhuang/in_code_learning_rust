use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Min-Heap (Dijkstra-like) approach.
    /// Time: O(n^2 log n), Space: O(n^2)
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dist = vec![vec![i32::MAX; n]; n];
        // (max_elevation_on_path, row, col)
        let mut heap = BinaryHeap::new();
        dist[0][0] = grid[0][0];
        heap.push(Reverse((grid[0][0], 0usize, 0usize)));

        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some(Reverse((cost, r, c))) = heap.pop() {
            if r == n - 1 && c == n - 1 {
                return cost;
            }
            if cost > dist[r][c] {
                continue;
            }
            for (dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                let new_cost = cost.max(grid[nr][nc]);
                if new_cost < dist[nr][nc] {
                    dist[nr][nc] = new_cost;
                    heap.push(Reverse((new_cost, nr, nc)));
                }
            }
        }
        -1
    }

    /// Binary Search + BFS approach.
    /// Time: O(n^2 log n), Space: O(n^2)
    pub fn swim_in_water_bs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let max_val = n as i32 * n as i32 - 1;

        let can_reach = |t: i32| -> bool {
            if grid[0][0] > t || grid[n - 1][n - 1] > t {
                return false;
            }
            let mut visited = vec![vec![false; n]; n];
            let mut queue = VecDeque::new();
            visited[0][0] = true;
            queue.push_back((0usize, 0usize));
            let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            while let Some((r, c)) = queue.pop_front() {
                if r == n - 1 && c == n - 1 {
                    return true;
                }
                for (dr, dc) in &dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if !visited[nr][nc] && grid[nr][nc] <= t {
                        visited[nr][nc] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }
            false
        };

        let mut lo = grid[0][0].max(grid[n - 1][n - 1]);
        let mut hi = max_val;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if can_reach(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
        assert_eq!(Solution::swim_in_water_bs(vec![vec![0, 2], vec![1, 3]]), 3);
    }

    #[test]
    fn test_5x5() {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        assert_eq!(Solution::swim_in_water(grid.clone()), 16);
        assert_eq!(Solution::swim_in_water_bs(grid), 16);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::swim_in_water(vec![vec![0]]), 0);
        assert_eq!(Solution::swim_in_water_bs(vec![vec![0]]), 0);
    }

    #[test]
    fn test_two_by_two_a() {
        assert_eq!(Solution::swim_in_water(vec![vec![0, 1], vec![3, 2]]), 2);
        assert_eq!(Solution::swim_in_water_bs(vec![vec![0, 1], vec![3, 2]]), 2);
    }

    #[test]
    fn test_two_by_two_b() {
        assert_eq!(Solution::swim_in_water(vec![vec![3, 2], vec![0, 1]]), 3);
        assert_eq!(Solution::swim_in_water_bs(vec![vec![3, 2], vec![0, 1]]), 3);
    }
}
