// lc 1202 — O(n·α(n) + n·log(n)) union-find; O(n·log(n) + E) DFS
use std::collections::HashMap;

pub struct Solution;

pub struct Solution2;

// Union-Find with rank + path compression
impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<usize> = vec![0; n];

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, a: usize, b: usize) {
            let mut ra = find(parent, a);
            let mut rb = find(parent, b);
            if ra == rb {
                return;
            }
            if rank[ra] < rank[rb] {
                std::mem::swap(&mut ra, &mut rb);
            }
            parent[rb] = ra;
            if rank[ra] == rank[rb] {
                rank[ra] += 1;
            }
        }

        for pair in &pairs {
            union(&mut parent, &mut rank, pair[0] as usize, pair[1] as usize);
        }

        let chars: Vec<char> = s.chars().collect();
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            groups.entry(root).or_default().push(i);
        }

        let mut result = vec![' '; n];
        for (_root, indices) in &groups {
            let mut sorted_chars: Vec<char> = indices.iter().map(|&i| chars[i]).collect();
            sorted_chars.sort();
            let mut sorted_indices = indices.clone();
            sorted_indices.sort();
            for (idx, &i) in sorted_indices.iter().enumerate() {
                result[i] = sorted_chars[idx];
            }
        }

        result.into_iter().collect()
    }
}

// DFS connected components
impl Solution2 {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for pair in &pairs {
            let (a, b) = (pair[0] as usize, pair[1] as usize);
            adj[a].push(b);
            adj[b].push(a);
        }

        let chars: Vec<char> = s.chars().collect();
        let mut visited = vec![false; n];
        let mut result = vec![' '; n];

        for i in 0..n {
            if visited[i] {
                continue;
            }
            let mut component = Vec::new();
            let mut stack = vec![i];
            while let Some(node) = stack.pop() {
                if visited[node] {
                    continue;
                }
                visited[node] = true;
                component.push(node);
                for &nbr in &adj[node] {
                    if !visited[nbr] {
                        stack.push(nbr);
                    }
                }
            }

            component.sort();
            let mut sorted_chars: Vec<char> = component.iter().map(|&idx| chars[idx]).collect();
            sorted_chars.sort();
            for (pos, &idx) in component.iter().enumerate() {
                result[idx] = sorted_chars[pos];
            }
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::smallest_string_with_swaps::{Solution, Solution2};

    fn assert_both(s: &str, pairs: Vec<Vec<i32>>, expected: &str) {
        let got1 = Solution::smallest_string_with_swaps(s.to_string(), pairs.clone());
        let got2 = Solution2::smallest_string_with_swaps(s.to_string(), pairs);
        assert_eq!(expected, got1, "Solution1 failed for input {:?}", s);
        assert_eq!(expected, got2, "Solution2 failed for input {:?}", s);
    }

    #[test]
    fn example1() {
        assert_both("dcab", vec![vec![0, 3], vec![1, 2]], "bacd");
    }

    #[test]
    fn example2() {
        assert_both("dcab", vec![vec![0, 3], vec![1, 2], vec![0, 2]], "abcd");
    }

    #[test]
    fn example3() {
        assert_both("cba", vec![vec![0, 1], vec![1, 2]], "abc");
    }

    #[test]
    fn single_char() {
        assert_both("a", vec![], "a");
    }

    #[test]
    fn no_pairs() {
        assert_both("dcba", vec![], "dcba");
    }

    #[test]
    fn full_chain() {
        assert_both(
            "edcba",
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
            "abcde",
        );
    }

    #[test]
    fn partial_components() {
        assert_both("dcbaf", vec![vec![0, 1], vec![2, 3]], "cdabf");
    }
}
