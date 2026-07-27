// lc 1584 — O(n²·log n) Prim's; O(n²·log n) Kruskal's

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;
pub struct Solution2;

impl Solution {
    /// Prim's algorithm using a min-heap (BinaryHeap with Reverse).
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 1 {
            return 0;
        }

        let mut in_mst = vec![false; n];
        // heap entries: (cost, node_index)
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i32, 0usize))); // start from node 0
        let mut total_cost = 0;
        let mut edges_added = 0;

        // O(n²·log n): each of n nodes can push up to n-1 edges into the heap
        while let Some(Reverse((cost, u))) = heap.pop() {
            if in_mst[u] {
                continue;
            }
            in_mst[u] = true;
            total_cost += cost;
            edges_added += 1;
            if edges_added == n {
                break;
            }

            // O(n) per node: push all edges from u to nodes not yet in MST
            for v in 0..n {
                if !in_mst[v] {
                    let dist = (points[u][0] - points[v][0]).abs()
                        + (points[u][1] - points[v][1]).abs();
                    heap.push(Reverse((dist, v)));
                }
            }
        }

        total_cost
    }
}

impl Solution2 {
    /// Kruskal's algorithm with union-find (path halving + union by rank).
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 1 {
            return 0;
        }

        // O(n²) edges total for a complete graph
        let mut edges: Vec<(i32, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in (i + 1)..n {
                let dist = (points[i][0] - points[j][0]).abs()
                    + (points[i][1] - points[j][1]).abs();
                edges.push((dist, i, j));
            }
        }

        // O(n²·log n) sort
        edges.sort_unstable();

        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank = vec![0u8; n];

        // Find with path halving — O(α(n)) amortized
        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]]; // path halving
                x = parent[x];
            }
            x
        }

        // Union by rank — O(α(n)) amortized
        fn union(parent: &mut [usize], rank: &mut [u8], x: usize, y: usize) -> bool {
            let rx = find(parent, x);
            let ry = find(parent, y);
            if rx == ry {
                return false;
            }
            if rank[rx] < rank[ry] {
                parent[rx] = ry;
            } else if rank[rx] > rank[ry] {
                parent[ry] = rx;
            } else {
                parent[ry] = rx;
                rank[rx] += 1;
            }
            true
        }

        let mut total_cost = 0;
        let mut edges_used = 0;

        // Process edges in ascending order; stop after n-1 edges
        for (cost, u, v) in edges {
            if union(&mut parent, &mut rank, u, v) {
                total_cost += cost;
                edges_used += 1;
                if edges_used == n - 1 {
                    break;
                }
            }
        }

        total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let points = vec![
            vec![0, 0],
            vec![2, 2],
            vec![3, 10],
            vec![5, 2],
            vec![7, 0],
        ];
        assert_eq!(Solution::min_cost_connect_points(points.clone()), 20);
        assert_eq!(Solution2::min_cost_connect_points(points), 20);
    }

    #[test]
    fn test_example2() {
        let points = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];
        assert_eq!(Solution::min_cost_connect_points(points.clone()), 18);
        assert_eq!(Solution2::min_cost_connect_points(points), 18);
    }

    #[test]
    fn test_single_point() {
        let points = vec![vec![0, 0]];
        assert_eq!(Solution::min_cost_connect_points(points.clone()), 0);
        assert_eq!(Solution2::min_cost_connect_points(points), 0);
    }

    #[test]
    fn test_two_points() {
        let points = vec![vec![0, 0], vec![1, 1]];
        assert_eq!(Solution::min_cost_connect_points(points.clone()), 2);
        assert_eq!(Solution2::min_cost_connect_points(points), 2);
    }

    #[test]
    fn test_collinear() {
        let points = vec![vec![0, 0], vec![1, 0], vec![3, 0]];
        assert_eq!(Solution::min_cost_connect_points(points.clone()), 3);
        assert_eq!(Solution2::min_cost_connect_points(points), 3);
    }
}
