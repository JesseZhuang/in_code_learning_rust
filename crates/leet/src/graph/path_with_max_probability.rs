use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    /// Solution 1: Modified Dijkstra with max-heap.
    /// O((V+E) log V) time, O(V+E) space.
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let n = n as usize;
        let start = start_node as usize;
        let end = end_node as usize;

        // Build adjacency list — O(E)
        let mut graph: Vec<Vec<(usize, f64)>> = vec![vec![]; n];
        for (i, edge) in edges.iter().enumerate() {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push((v, succ_prob[i]));
            graph[v].push((u, succ_prob[i]));
        }

        // dist[i] = max probability to reach node i from start
        let mut dist = vec![0.0_f64; n];
        dist[start] = 1.0;

        // Max-heap storing (probability_bits_as_u64, node)
        // We use u64 bit representation for f64 ordering since f64 doesn't impl Ord.
        let mut heap = BinaryHeap::new();
        heap.push((FloatOrd(1.0), start));

        // Dijkstra relaxation — O((V+E) log V)
        while let Some((FloatOrd(prob), u)) = heap.pop() {
            if u == end {
                return prob;
            }
            if prob < dist[u] {
                continue; // stale entry
            }
            // Relax neighbors — each edge processed at most twice
            for &(v, edge_prob) in &graph[u] {
                let new_prob = prob * edge_prob;
                if new_prob > dist[v] {
                    dist[v] = new_prob;
                    heap.push((FloatOrd(new_prob), v));
                }
            }
        }

        0.0
    }

    /// Solution 2: Bellman-Ford relaxation.
    /// O(V * E) time, O(V) space.
    pub fn max_probability2(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let n = n as usize;
        let start = start_node as usize;
        let end = end_node as usize;

        let mut dist = vec![0.0_f64; n];
        dist[start] = 1.0;

        // Relax all edges up to V-1 times — O(V * E)
        for _ in 0..n - 1 {
            let mut updated = false;
            // Each iteration relaxes every edge — O(E)
            for (i, edge) in edges.iter().enumerate() {
                let (u, v) = (edge[0] as usize, edge[1] as usize);
                let p = succ_prob[i];
                if dist[u] * p > dist[v] {
                    dist[v] = dist[u] * p;
                    updated = true;
                }
                if dist[v] * p > dist[u] {
                    dist[u] = dist[v] * p;
                    updated = true;
                }
            }
            if !updated {
                break; // early termination
            }
        }

        dist[end]
    }
}

/// Wrapper for f64 to implement Ord for use in BinaryHeap (max-heap).
#[derive(PartialEq, PartialOrd)]
struct FloatOrd(f64);

impl Eq for FloatOrd {}

impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_both(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
        expected: f64,
    ) {
        let eps = 1e-5;
        let r1 = Solution::max_probability(n, edges.clone(), succ_prob.clone(), start, end);
        let r2 = Solution::max_probability2(n, edges, succ_prob, start, end);
        assert!(
            (r1 - expected).abs() < eps,
            "Dijkstra: got {r1}, expected {expected}"
        );
        assert!(
            (r2 - expected).abs() < eps,
            "Bellman-Ford: got {r2}, expected {expected}"
        );
    }

    #[test]
    fn test_lc_example1() {
        // 0 --0.5-- 1 --0.5-- 2, 0 --0.2-- 2
        // Best: 0->1->2 = 0.25
        run_both(
            3,
            vec![vec![0, 1], vec![1, 2], vec![0, 2]],
            vec![0.5, 0.5, 0.2],
            0,
            2,
            0.25,
        );
    }

    #[test]
    fn test_lc_example2() {
        // 0 --0.5-- 1 --0.5-- 2, 0 --0.3-- 2
        // Best: 0->2 = 0.3
        run_both(
            3,
            vec![vec![0, 1], vec![1, 2], vec![0, 2]],
            vec![0.5, 0.5, 0.3],
            0,
            2,
            0.3,
        );
    }

    #[test]
    fn test_no_path() {
        // Node 2 is disconnected
        run_both(3, vec![vec![0, 1]], vec![0.5], 0, 2, 0.0);
    }

    #[test]
    fn test_single_node() {
        // start == end
        run_both(1, vec![], vec![], 0, 0, 1.0);
    }

    #[test]
    fn test_direct_edge() {
        run_both(2, vec![vec![0, 1]], vec![0.8], 0, 1, 0.8);
    }

    #[test]
    fn test_longer_path_better() {
        // Direct 0->2 = 0.1, but 0->1->2 = 0.9*0.9 = 0.81
        run_both(
            3,
            vec![vec![0, 1], vec![1, 2], vec![0, 2]],
            vec![0.9, 0.9, 0.1],
            0,
            2,
            0.81,
        );
    }

    #[test]
    fn test_disconnected_components() {
        // Two components: {0,1} and {2,3}
        run_both(
            4,
            vec![vec![0, 1], vec![2, 3]],
            vec![0.5, 0.7],
            0,
            3,
            0.0,
        );
    }
}
