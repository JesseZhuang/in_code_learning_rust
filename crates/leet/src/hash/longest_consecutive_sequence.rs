use std::collections::{HashMap, HashSet};

/// LeetCode 128 - Longest Consecutive Sequence
pub struct Solution;

impl Solution {
    /// HashSet approach: Only start counting from sequence starts (where num-1 not in set).
    /// Time: O(n) - each number visited at most twice (once for check, once as part of sequence)
    /// Space: O(n) - HashSet storage
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut max_len = 0;

        for &num in &num_set {
            // Only start counting if this is the beginning of a sequence
            if !num_set.contains(&(num - 1)) {
                let mut current = num;
                let mut current_len = 1;

                // Count consecutive numbers
                while num_set.contains(&(current + 1)) {
                    current += 1;
                    current_len += 1;
                }

                max_len = max_len.max(current_len);
            }
        }

        max_len
    }

    /// Union Find approach: Union adjacent numbers and find largest component.
    /// Time: O(n·α(n)) ≈ O(n) - α(n) is inverse Ackermann function (effectively constant)
    /// Space: O(n) - parent and size maps
    pub fn longest_consecutive_uf(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut uf = UnionFind::new();
        let num_set: HashSet<i32> = nums.iter().copied().collect();

        // Initialize each number as its own component
        for &num in &num_set {
            uf.make_set(num);
        }

        // Union adjacent numbers
        for &num in &num_set {
            // Check if num+1 exists and union them
            if num_set.contains(&(num + 1)) {
                uf.union(num, num + 1);
            }
        }

        uf.max_size()
    }
}

/// Union Find data structure with path compression and union by size
struct UnionFind {
    parent: HashMap<i32, i32>,
    size: HashMap<i32, i32>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            size: HashMap::new(),
        }
    }

    fn make_set(&mut self, x: i32) {
        self.parent.insert(x, x);
        self.size.insert(x, 1);
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.parent[&x] != x {
            // Path compression: make every node point directly to root
            let root = self.find(self.parent[&x]);
            self.parent.insert(x, root);
        }
        self.parent[&x]
    }

    fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            let size_x = self.size[&root_x];
            let size_y = self.size[&root_y];

            // Union by size: attach smaller tree under larger tree
            if size_x < size_y {
                self.parent.insert(root_x, root_y);
                self.size.insert(root_y, size_x + size_y);
            } else {
                self.parent.insert(root_y, root_x);
                self.size.insert(root_x, size_x + size_y);
            }
        }
    }

    fn max_size(&self) -> i32 {
        *self.size.values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::longest_consecutive_sequence::Solution;

    #[test]
    fn example1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums.clone()), 4);
        assert_eq!(Solution::longest_consecutive_uf(nums), 4);
    }

    #[test]
    fn example2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums.clone()), 9);
        assert_eq!(Solution::longest_consecutive_uf(nums), 9);
    }

    #[test]
    fn empty() {
        let nums = vec![];
        assert_eq!(Solution::longest_consecutive(nums.clone()), 0);
        assert_eq!(Solution::longest_consecutive_uf(nums), 0);
    }

    #[test]
    fn single() {
        let nums = vec![1];
        assert_eq!(Solution::longest_consecutive(nums.clone()), 1);
        assert_eq!(Solution::longest_consecutive_uf(nums), 1);
    }

    #[test]
    fn duplicates() {
        let nums = vec![1, 2, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums.clone()), 3);
        assert_eq!(Solution::longest_consecutive_uf(nums), 3);
    }

    #[test]
    fn negative() {
        let nums = vec![-1, -2, 0, 1, 2];
        assert_eq!(Solution::longest_consecutive(nums.clone()), 5);
        assert_eq!(Solution::longest_consecutive_uf(nums), 5);
    }

    #[test]
    fn no_consecutive() {
        let nums = vec![1, 3, 5, 7, 9];
        assert_eq!(Solution::longest_consecutive(nums.clone()), 1);
        assert_eq!(Solution::longest_consecutive_uf(nums), 1);
    }

    #[test]
    fn all_same() {
        let nums = vec![5, 5, 5, 5];
        assert_eq!(Solution::longest_consecutive(nums.clone()), 1);
        assert_eq!(Solution::longest_consecutive_uf(nums), 1);
    }

    #[test]
    fn two_sequences() {
        let nums = vec![1, 2, 3, 10, 11, 12, 13];
        assert_eq!(Solution::longest_consecutive(nums.clone()), 4);
        assert_eq!(Solution::longest_consecutive_uf(nums), 4);
    }
}
