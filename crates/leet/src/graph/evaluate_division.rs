use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    /// BFS on weighted directed graph.
    /// Time: O(Q * (V + E)), Space: O(V + E)
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        // Build adjacency list: a -> [(b, val), ...], b -> [(a, 1/val), ...]
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        for (eq, &val) in equations.iter().zip(values.iter()) {
            let a = &eq[0];
            let b = &eq[1];
            graph.entry(a.clone()).or_default().push((b.clone(), val));
            graph.entry(b.clone()).or_default().push((a.clone(), 1.0 / val));
        }

        let mut results = Vec::with_capacity(queries.len());
        for q in &queries {
            let src = &q[0];
            let dst = &q[1];
            if !graph.contains_key(src) || !graph.contains_key(dst) {
                results.push(-1.0);
                continue;
            }
            if src == dst {
                results.push(1.0);
                continue;
            }
            // BFS
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            queue.push_back((src.clone(), 1.0));
            visited.insert(src.clone());
            let mut found = -1.0;
            while let Some((node, acc)) = queue.pop_front() {
                if &node == dst {
                    found = acc;
                    break;
                }
                if let Some(neighbors) = graph.get(&node) {
                    for (next, weight) in neighbors {
                        if visited.insert(next.clone()) {
                            queue.push_back((next.clone(), acc * weight));
                        }
                    }
                }
            }
            results.push(found);
        }
        results
    }

    /// Union-Find with weighted edges.
    /// weight[x] = x / root(x). Query a/b = weight[a] / weight[b] if same root.
    /// Time: O((E + Q) * alpha(n)), Space: O(V)
    pub fn calc_equation_union_find(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut parent: HashMap<String, String> = HashMap::new();
        let mut weight: HashMap<String, f64> = HashMap::new(); // weight[x] = x / parent[x]
        let mut rank: HashMap<String, u32> = HashMap::new();

        // Find with path compression. Returns root and updates weight[x] = x / root.
        fn find(
            x: &str,
            parent: &mut HashMap<String, String>,
            weight: &mut HashMap<String, f64>,
        ) -> String {
            if parent[x] != x {
                let p = parent[x].clone();
                let root = find(&p, parent, weight);
                // weight[x] = x/parent * parent/root = x/root
                let w = weight[x] * weight[&p];
                weight.insert(x.to_string(), w);
                parent.insert(x.to_string(), root.clone());
                root
            } else {
                x.to_string()
            }
        }

        // Initialize
        for eq in &equations {
            for node in eq {
                if !parent.contains_key(node) {
                    parent.insert(node.clone(), node.clone());
                    weight.insert(node.clone(), 1.0);
                    rank.insert(node.clone(), 0);
                }
            }
        }

        // Union: given a/b = val
        for (eq, &val) in equations.iter().zip(values.iter()) {
            let a = &eq[0];
            let b = &eq[1];
            let ra = find(a, &mut parent, &mut weight);
            let rb = find(b, &mut parent, &mut weight);
            if ra == rb {
                continue;
            }
            // After find: weight[a] = a/ra, weight[b] = b/rb
            // We know a/b = val, so ra/rb = val * weight[b] / weight[a]
            let w = val * weight[b] / weight[a];
            let rank_a = rank[&ra];
            let rank_b = rank[&rb];
            if rank_a < rank_b {
                // Attach ra under rb: parent[ra] = rb, weight[ra] = ra/rb = w
                parent.insert(ra.clone(), rb.clone());
                weight.insert(ra.clone(), w);
            } else if rank_a > rank_b {
                // Attach rb under ra: parent[rb] = ra, weight[rb] = rb/ra = 1/w
                parent.insert(rb.clone(), ra.clone());
                weight.insert(rb.clone(), 1.0 / w);
            } else {
                parent.insert(rb.clone(), ra.clone());
                weight.insert(rb.clone(), 1.0 / w);
                *rank.get_mut(&ra).unwrap() += 1;
            }
        }

        // Query
        let mut results = Vec::with_capacity(queries.len());
        for q in &queries {
            let a = &q[0];
            let b = &q[1];
            if !parent.contains_key(a) || !parent.contains_key(b) {
                results.push(-1.0);
                continue;
            }
            let ra = find(a, &mut parent, &mut weight);
            let rb = find(b, &mut parent, &mut weight);
            if ra != rb {
                results.push(-1.0);
            } else {
                // a/b = (a/root) / (b/root) = weight[a] / weight[b]
                results.push(weight[a] / weight[b]);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&[&str]]) -> Vec<Vec<String>> {
        v.iter()
            .map(|pair| pair.iter().map(|s| s.to_string()).collect())
            .collect()
    }

    fn assert_close(result: &[f64], expected: &[f64]) {
        assert_eq!(result.len(), expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert!(
                (r - e).abs() < 1e-5,
                "expected {}, got {}",
                e,
                r
            );
        }
    }

    #[test]
    fn example1() {
        let equations = s(&[&["a", "b"], &["b", "c"]]);
        let values = vec![2.0, 3.0];
        let queries = s(&[
            &["a", "c"],
            &["b", "a"],
            &["a", "e"],
            &["a", "a"],
            &["x", "x"],
        ]);
        let expected = vec![6.0, 0.5, -1.0, 1.0, -1.0];
        assert_close(&Solution::calc_equation(equations.clone(), values.clone(), queries.clone()), &expected);
        assert_close(&Solution::calc_equation_union_find(equations, values, queries), &expected);
    }

    #[test]
    fn example2() {
        let equations = s(&[&["a", "b"], &["b", "c"], &["bc", "cd"]]);
        let values = vec![1.5, 2.5, 5.0];
        let queries = s(&[
            &["a", "c"],
            &["c", "b"],
            &["bc", "cd"],
            &["cd", "bc"],
        ]);
        let expected = vec![3.75, 0.4, 5.0, 0.2];
        assert_close(&Solution::calc_equation(equations.clone(), values.clone(), queries.clone()), &expected);
        assert_close(&Solution::calc_equation_union_find(equations, values, queries), &expected);
    }

    #[test]
    fn disconnected() {
        let equations = s(&[&["a", "b"], &["c", "d"]]);
        let values = vec![2.0, 3.0];
        let queries = s(&[&["a", "d"], &["c", "b"]]);
        let expected = vec![-1.0, -1.0];
        assert_close(&Solution::calc_equation(equations.clone(), values.clone(), queries.clone()), &expected);
        assert_close(&Solution::calc_equation_union_find(equations, values, queries), &expected);
    }

    #[test]
    fn chain() {
        let equations = s(&[&["a", "b"], &["b", "c"], &["c", "d"], &["d", "e"]]);
        let values = vec![2.0, 3.0, 4.0, 5.0];
        let queries = s(&[&["a", "e"], &["e", "a"]]);
        let expected = vec![120.0, 1.0 / 120.0];
        assert_close(&Solution::calc_equation(equations.clone(), values.clone(), queries.clone()), &expected);
        assert_close(&Solution::calc_equation_union_find(equations, values, queries), &expected);
    }
}
