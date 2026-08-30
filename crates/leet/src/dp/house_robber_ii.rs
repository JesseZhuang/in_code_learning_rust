/// leet 213

pub struct Solution;

impl Solution {
    /// Split into two linear sub-problems (exclude last, exclude first).
    /// O(n) time, O(1) space.
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        if n == 1 { return nums[0]; }
        Self::rob_linear(&nums[..n - 1]).max(Self::rob_linear(&nums[1..]))
    }

    /// Linear house robber on a slice. O(n) time, O(1) space.
    fn rob_linear(nums: &[i32]) -> i32 {
        let (mut rob_prev, mut n_rob_prev) = (0, 0);
        for &n in nums {
            let rob_cur = n_rob_prev + n;
            n_rob_prev = n_rob_prev.max(rob_prev);
            rob_prev = rob_cur;
        }
        rob_prev.max(n_rob_prev)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
        assert_eq!(Solution::rob(vec![5]), 5);
        assert_eq!(Solution::rob(vec![1, 2]), 2);
        assert_eq!(Solution::rob(vec![3, 3]), 3);
        assert_eq!(Solution::rob(vec![3, 3, 3, 3]), 6);
        assert_eq!(Solution::rob(vec![1000, 1, 1000, 1]), 2000);
        assert_eq!(Solution::rob(vec![0, 0, 0, 0]), 0);
        assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5]), 8);
    }
}
