pub struct Solution;

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for index in 1..nums.len() {
            nums[index] += nums[index - 1];
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn singleton_stays_unchanged() {
        assert_eq!(Solution::running_sum(vec![5]), vec![5]);
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(
            Solution::running_sum(vec![1, 1, 1, 1, 1]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(
            Solution::running_sum(vec![3, 1, 2, 10, 1]),
            vec![3, 4, 6, 16, 17]
        );
    }

    #[test]
    fn all_zeros_remain_zero() {
        assert_eq!(Solution::running_sum(vec![0, 0, 0]), vec![0, 0, 0]);
    }

    #[test]
    fn negative_values_accumulate() {
        assert_eq!(Solution::running_sum(vec![-1, -2, -3]), vec![-1, -3, -6]);
    }

    #[test]
    fn mixed_signs_accumulate_in_order() {
        assert_eq!(
            Solution::running_sum(vec![4, -6, 0, 3, -1]),
            vec![4, -2, -2, 1, 0]
        );
    }

    #[test]
    fn maximum_magnitude_values_cancel() {
        assert_eq!(
            Solution::running_sum(vec![1_000_000, -1_000_000, 1_000_000, -1_000_000]),
            vec![1_000_000, 0, 1_000_000, 0]
        );
    }

    #[test]
    fn maximum_length_and_magnitude_fit_in_i32() {
        let expected: Vec<i32> = (1..=1_000).map(|count| count * 1_000_000).collect();
        assert_eq!(Solution::running_sum(vec![1_000_000; 1_000]), expected);
    }
}
