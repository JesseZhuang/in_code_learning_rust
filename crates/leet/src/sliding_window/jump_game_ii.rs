use std::cmp::max;

/// LeetCode 45, medium, tags: array, dp, greedy.

pub struct Solution;

impl Solution {
    /// Greedy. O(n) time, O(1) space.
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut res, mut reach, mut p) = (0i32, 0usize, 0usize);
        let mut i = 0usize;
        while p < nums.len() - 1 {
            reach = max(reach, i + nums[i] as usize);
            if i == p {
                res += 1;
                p = reach;
            }
            i += 1;
        }
        res
    }

    /// BFS level-order. O(n) time, O(1) space.
    pub fn jump_bfs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        let (mut level, mut cur_end, mut nxt_end) = (0i32, 0usize, 0usize);
        for i in 0..n - 1 {
            nxt_end = max(nxt_end, i + nums[i] as usize);
            if i == cur_end {
                level += 1;
                cur_end = nxt_end;
                if cur_end >= n - 1 {
                    break;
                }
            }
        }
        level
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(2, Solution::jump_bfs(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test_example2() {
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
        assert_eq!(2, Solution::jump_bfs(vec![2, 3, 0, 1, 4]));
    }

    #[test]
    fn test_single() {
        assert_eq!(0, Solution::jump(vec![0]));
        assert_eq!(0, Solution::jump_bfs(vec![0]));
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(1, Solution::jump(vec![1, 2]));
        assert_eq!(1, Solution::jump_bfs(vec![1, 2]));
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(4, Solution::jump(vec![1, 1, 1, 1, 1]));
        assert_eq!(4, Solution::jump_bfs(vec![1, 1, 1, 1, 1]));
    }

    #[test]
    fn test_large_first() {
        assert_eq!(1, Solution::jump(vec![10, 0, 0, 0, 0]));
        assert_eq!(1, Solution::jump_bfs(vec![10, 0, 0, 0, 0]));
    }

    #[test]
    fn test_zeros_middle() {
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 0, 4]));
        assert_eq!(2, Solution::jump_bfs(vec![2, 3, 0, 0, 4]));
    }

    #[test]
    fn test_greedy_choice() {
        assert_eq!(3, Solution::jump(vec![1, 2, 1, 1, 1]));
        assert_eq!(3, Solution::jump_bfs(vec![1, 2, 1, 1, 1]));
    }
}
