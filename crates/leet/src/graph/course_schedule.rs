use std::collections::VecDeque;

/// leet 207
pub struct Solution;

impl Solution {
    /// Iterative DFS cycle detection using color array (WHITE/GRAY/BLACK).
    /// O(V+E) time, O(V+E) space
    pub fn can_finish_dfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut adj = vec![vec![]; n];
        for edge in &prerequisites {
            adj[edge[1] as usize].push(edge[0] as usize);
        }

        // 0 = WHITE, 1 = GRAY, 2 = BLACK
        let mut color = vec![0u8; n];

        for start in 0..n {
            if color[start] != 0 {
                continue;
            }
            // Iterative DFS using explicit stack
            let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
            color[start] = 1; // GRAY

            while let Some((node, idx)) = stack.last_mut() {
                if *idx < adj[*node].len() {
                    let neighbor = adj[*node][*idx];
                    *idx += 1;
                    if color[neighbor] == 1 {
                        return false; // back edge -> cycle
                    }
                    if color[neighbor] == 0 {
                        color[neighbor] = 1;
                        stack.push((neighbor, 0));
                    }
                } else {
                    let (finished, _) = stack.pop().unwrap();
                    color[finished] = 2; // BLACK
                }
            }
        }
        true
    }

    /// BFS topological sort (Kahn's algorithm) with in-degree array.
    /// O(V+E) time, O(V+E) space
    pub fn can_finish_bfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut adj = vec![vec![]; n];
        let mut in_degree = vec![0u32; n];

        for edge in &prerequisites {
            adj[edge[1] as usize].push(edge[0] as usize);
            in_degree[edge[0] as usize] += 1;
        }

        let mut queue = VecDeque::new();
        for i in 0..n {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut processed = 0usize;
        while let Some(node) = queue.pop_front() {
            processed += 1;
            for &neighbor in &adj[node] {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        processed == n
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check(num: i32, prereqs: &[[i32; 2]], expected: bool) {
        let edges: Vec<Vec<i32>> = prereqs.iter().map(|e| e.to_vec()).collect();
        assert_eq!(Solution::can_finish_dfs(num, edges.clone()), expected);
        assert_eq!(Solution::can_finish_bfs(num, edges), expected);
    }

    #[test]
    fn example1_no_cycle() {
        // 2 courses, [1,0] means 0->1
        check(2, &[[1, 0]], true);
    }

    #[test]
    fn example2_cycle() {
        // 2 courses, 0->1 and 1->0
        check(2, &[[1, 0], [0, 1]], false);
    }

    #[test]
    fn no_prerequisites() {
        check(5, &[], true);
    }

    #[test]
    fn single_course() {
        check(1, &[], true);
    }

    #[test]
    fn chain_no_cycle() {
        // 0->1->2->3
        check(4, &[[1, 0], [2, 1], [3, 2]], true);
    }

    #[test]
    fn cycle_in_chain() {
        // 0->1->2->3->1
        check(4, &[[1, 0], [2, 1], [3, 2], [1, 3]], false);
    }

    #[test]
    fn disconnected_components() {
        // Two separate chains: 0->1, 2->3
        check(4, &[[1, 0], [3, 2]], true);
    }

    #[test]
    fn diamond_dag() {
        // 0->1, 0->2, 1->3, 2->3
        check(4, &[[1, 0], [2, 0], [3, 1], [3, 2]], true);
    }

    #[test]
    fn self_loop() {
        check(2, &[[0, 0]], false);
    }

    #[test]
    fn all_depend_on_one() {
        // 0 is prerequisite for 1,2,3,4
        check(5, &[[1, 0], [2, 0], [3, 0], [4, 0]], true);
    }

    #[test]
    fn multiple_components_one_cycle() {
        // Component 1: 0->1 (no cycle)
        // Component 2: 2->3->4->2 (cycle)
        check(5, &[[1, 0], [3, 2], [4, 3], [2, 4]], false);
    }
}
