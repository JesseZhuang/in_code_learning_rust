use std::cmp::{max, min};

/// leet 152

impl Solution {
    /// Kadane's variant. O(n) time, O(1) space.
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max_here, mut min_here, mut res) = (1, 1, i32::MIN);
        for n in nums { // O(n)
            let (p1, p2) = (max_here * n, min_here * n); // compute before update
            max_here = max(n, max(p1, p2));
            min_here = min(n, min(p1, p2));
            res = max(res, max_here);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product(vec![-2]), -2);
        assert_eq!(Solution::max_product(vec![0, 2]), 2);
        assert_eq!(Solution::max_product(vec![-2, 3, -4]), 24);
        assert_eq!(Solution::max_product(vec![2, -5, -2, -4, 3]), 24);
        assert_eq!(Solution::max_product(vec![-1, -2, -3, 0]), 6);
        assert_eq!(Solution::max_product(vec![1, -2, 3, -4, -3, -2, 1]), 144);
    }
}
