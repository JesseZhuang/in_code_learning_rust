use std::collections::VecDeque;

/// leet 2101
pub struct Solution;

impl Solution {
    /// BFS approach: build directed graph, try each bomb as start, return max reachable.
    /// Time O(n^3), Space O(n^2)
    pub fn maximum_detonation_bfs(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let adj = Self::build_graph(&bombs);

        let mut max_reach = 0;
        for start in 0..n {
            let mut visited = vec![false; n];
            visited[start] = true;
            let mut queue = VecDeque::new();
            queue.push_back(start);
            let mut count = 1;

            while let Some(node) = queue.pop_front() {
                for &nei in &adj[node] {
                    if !visited[nei] {
                        visited[nei] = true;
                        count += 1;
                        queue.push_back(nei);
                    }
                }
            }
            max_reach = max_reach.max(count);
            if max_reach == n as i32 {
                return max_reach;
            }
        }
        max_reach
    }

    /// DFS approach: build directed graph, try each bomb as start, return max reachable.
    /// Time O(n^3), Space O(n^2)
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let adj = Self::build_graph(&bombs);

        let mut max_reach = 0;
        for start in 0..n {
            let mut visited = vec![false; n];
            visited[start] = true;
            let mut stack = vec![start];
            let mut count = 1;

            while let Some(node) = stack.pop() {
                for &nei in &adj[node] {
                    if !visited[nei] {
                        visited[nei] = true;
                        count += 1;
                        stack.push(nei);
                    }
                }
            }
            max_reach = max_reach.max(count);
            if max_reach == n as i32 {
                return max_reach;
            }
        }
        max_reach
    }

    fn build_graph(bombs: &[Vec<i32>]) -> Vec<Vec<usize>> {
        let n = bombs.len();
        let mut adj = vec![vec![]; n];
        for i in 0..n {
            let (xi, yi, ri) = (bombs[i][0] as i64, bombs[i][1] as i64, bombs[i][2] as i64);
            for j in 0..n {
                if i == j {
                    continue;
                }
                let dx = xi - bombs[j][0] as i64;
                let dy = yi - bombs[j][1] as i64;
                if dx * dx + dy * dy <= ri * ri {
                    adj[i].push(j);
                }
            }
        }
        adj
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check(bombs: &[[i32; 3]], expected: i32) {
        let input: Vec<Vec<i32>> = bombs.iter().map(|b| b.to_vec()).collect();
        assert_eq!(Solution::maximum_detonation_bfs(input.clone()), expected);
        assert_eq!(Solution::maximum_detonation(input), expected);
    }

    #[test]
    fn example1() {
        check(&[[2, 1, 3], [6, 1, 4]], 2);
    }

    #[test]
    fn example2() {
        check(&[[1, 1, 5], [10, 10, 5]], 1);
    }

    #[test]
    fn example3() {
        check(&[[1, 2, 3], [2, 3, 1], [3, 4, 2], [4, 5, 3], [5, 6, 4]], 5);
    }

    #[test]
    fn single() {
        check(&[[0, 0, 1]], 1);
    }

    #[test]
    fn no_chain() {
        check(&[[0, 0, 1], [100, 100, 1], [200, 200, 1]], 1);
    }

    #[test]
    fn one_directional() {
        // bomb 0 can reach bomb 1, but bomb 1 cannot reach bomb 0
        check(&[[0, 0, 5], [4, 0, 1]], 2);
    }

    #[test]
    fn exact_boundary() {
        // distance = 5, radius = 5 -> exactly on boundary, should detonate
        check(&[[0, 0, 5], [3, 4, 1]], 2);
    }

    #[test]
    fn chain() {
        // 0 -> 1 -> 2 (each can reach the next but not further)
        check(&[[0, 0, 2], [1, 0, 2], [2, 0, 2]], 3);
    }
}
