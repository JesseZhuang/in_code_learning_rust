use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// BFS approach — O(n^2) time, O(n^2) space
    pub fn shortest_path_bfs(grid: &mut Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] != 0 || grid[n - 1][n - 1] != 0 {
            return -1; // start or end blocked
        }
        if n == 1 {
            return 1; // single cell
        }

        let dirs: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        let mut queue = VecDeque::new();
        queue.push_back((0usize, 0usize, 1i32)); // (row, col, distance)
        grid[0][0] = 1; // mark visited by setting to non-zero

        while let Some((r, c, dist)) = queue.pop_front() { // BFS guarantees shortest path
            for &(dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= n as i32 || nc >= n as i32 {
                    continue; // out of bounds
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] != 0 {
                    continue; // blocked or visited
                }
                if nr == n - 1 && nc == n - 1 {
                    return dist + 1; // reached destination
                }
                grid[nr][nc] = 1; // mark visited
                queue.push_back((nr, nc, dist + 1));
            }
        }
        -1 // no path found
    }

    /// A* approach with Chebyshev distance heuristic — O(n^2 log n) time, O(n^2) space
    pub fn shortest_path_a_star(grid: &mut Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] != 0 || grid[n - 1][n - 1] != 0 {
            return -1;
        }
        if n == 1 {
            return 1;
        }

        let dirs: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        // Chebyshev distance as admissible heuristic for 8-directional movement
        let heuristic = |r: usize, c: usize| -> i32 {
            std::cmp::max((n - 1 - r) as i32, (n - 1 - c) as i32)
        };

        // Min-heap: (f = g + h, g, row, col)
        let mut heap = BinaryHeap::new();
        let start_h = heuristic(0, 0);
        heap.push(Reverse((1 + start_h, 1i32, 0usize, 0usize))); // f, g, r, c
        grid[0][0] = 1; // mark visited

        while let Some(Reverse((_f, g, r, c))) = heap.pop() {
            for &(dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= n as i32 || nc >= n as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] != 0 {
                    continue;
                }
                let new_g = g + 1;
                if nr == n - 1 && nc == n - 1 {
                    return new_g; // reached destination
                }
                grid[nr][nc] = 1; // mark visited
                let f = new_g + heuristic(nr, nc); // f = g + h
                heap.push(Reverse((f, new_g, nr, nc)));
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_both(grid: Vec<Vec<i32>>, expected: i32) {
        let mut g1 = grid.clone();
        assert_eq!(Solution::shortest_path_bfs(&mut g1), expected);
        let mut g2 = grid;
        assert_eq!(Solution::shortest_path_a_star(&mut g2), expected);
    }

    #[test]
    fn test_two_by_two_path() {
        run_both(vec![vec![0, 1], vec![1, 0]], 2);
    }

    #[test]
    fn test_three_by_three_path() {
        run_both(vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]], 4);
    }

    #[test]
    fn test_blocked_start() {
        run_both(vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]], -1);
    }

    #[test]
    fn test_single_cell() {
        run_both(vec![vec![0]], 1);
    }

    #[test]
    fn test_blocked_end() {
        run_both(vec![vec![0, 0], vec![0, 1]], -1);
    }

    #[test]
    fn test_diagonal_path() {
        run_both(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]], 3);
    }
}
