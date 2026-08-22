/// leet 785
pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    /// BFS coloring. O(V+E) time, O(V) space.
    pub fn is_bipartite_bfs(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut color = vec![-1i8; n]; // O(V) space; -1 = unvisited, 0/1 = two colors

        for start in 0..n { // handle disconnected components
            if color[start] != -1 {
                continue;
            }
            color[start] = 0;
            let mut queue = VecDeque::new(); // O(V) space worst case
            queue.push_back(start);

            while let Some(u) = queue.pop_front() { // BFS traversal O(V+E)
                for &v in &graph[u] { // scan neighbors O(degree(u))
                    let v = v as usize;
                    if color[v] == -1 {
                        color[v] = 1 - color[u]; // assign opposite color
                        queue.push_back(v);
                    } else if color[v] == color[u] {
                        return false; // same color on both ends → not bipartite
                    }
                }
            }
        }
        true
    }

    /// Union-Find approach. For each node u, union all its neighbors together;
    /// if u ends up in the same set as any neighbor, the graph is not bipartite.
    /// O(V*α(V) + E) time, O(V) space.
    pub fn is_bipartite_uf(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut parent: Vec<usize> = (0..n).collect(); // O(V) space
        let mut rank = vec![0u32; n]; // O(V) space

        for u in 0..n { // O(V) outer loop
            if graph[u].is_empty() {
                continue;
            }
            let first_neighbor = graph[u][0] as usize;
            for &v in &graph[u] { // union all neighbors of u together O(degree(u)*α(V))
                let v = v as usize;
                Self::union(&mut parent, &mut rank, first_neighbor, v);
            }
            // check: u must NOT be in the same set as its neighbors
            if Self::find(&mut parent, u) == Self::find(&mut parent, first_neighbor) {
                return false; // u in same component as neighbor → odd cycle
            }
        }
        true
    }

    /// Find with path compression. Amortized O(α(n)).
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]); // path compression
        }
        parent[x]
    }

    /// Union by rank. Returns true if two distinct components were merged.
    fn union(parent: &mut Vec<usize>, rank: &mut Vec<u32>, x: usize, y: usize) -> bool {
        let rx = Self::find(parent, x);
        let ry = Self::find(parent, y);
        if rx == ry {
            return false;
        }
        match rank[rx].cmp(&rank[ry]) { // union by rank keeps tree flat
            std::cmp::Ordering::Less => parent[rx] = ry,
            std::cmp::Ordering::Greater => parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                parent[ry] = rx;
                rank[rx] += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run(graph: Vec<Vec<i32>>, expected: bool) {
        assert_eq!(Solution::is_bipartite_bfs(graph.clone()), expected);
        assert_eq!(Solution::is_bipartite_uf(graph), expected);
    }

    #[test]
    fn example1_bipartite() {
        run(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]], true);
    }

    #[test]
    fn example2_not_bipartite() {
        run(
            vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]],
            false,
        );
    }

    #[test]
    fn single_node() {
        run(vec![vec![]], true);
    }

    #[test]
    fn disconnected_bipartite() {
        run(vec![vec![1], vec![0], vec![3], vec![2]], true);
    }

    #[test]
    fn disconnected_with_odd_cycle() {
        run(
            vec![vec![1], vec![0], vec![3, 4], vec![2, 4], vec![2, 3]],
            false,
        );
    }

    #[test]
    fn no_edges() {
        run(vec![vec![], vec![], vec![]], true);
    }

    #[test]
    fn triangle_not_bipartite() {
        run(vec![vec![1, 2], vec![0, 2], vec![0, 1]], false);
    }

    #[test]
    fn k33_bipartite() {
        run(
            vec![
                vec![3, 4, 5],
                vec![3, 4, 5],
                vec![3, 4, 5],
                vec![0, 1, 2],
                vec![0, 1, 2],
                vec![0, 1, 2],
            ],
            true,
        );
    }
}
