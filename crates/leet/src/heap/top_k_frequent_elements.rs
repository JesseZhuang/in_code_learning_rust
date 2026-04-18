// lc 347 - Top K Frequent Elements

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    /// Bucket sort approach.
    /// Time O(n), Space O(n)
    pub fn top_k_frequent_bucket(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut count: HashMap<i32, usize> = HashMap::new();
        for &num in &nums { // O(n)
            *count.entry(num).or_insert(0) += 1;
        }
        let mut buckets: Vec<Vec<i32>> = vec![vec![]; n + 1]; // index = frequency
        for (&num, &freq) in &count { // O(n) distribute
            buckets[freq].push(num);
        }
        let mut res = Vec::with_capacity(k as usize);
        for freq in (1..=n).rev() { // O(n) collect from highest freq
            for &num in &buckets[freq] {
                res.push(num);
                if res.len() == k as usize {
                    return res;
                }
            }
        }
        res
    }

    /// Min-heap of size k approach.
    /// Time O(n log k), Space O(n)
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut count: HashMap<i32, usize> = HashMap::new();
        for &num in &nums { // O(n)
            *count.entry(num).or_insert(0) += 1;
        }
        let mut heap: BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::new(); // min-heap by freq
        for (&num, &freq) in &count { // O(n log k)
            heap.push(Reverse((freq, num)));
            if heap.len() > k {
                heap.pop(); // evict least frequent
            }
        }
        heap.into_iter().map(|Reverse((_, num))| num).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket() {
        let mut res = Solution::top_k_frequent_bucket(vec![1, 1, 1, 2, 2, 3], 2);
        res.sort();
        assert_eq!(res, vec![1, 2]);

        let res = Solution::top_k_frequent_bucket(vec![1], 1);
        assert_eq!(res, vec![1]);

        let mut res = Solution::top_k_frequent_bucket(vec![-1, -1, -2, -2, -2, -3], 2);
        res.sort();
        assert_eq!(res, vec![-2, -1]);

        let mut res = Solution::top_k_frequent_bucket(vec![4, 4, 4, 1, 1, 2, 2, 2, 3], 3);
        res.sort();
        assert_eq!(res, vec![1, 2, 4]);

        let res = Solution::top_k_frequent_bucket(vec![5, 5, 5, 5], 1);
        assert_eq!(res, vec![5]);
    }

    #[test]
    fn test_heap() {
        let mut res = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        res.sort();
        assert_eq!(res, vec![1, 2]);

        let res = Solution::top_k_frequent(vec![1], 1);
        assert_eq!(res, vec![1]);

        let mut res = Solution::top_k_frequent(vec![-1, -1, -2, -2, -2, -3], 2);
        res.sort();
        assert_eq!(res, vec![-2, -1]);

        let mut res = Solution::top_k_frequent(vec![4, 4, 4, 1, 1, 2, 2, 2, 3], 3);
        res.sort();
        assert_eq!(res, vec![1, 2, 4]);

        let res = Solution::top_k_frequent(vec![5, 5, 5, 5], 1);
        assert_eq!(res, vec![5]);
    }
}
