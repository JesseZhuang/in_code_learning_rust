pub struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut min_sum = nums[0];
        let mut cur_max = 0;
        let mut cur_min = 0;
        let mut total = 0;

        for &x in &nums {
            cur_max = x.max(cur_max + x);
            max_sum = max_sum.max(cur_max);
            cur_min = x.min(cur_min + x);
            min_sum = min_sum.min(cur_min);
            total += x;
        }

        if total == min_sum {
            max_sum
        } else {
            max_sum.max(total - min_sum)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_subarray_sum_circular() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
        assert_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
        assert_eq!(Solution::max_subarray_sum_circular(vec![7]), 7);
        assert_eq!(Solution::max_subarray_sum_circular(vec![-1]), -1);
        assert_eq!(Solution::max_subarray_sum_circular(vec![-5, -3, -4, -1, -2]), -1);
        assert_eq!(Solution::max_subarray_sum_circular(vec![3, 1, 2, 4]), 10);
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, -100, 2, 4]), 11);
        assert_eq!(Solution::max_subarray_sum_circular(vec![3, -1]), 3);
        assert_eq!(Solution::max_subarray_sum_circular(vec![8, -1, -1, 8, -7, 4]), 18);
    }
}
