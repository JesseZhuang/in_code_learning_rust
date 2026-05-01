// LeetCode 1192, hard, tags: dfs, graph, bi-connected component.

pub struct Solution;

impl Solution {
    /// O(n+e) time, O(n+e) space. Tarjan's algorithm.
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in &connections {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }
        let mut ranks = vec![0i32; n];
        let mut res = vec![];
        Self::dfs(&adj, &mut ranks, &mut res, 0, 0, 1);
        res
    }

    fn dfs(
        adj: &[Vec<usize>],
        ranks: &mut [i32],
        res: &mut Vec<Vec<i32>>,
        parent: usize,
        v: usize,
        rank: i32,
    ) {
        if ranks[v] != 0 {
            return;
        }
        ranks[v] = rank;
        for &w in &adj[v] {
            if w == parent {
                continue;
            }
            Self::dfs(adj, ranks, res, v, w, rank + 1);
            ranks[v] = ranks[v].min(ranks[w]);
            if rank < ranks[w] {
                res.push(vec![v as i32, w as i32]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let connections = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]];
        let result = Solution::critical_connections(4, connections);
        assert_eq!(result, vec![vec![1, 3]]);
    }

    #[test]
    fn test_example2() {
        let connections = vec![vec![0, 1]];
        let result = Solution::critical_connections(2, connections);
        assert_eq!(result, vec![vec![0, 1]]);
    }
}
