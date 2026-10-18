use std::collections::{BinaryHeap, VecDeque};

pub struct Solution;

impl Solution {
    /// O(n) time, O(k) extra space; reuses the input for DP scores.
    pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut deque = VecDeque::new();
        deque.push_back(0usize);

        for index in 1..nums.len() {
            while deque.front().is_some_and(|&front| front + k < index) {
                deque.pop_front();
            }

            nums[index] += nums[*deque.front().unwrap()];
            while deque.back().is_some_and(|&back| nums[back] <= nums[index]) {
                deque.pop_back();
            }
            deque.push_back(index);
        }

        *nums.last().unwrap()
    }

    /// O(n log n) time, O(n) extra space with lazy heap expiry.
    pub fn max_result_heap(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        heap.push((nums[0], 0usize));
        let mut score = nums[0];

        for (index, &value) in nums.iter().enumerate().skip(1) {
            while heap
                .peek()
                .is_some_and(|&(_, previous)| previous + k < index)
            {
                heap.pop();
            }

            score = value + heap.peek().unwrap().0;
            heap.push((score, index));
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_both(nums: Vec<i32>, k: i32, expected: i32) {
        assert_eq!(Solution::max_result(nums.clone(), k), expected);
        assert_eq!(Solution::max_result_heap(nums, k), expected);
    }

    #[test]
    fn examples() {
        assert_both(vec![1, -1, -2, 4, -7, 3], 2, 7);
        assert_both(vec![10, -5, -2, 4, 0, 3], 3, 17);
        assert_both(vec![1, -5, -20, 4, -1, 3, -6, -3], 2, 0);
    }

    #[test]
    fn single_element() {
        assert_both(vec![-10_000], 1, -10_000);
    }

    #[test]
    fn all_negative() {
        assert_both(vec![-5, -1, -10, -2], 2, -8);
    }

    #[test]
    fn equal_scores_and_expiry() {
        assert_both(vec![1, 0, 0, -20, 5], 2, 6);
        assert_both(vec![100, -200, -200, 1], 2, -99);
    }

    #[test]
    fn jumps_can_reach_any_remaining_index() {
        assert_both(vec![1, -100, -100, 10], 100_000, 11);
    }

    #[test]
    fn large_signed_scores() {
        assert_both(vec![10_000; 100_000], 1, 1_000_000_000);
        assert_both(vec![-10_000; 100_000], 1, -1_000_000_000);
    }
}
