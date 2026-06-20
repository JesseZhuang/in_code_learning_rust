use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// DFS + memoization approach.
    /// Time: O(m*n) — each cell visited once due to memo.
    /// Space: O(m*n) — memo table + recursion stack.
    pub fn longest_increasing_path_dfs(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        let mut memo = vec![vec![0i32; n]; m]; // O(m*n) space
        let mut ans = 1;
        for i in 0..m {
            for j in 0..n {
                ans = ans.max(Self::dfs(&matrix, &mut memo, i, j, m, n));
            }
        }
        ans
    }

    fn dfs(matrix: &[Vec<i32>], memo: &mut [Vec<i32>], i: usize, j: usize, m: usize, n: usize) -> i32 {
        if memo[i][j] != 0 {
            return memo[i][j]; // O(1) cached lookup
        }
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut best = 1;
        for &(di, dj) in &dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                let (ni, nj) = (ni as usize, nj as usize);
                if matrix[ni][nj] > matrix[i][j] {
                    best = best.max(1 + Self::dfs(matrix, memo, ni, nj, m, n)); // recurse only to strictly larger neighbors
                }
            }
        }
        memo[i][j] = best;
        best
    }

    /// Topological sort BFS (Kahn's algorithm on the DAG of increasing edges).
    /// Time: O(m*n) — each cell enqueued/dequeued once.
    /// Space: O(m*n) — in-degree table + queue.
    pub fn longest_increasing_path_bfs(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        // Build in-degree: count how many neighbors are strictly smaller (i.e., edges pointing into this cell).
        let mut indegree = vec![vec![0u32; n]; m]; // O(m*n) space
        for i in 0..m {
            for j in 0..n {
                for &(di, dj) in &dirs {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        let (ni, nj) = (ni as usize, nj as usize);
                        if matrix[ni][nj] < matrix[i][j] {
                            indegree[i][j] += 1; // edge from smaller neighbor to this cell
                        }
                    }
                }
            }
        }

        // Enqueue all cells with in-degree 0 (local minima — path starts here).
        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if indegree[i][j] == 0 {
                    queue.push_back((i, j));
                }
            }
        }

        // BFS layer by layer; each layer adds 1 to the path length.
        let mut layers = 0;
        while !queue.is_empty() {
            layers += 1;
            for _ in 0..queue.len() {
                let (i, j) = queue.pop_front().unwrap();
                for &(di, dj) in &dirs {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        let (ni, nj) = (ni as usize, nj as usize);
                        if matrix[ni][nj] > matrix[i][j] {
                            indegree[ni][nj] -= 1;
                            if indegree[ni][nj] == 0 {
                                queue.push_back((ni, nj)); // all smaller predecessors processed
                            }
                        }
                    }
                }
            }
        }
        layers
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn cases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]], 4),
            (vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]], 4),
            (vec![vec![1]], 1),
            (vec![vec![1, 2]], 2),
            (vec![vec![7, 8, 9], vec![9, 7, 6], vec![7, 2, 3]], 6),
            (vec![vec![1, 1], vec![1, 1]], 1),
        ]
    }

    #[test]
    fn test_dfs() {
        for (matrix, expected) in cases() {
            assert_eq!(Solution::longest_increasing_path_dfs(matrix), expected);
        }
    }

    #[test]
    fn test_bfs() {
        for (matrix, expected) in cases() {
            assert_eq!(Solution::longest_increasing_path_bfs(matrix), expected);
        }
    }
}
