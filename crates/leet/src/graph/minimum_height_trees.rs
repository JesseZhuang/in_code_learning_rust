pub struct Solution;

impl Solution {
    /// Topological sort: peel leaves layer by layer until ≤2 nodes remain.
    /// Time: O(n), Space: O(n)
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }

        // Build adjacency list and degree array — O(n)
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        let mut degree: Vec<usize> = vec![0; n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
            degree[u] += 1;
            degree[v] += 1;
        }

        // Collect initial leaves — O(n)
        let mut leaves: Vec<usize> = (0..n).filter(|&i| degree[i] == 1).collect();
        let mut remaining = n;

        // Peel leaves layer by layer — O(n) total across all iterations
        while remaining > 2 {
            remaining -= leaves.len();
            let mut new_leaves = Vec::new();
            for &leaf in &leaves {
                for &neighbor in &adj[leaf] {
                    degree[neighbor] -= 1;
                    if degree[neighbor] == 1 {
                        new_leaves.push(neighbor);
                    }
                }
            }
            leaves = new_leaves;
        }

        leaves.iter().map(|&x| x as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
    }

    #[test]
    fn test_two_nodes() {
        let mut result = Solution::find_min_height_trees(2, vec![vec![0, 1]]);
        result.sort();
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_star() {
        // n=4, edges=[[1,0],[1,2],[1,3]] → [1]
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        assert_eq!(Solution::find_min_height_trees(4, edges), vec![1]);
    }

    #[test]
    fn test_six_nodes() {
        // n=6, edges=[[3,0],[3,1],[3,2],[3,4],[5,4]] → [3,4]
        let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
        let mut result = Solution::find_min_height_trees(6, edges);
        result.sort();
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_path_odd() {
        // n=5, edges=[[0,1],[1,2],[2,3],[3,4]] → [2]
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::find_min_height_trees(5, edges), vec![2]);
    }

    #[test]
    fn test_path_even() {
        // n=4, edges=[[0,1],[1,2],[2,3]] → [1,2]
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let mut result = Solution::find_min_height_trees(4, edges);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }
}
