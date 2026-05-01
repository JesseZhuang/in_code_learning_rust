// LeetCode 3254, medium, tags: array, sliding window.

pub struct Solution;

impl Solution {
    /// O(n) time, O(1) space.
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k == 1 {
            return nums;
        }
        let n = nums.len();
        let mut res = vec![-1i32; n - k + 1];
        let mut streak = 1usize;
        for i in 0..n - 1 {
            if nums[i] + 1 == nums[i + 1] {
                streak += 1;
            } else {
                streak = 1;
            }
            if streak >= k {
                res[i + 2 - k] = nums[i + 1];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![3, 4, -1, -1, -1],
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3)
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![-1, -1],
            Solution::results_array(vec![2, 2, 2, 2, 2], 4)
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            vec![-1, 3, -1, 3, -1],
            Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2)
        );
    }
}
