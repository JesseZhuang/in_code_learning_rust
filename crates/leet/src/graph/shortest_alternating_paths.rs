use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// BFS over `(node, previous color)` states with a neutral start.
    /// Time: O(n + m), Space: O(n + m), where m is the number of red and blue edges.
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![Vec::new(); n], vec![Vec::new(); n]];
        for edge in red_edges {
            graph[0][edge[0] as usize].push(edge[1] as usize);
        }
        for edge in blue_edges {
            graph[1][edge[0] as usize].push(edge[1] as usize);
        }

        let mut distances = vec![-1; n];
        distances[0] = 0;

        let mut visited = vec![vec![false; n], vec![false; n]];
        visited[0][0] = true;
        visited[1][0] = true;

        let mut queue = VecDeque::new();
        queue.push_back((0usize, 2usize, 0i32));

        while let Some((node, previous_color, distance)) = queue.pop_front() {
            for color in 0..2 {
                if color == previous_color {
                    continue;
                }

                for &next in &graph[color][node] {
                    if visited[color][next] {
                        continue;
                    }
                    visited[color][next] = true;
                    if distances[next] == -1 {
                        distances[next] = distance + 1;
                    }
                    queue.push_back((next, color, distance + 1));
                }
            }
        }

        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![]),
            vec![0, 1, -1]
        );
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]]),
            vec![0, 1, -1]
        );
    }

    #[test]
    fn single_node_starts_at_zero() {
        assert_eq!(
            Solution::shortest_alternating_paths(1, vec![], vec![]),
            vec![0]
        );
    }

    #[test]
    fn alternating_continuation_reaches_each_next_node() {
        assert_eq!(
            Solution::shortest_alternating_paths(4, vec![vec![0, 1], vec![2, 3]], vec![vec![1, 2]]),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec![vec![1, 2]], vec![vec![0, 1]]),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn same_node_reached_with_different_previous_colors_matters() {
        assert_eq!(
            Solution::shortest_alternating_paths(
                4,
                vec![vec![0, 1], vec![1, 2]],
                vec![vec![0, 1], vec![1, 3]]
            ),
            vec![0, 1, 2, 2]
        );
    }

    #[test]
    fn handles_cycles_duplicates_and_unreachable_nodes() {
        assert_eq!(
            Solution::shortest_alternating_paths(
                6,
                vec![vec![0, 1], vec![0, 1], vec![2, 3], vec![3, 2]],
                vec![vec![1, 0], vec![1, 2], vec![1, 2], vec![3, 4]]
            ),
            vec![0, 1, 2, 3, 4, -1]
        );
    }
}
