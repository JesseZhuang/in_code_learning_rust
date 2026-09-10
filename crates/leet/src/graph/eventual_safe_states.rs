use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// DFS 3-coloring approach — O(V+E) time, O(V) space.
    /// color[i]: 0 = unvisited, 1 = visiting (on current path), 2 = safe.
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut color = vec![0u8; n]; // O(V) space for color array
        let mut result = Vec::new();
        for i in 0..n {
            if Self::dfs(i, &graph, &mut color) {
                result.push(i as i32);
            }
        }
        result // already sorted since we iterate 0..n in order
    }

    /// Returns true if node is safe (all paths lead to terminal).
    fn dfs(node: usize, graph: &[Vec<i32>], color: &mut [u8]) -> bool {
        if color[node] != 0 {
            return color[node] == 2; // visited: safe only if color==2
        }
        color[node] = 1; // mark visiting — cycle detection
        for &next in &graph[node] {
            // O(E) total across all DFS calls
            if !Self::dfs(next as usize, graph, color) {
                return false; // neighbor is in cycle or leads to cycle
            }
        }
        color[node] = 2; // all paths safe — mark node safe
        true
    }

    /// Reverse graph + topological sort BFS — O(V+E) time, O(V+E) space.
    /// Start from terminal nodes (out-degree 0) and propagate safety inward.
    pub fn eventual_safe_nodes_bfs(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut out_degree = vec![0usize; n];
        let mut reverse: Vec<Vec<usize>> = vec![vec![]; n]; // O(V+E) space for reverse graph

        // Build reverse adjacency list — O(V+E)
        for (u, neighbors) in graph.iter().enumerate() {
            out_degree[u] = neighbors.len();
            for &v in neighbors {
                reverse[v as usize].push(u); // edge u->v becomes v->u in reverse
            }
        }

        // Seed queue with terminal nodes (out-degree 0)
        let mut queue = VecDeque::new();
        for i in 0..n {
            if out_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut safe = vec![false; n];
        // BFS: propagate safety backwards through reverse edges — O(V+E)
        while let Some(node) = queue.pop_front() {
            safe[node] = true;
            for &prev in &reverse[node] {
                out_degree[prev] -= 1; // one fewer unsafe outgoing edge
                if out_degree[prev] == 0 {
                    queue.push_back(prev); // all outgoing edges lead to safe nodes
                }
            }
        }

        // Collect safe nodes — already sorted by construction
        (0..n).filter(|&i| safe[i]).map(|i| i as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1_dfs() {
        // Nodes 5,6 are terminal; 2->5, 4->5 are safe; node 0,1,3 form cycle via 0->1->3->0
        let graph = vec![
            vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![], vec![],
        ];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![2, 4, 5, 6]);
    }

    #[test]
    fn test_example1_bfs() {
        let graph = vec![
            vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![], vec![],
        ];
        assert_eq!(Solution::eventual_safe_nodes_bfs(graph), vec![2, 4, 5, 6]);
    }

    #[test]
    fn test_example2_dfs() {
        // Only node 4 is terminal and safe; all others reach cycle 0->1->1 or 0->3->0
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![4]);
    }

    #[test]
    fn test_example2_bfs() {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        assert_eq!(Solution::eventual_safe_nodes_bfs(graph), vec![4]);
    }

    #[test]
    fn test_all_terminal_dfs() {
        // No edges at all — every node is terminal and safe
        let graph = vec![vec![], vec![], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![0, 1, 2]);
    }

    #[test]
    fn test_all_terminal_bfs() {
        let graph = vec![vec![], vec![], vec![]];
        assert_eq!(Solution::eventual_safe_nodes_bfs(graph), vec![0, 1, 2]);
    }

    #[test]
    fn test_single_node_dfs() {
        let graph = vec![vec![]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![0]);
    }

    #[test]
    fn test_single_node_bfs() {
        let graph = vec![vec![]];
        assert_eq!(Solution::eventual_safe_nodes_bfs(graph), vec![0]);
    }

    #[test]
    fn test_self_loop_dfs() {
        // Node 0 has self-loop — always revisits itself, never reaches terminal
        let graph = vec![vec![0]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![]);
    }

    #[test]
    fn test_self_loop_bfs() {
        let graph = vec![vec![0]];
        assert_eq!(Solution::eventual_safe_nodes_bfs(graph), vec![]);
    }

    #[test]
    fn test_two_node_cycle_dfs() {
        let graph = vec![vec![1], vec![0]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![]);
    }

    #[test]
    fn test_two_node_cycle_bfs() {
        let graph = vec![vec![1], vec![0]];
        assert_eq!(Solution::eventual_safe_nodes_bfs(graph), vec![]);
    }

    #[test]
    fn test_chain_dfs() {
        // 0->1->2 (terminal) — all nodes eventually reach terminal
        let graph = vec![vec![1], vec![2], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![0, 1, 2]);
    }

    #[test]
    fn test_chain_bfs() {
        let graph = vec![vec![1], vec![2], vec![]];
        assert_eq!(Solution::eventual_safe_nodes_bfs(graph), vec![0, 1, 2]);
    }

    #[test]
    fn test_mixed_dfs() {
        // 0->1->2->0 is a cycle; 3->4 (terminal) is safe
        let graph = vec![vec![1], vec![2], vec![0], vec![4], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![3, 4]);
    }

    #[test]
    fn test_mixed_bfs() {
        let graph = vec![vec![1], vec![2], vec![0], vec![4], vec![]];
        assert_eq!(Solution::eventual_safe_nodes_bfs(graph), vec![3, 4]);
    }
}
