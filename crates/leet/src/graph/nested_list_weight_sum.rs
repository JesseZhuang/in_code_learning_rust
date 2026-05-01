/// leet 339

pub struct Solution;

/// Nested integer representation
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl Solution {
    /// DFS. O(n) time, O(d) space where d = max depth.
    pub fn depth_sum(nested_list: &[NestedInteger]) -> i32 {
        Self::dfs(nested_list, 1)
    }

    fn dfs(list: &[NestedInteger], depth: i32) -> i32 {
        let mut res = 0;
        for ni in list {
            match ni {
                NestedInteger::Int(val) => res += val * depth,
                NestedInteger::List(inner) => res += Self::dfs(inner, depth + 1),
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use NestedInteger::{Int, List};

    #[test]
    fn example1() {
        // [[1,1],2,[1,1]] -> 10
        let nl = vec![
            List(vec![Int(1), Int(1)]),
            Int(2),
            List(vec![Int(1), Int(1)]),
        ];
        assert_eq!(Solution::depth_sum(&nl), 10);
    }

    #[test]
    fn example2() {
        // [1,[4,[6]]] -> 27
        let nl = vec![
            Int(1),
            List(vec![Int(4), List(vec![Int(6)])]),
        ];
        assert_eq!(Solution::depth_sum(&nl), 27);
    }

    #[test]
    fn example3() {
        // [0] -> 0
        let nl = vec![Int(0)];
        assert_eq!(Solution::depth_sum(&nl), 0);
    }
}
