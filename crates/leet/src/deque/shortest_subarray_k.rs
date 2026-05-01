// LeetCode 862, hard, tags: array, queue, sliding window, monotonic queue, prefix sum.

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// O(n) time, O(n) space.
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut res = n + 1;
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }
        let k = k as i64;
        let mut dq: VecDeque<usize> = VecDeque::new();
        for i in 0..=n {
            while let Some(&front) = dq.front() {
                if prefix[i] - prefix[front] >= k {
                    res = res.min(i - dq.pop_front().unwrap());
                } else {
                    break;
                }
            }
            while let Some(&back) = dq.back() {
                if prefix[i] <= prefix[back] {
                    dq.pop_back();
                } else {
                    break;
                }
            }
            dq.push_back(i);
        }
        if res <= n { res as i32 } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(1, Solution::shortest_subarray(vec![1], 1));
    }

    #[test]
    fn test_example2() {
        assert_eq!(-1, Solution::shortest_subarray(vec![1, 2], 4));
    }

    #[test]
    fn test_example3() {
        assert_eq!(3, Solution::shortest_subarray(vec![2, -1, 2], 3));
    }
}
