use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    /// LeetCode 2542 - Maximum Subsequence Score
    /// Time: O(n log n), Space: O(n)
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = nums1.len();

        // Pair up and sort by nums2 descending
        let mut pairs: Vec<(i32, i32)> = (0..n).map(|i| (nums2[i], nums1[i])).collect();
        pairs.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        // Min-heap of size k on nums1 values
        let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
        let mut sum: i64 = 0;
        let mut ans: i64 = 0;

        for (min_val, n1) in pairs {
            let n1 = n1 as i64;
            sum += n1;
            heap.push(Reverse(n1));

            if heap.len() > k {
                if let Some(Reverse(smallest)) = heap.pop() {
                    sum -= smallest;
                }
            }

            if heap.len() == k {
                ans = ans.max(sum * min_val as i64);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3), 12);
    }

    #[test]
    fn test_k_one() {
        assert_eq!(Solution::max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1), 30);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(Solution::max_score(vec![1, 1, 1], vec![1, 1, 1], 2), 2);
    }

    #[test]
    fn test_all_selected() {
        assert_eq!(Solution::max_score(vec![2, 3, 5], vec![4, 2, 1], 3), 10);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_score(vec![5], vec![6], 1), 30);
    }

    #[test]
    fn test_k_two() {
        assert_eq!(Solution::max_score(vec![10, 20, 30], vec![1, 2, 3], 2), 100);
    }
}
