use std::collections::VecDeque;

/// leet 210
pub struct Solution;

impl Solution {
    /// BFS topological sort (Kahn's algorithm).
    /// O(V+E) time, O(V+E) space
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
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

        let mut order = Vec::with_capacity(n);
        while let Some(node) = queue.pop_front() {
            order.push(node as i32);
            for &neighbor in &adj[node] {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        if order.len() == n { order } else { vec![] }
    }

    /// DFS reverse post-order with cycle detection.
    /// O(V+E) time, O(V+E) space
    pub fn find_order_dfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut adj = vec![vec![]; n];
        for edge in &prerequisites {
            adj[edge[1] as usize].push(edge[0] as usize);
        }

        // 0 = WHITE, 1 = GRAY, 2 = BLACK
        let mut color = vec![0u8; n];
        let mut order = Vec::with_capacity(n);

        for start in 0..n {
            if color[start] != 0 {
                continue;
            }
            let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
            color[start] = 1;

            while let Some((node, idx)) = stack.last_mut() {
                if *idx < adj[*node].len() {
                    let neighbor = adj[*node][*idx];
                    *idx += 1;
                    if color[neighbor] == 1 {
                        return vec![];
                    }
                    if color[neighbor] == 0 {
                        color[neighbor] = 1;
                        stack.push((neighbor, 0));
                    }
                } else {
                    let (finished, _) = stack.pop().unwrap();
                    color[finished] = 2;
                    order.push(finished as i32);
                }
            }
        }

        order.reverse();
        order
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashMap;

    fn is_valid_order(order: &[i32], n: i32, prerequisites: &[Vec<i32>]) -> bool {
        if order.len() != n as usize {
            return false;
        }
        let pos: HashMap<i32, usize> = order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        if pos.len() != n as usize {
            return false;
        }
        for edge in prerequisites {
            if pos[&edge[1]] >= pos[&edge[0]] {
                return false;
            }
        }
        true
    }

    fn check(n: i32, prereqs: &[[i32; 2]], has_order: bool) {
        let edges: Vec<Vec<i32>> = prereqs.iter().map(|e| e.to_vec()).collect();
        let result_bfs = Solution::find_order(n, edges.clone());
        let result_dfs = Solution::find_order_dfs(n, edges.clone());

        if has_order {
            assert!(is_valid_order(&result_bfs, n, &edges), "BFS: {:?}", result_bfs);
            assert!(is_valid_order(&result_dfs, n, &edges), "DFS: {:?}", result_dfs);
        } else {
            assert!(result_bfs.is_empty());
            assert!(result_dfs.is_empty());
        }
    }

    #[test]
    fn example1() {
        check(2, &[[1, 0]], true);
    }

    #[test]
    fn example2() {
        check(4, &[[1, 0], [2, 0], [3, 1], [3, 2]], true);
    }

    #[test]
    fn cycle() {
        check(2, &[[1, 0], [0, 1]], false);
    }

    #[test]
    fn no_prerequisites() {
        check(3, &[], true);
    }

    #[test]
    fn single_course() {
        check(1, &[], true);
    }

    #[test]
    fn chain() {
        check(4, &[[1, 0], [2, 1], [3, 2]], true);
    }

    #[test]
    fn cycle_in_chain() {
        check(4, &[[1, 0], [2, 1], [3, 2], [0, 3]], false);
    }

    #[test]
    fn disconnected() {
        check(4, &[[1, 0], [3, 2]], true);
    }

    #[test]
    fn diamond() {
        check(4, &[[1, 0], [2, 0], [3, 1], [3, 2]], true);
    }

    #[test]
    fn all_depend_on_one() {
        check(5, &[[1, 0], [2, 0], [3, 0], [4, 0]], true);
    }

    #[test]
    fn large_chain() {
        let n = 2000;
        let prereqs: Vec<[i32; 2]> = (1..n).map(|i| [i, i - 1]).collect();
        check(n, &prereqs, true);
    }
}
