pub struct Solution;

impl Solution {
    /// Union-Find approach: Time O(n + e*α(n)), Space O(n)
    pub fn count_components_uf(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).collect(); // O(n) space
        let mut rank = vec![0u32; n]; // O(n) space
        let mut components = n as i32;

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]); // path compression → amortized O(α(n))
            }
            parent[x]
        }

        for edge in &edges { // O(e) iterations
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            let ra = find(&mut parent, a); // O(α(n)) amortized
            let rb = find(&mut parent, b);
            if ra != rb {
                // union by rank
                match rank[ra].cmp(&rank[rb]) {
                    std::cmp::Ordering::Less => parent[ra] = rb,
                    std::cmp::Ordering::Greater => parent[rb] = ra,
                    std::cmp::Ordering::Equal => {
                        parent[rb] = ra;
                        rank[ra] += 1;
                    }
                }
                components -= 1;
            }
        }

        components
    }

    /// DFS approach: Time O(n + e), Space O(n + e) for adjacency list + visited
    pub fn count_components_dfs(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n]; // O(n + e) space
        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            adj[a].push(b); // O(e) total edges stored
            adj[b].push(a);
        }

        let mut visited = vec![false; n]; // O(n) space
        let mut components = 0i32;

        for i in 0..n { // O(n) outer loop
            if !visited[i] {
                components += 1;
                // iterative DFS to avoid stack overflow
                let mut stack = vec![i];
                while let Some(node) = stack.pop() { // each node visited once → O(n) total
                    if visited[node] {
                        continue;
                    }
                    visited[node] = true;
                    for &neighbor in &adj[node] { // each edge traversed twice total → O(e)
                        if !visited[neighbor] {
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }

        components
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // 5 nodes, edges: 0-1, 1-2, 3-4 → 2 components: {0,1,2} and {3,4}
        let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::count_components_uf(5, edges.clone()), 2);
        assert_eq!(Solution::count_components_dfs(5, edges), 2);
    }

    #[test]
    fn example2_all_connected() {
        // 5 nodes, fully connected chain → 1 component
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::count_components_uf(5, edges.clone()), 1);
        assert_eq!(Solution::count_components_dfs(5, edges), 1);
    }

    #[test]
    fn no_edges() {
        // 4 nodes, no edges → 4 components
        let edges: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::count_components_uf(4, edges.clone()), 4);
        assert_eq!(Solution::count_components_dfs(4, edges), 4);
    }

    #[test]
    fn single_node() {
        // 1 node, no edges → 1 component
        let edges: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::count_components_uf(1, edges.clone()), 1);
        assert_eq!(Solution::count_components_dfs(1, edges), 1);
    }

    #[test]
    fn cycle_triangle() {
        // 3 nodes forming a triangle → 1 component
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        assert_eq!(Solution::count_components_uf(3, edges.clone()), 1);
        assert_eq!(Solution::count_components_dfs(3, edges), 1);
    }

    #[test]
    fn three_components() {
        // 6 nodes, 3 pairs → 3 components: {0,1}, {2,3}, {4,5}
        let edges = vec![vec![0, 1], vec![2, 3], vec![4, 5]];
        assert_eq!(Solution::count_components_uf(6, edges.clone()), 3);
        assert_eq!(Solution::count_components_dfs(6, edges), 3);
    }
}
