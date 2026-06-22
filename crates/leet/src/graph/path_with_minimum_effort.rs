use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

/// LeetCode 1631: Path With Minimum Effort
pub struct Solution;

impl Solution {
    /// Dijkstra's algorithm with min-heap.
    /// State: (effort, row, col)
    /// For each neighbor, new_effort = max(current_effort, abs_diff)
    /// O(m*n*log(m*n)) time, O(m*n) space
    pub fn minimum_effort_path_dijkstra(heights: Vec<Vec<i32>>) -> i32 {
        if heights.is_empty() || heights[0].is_empty() {
            return 0;
        }

        let rows = heights.len();
        let cols = heights[0].len();

        // Edge case: single cell
        if rows == 1 && cols == 1 {
            return 0;
        }

        // Distance array tracking minimum effort to reach each cell
        let mut dist = vec![vec![i32::MAX; cols]; rows];
        dist[0][0] = 0;

        // Min-heap: (effort, row, col)
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0, 0)));

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some(Reverse((effort, r, c))) = heap.pop() {
            // If we reached the destination, return the effort
            if r == rows - 1 && c == cols - 1 {
                return effort;
            }

            // Skip if we've already found a better path to this cell
            if effort > dist[r][c] {
                continue;
            }

            // Explore neighbors
            for &(dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    let edge_effort = (heights[r][c] - heights[nr][nc]).abs();
                    let new_effort = effort.max(edge_effort);

                    if new_effort < dist[nr][nc] {
                        dist[nr][nc] = new_effort;
                        heap.push(Reverse((new_effort, nr, nc)));
                    }
                }
            }
        }

        dist[rows - 1][cols - 1]
    }

    /// Binary search on effort + BFS to check if path exists with given max effort.
    /// O(m*n*log(max_height)) time, O(m*n) space
    pub fn minimum_effort_path_binary_search(heights: Vec<Vec<i32>>) -> i32 {
        if heights.is_empty() || heights[0].is_empty() {
            return 0;
        }

        let rows = heights.len();
        let cols = heights[0].len();

        if rows == 1 && cols == 1 {
            return 0;
        }

        // Binary search on the effort [0, 10^6]
        let mut left = 0;
        let mut right = 1_000_000;
        let mut result = right;

        while left <= right {
            let mid = left + (right - left) / 2;

            if Self::can_reach(mid, &heights, rows, cols) {
                result = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        result
    }

    /// BFS to check if we can reach bottom-right with max effort <= max_effort
    fn can_reach(max_effort: i32, heights: &[Vec<i32>], rows: usize, cols: usize) -> bool {
        let mut visited = vec![vec![false; cols]; rows];
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        visited[0][0] = true;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((r, c)) = queue.pop_front() {
            if r == rows - 1 && c == cols - 1 {
                return true;
            }

            for &(dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if !visited[nr][nc] {
                        let edge_effort = (heights[r][c] - heights[nr][nc]).abs();
                        if edge_effort <= max_effort {
                            visited[nr][nc] = true;
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
        }

        false
    }

    /// Default implementation using Dijkstra
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        Self::minimum_effort_path_dijkstra(heights)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check(heights: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(
            Solution::minimum_effort_path_dijkstra(heights.clone()),
            expected,
            "Dijkstra failed"
        );
        assert_eq!(
            Solution::minimum_effort_path_binary_search(heights.clone()),
            expected,
            "Binary search failed"
        );
    }

    #[test]
    fn example1() {
        let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
        check(heights, 2);
    }

    #[test]
    fn example2() {
        let heights = vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]];
        check(heights, 1);
    }

    #[test]
    fn example3() {
        let heights = vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1],
        ];
        check(heights, 0);
    }

    #[test]
    fn single_cell() {
        let heights = vec![vec![5]];
        check(heights, 0);
    }

    #[test]
    fn single_row() {
        let heights = vec![vec![1, 10, 6, 7, 9, 10, 4, 9]];
        check(heights, 9);
    }

    #[test]
    fn single_column() {
        let heights = vec![vec![1], vec![10], vec![6], vec![7]];
        check(heights, 9);
    }

    #[test]
    fn large_difference() {
        let heights = vec![vec![1, 100], vec![100, 1]];
        check(heights, 99);
    }
}
