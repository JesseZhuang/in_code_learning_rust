pub struct Solution;

impl Solution {
    /// Union Find with path compression and union by rank.
    /// Time: O(n * alpha(n)) ~= O(n), Space: O(n).
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent: Vec<usize> = (0..=n).collect();
        let mut rank = vec![0usize; n + 1];

        for edge in &edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            let (pu, pv) = (Self::find(&mut parent, u), Self::find(&mut parent, v));
            if pu == pv {
                return edge.clone(); // already connected — redundant
            }
            // union by rank
            if rank[pu] < rank[pv] {
                parent[pu] = pv;
            } else {
                parent[pv] = pu;
                if rank[pu] == rank[pv] {
                    rank[pu] += 1;
                }
            }
        }
        vec![]
    }

    fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]]; // path compression
            x = parent[x];
        }
        x
    }

    /// DFS cycle detection. O(n) time, O(n) space.
    pub fn find_redundant_connection_dfs(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut adj = vec![vec![]; n];
        for e in &edges {
            adj[e[0] as usize - 1].push(e[1] as usize - 1);
            adj[e[1] as usize - 1].push(e[0] as usize - 1);
        }

        let mut visited = vec![false; n];
        let mut parent = vec![usize::MAX; n];
        let mut cycle_start: Option<usize> = None;

        fn dfs(
            u: usize, adj: &[Vec<usize>], visited: &mut [bool],
            parent: &mut [usize], cycle_start: &mut Option<usize>,
        ) {
            visited[u] = true;
            for &v in &adj[u] {
                if !visited[v] {
                    parent[v] = u;
                    dfs(v, adj, visited, parent, cycle_start);
                } else if parent[u] != v && cycle_start.is_none() {
                    *cycle_start = Some(v);
                    parent[v] = u;
                }
            }
        }

        dfs(0, &adj, &mut visited, &mut parent, &mut cycle_start);

        let cs = cycle_start.unwrap();
        let mut cycle_nodes = std::collections::HashSet::new();
        let mut node = cs;
        loop {
            cycle_nodes.insert(node);
            node = parent[node];
            if node == cs {
                break;
            }
        }

        for i in (0..edges.len()).rev() {
            let u = edges[i][0] as usize - 1;
            let v = edges[i][1] as usize - 1;
            if cycle_nodes.contains(&u) && cycle_nodes.contains(&v) {
                return edges[i].clone();
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uf_example1() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        assert_eq!(vec![2, 3], Solution::find_redundant_connection(edges));
    }

    #[test]
    fn test_uf_example2() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
        assert_eq!(vec![1, 4], Solution::find_redundant_connection(edges));
    }

    #[test]
    fn test_uf_full_cycle() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1]];
        assert_eq!(vec![5, 1], Solution::find_redundant_connection(edges));
    }

    #[test]
    fn test_dfs_example1() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        assert_eq!(vec![2, 3], Solution::find_redundant_connection_dfs(edges));
    }

    #[test]
    fn test_dfs_example2() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
        assert_eq!(vec![1, 4], Solution::find_redundant_connection_dfs(edges));
    }

    #[test]
    fn test_dfs_full_cycle() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1]];
        assert_eq!(vec![5, 1], Solution::find_redundant_connection_dfs(edges));
    }
}
