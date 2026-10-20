pub struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut right_sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        let mut differences = Vec::with_capacity(nums.len());

        for value in nums {
            right_sum -= value;
            differences.push((left_sum - right_sum).abs());
            left_sum += value;
        }

        differences
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample() {
        assert_eq!(
            Solution::left_right_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
    }

    #[test]
    fn balanced_sums() {
        assert_eq!(
            Solution::left_right_difference(vec![1, 2, 1]),
            vec![3, 0, 3]
        );
    }

    #[test]
    fn maximum_size_and_values() {
        let expected = (0..1000)
            .map(|index: i32| ((index * 2 - 999) * 100_000).abs())
            .collect::<Vec<_>>();

        assert_eq!(
            Solution::left_right_difference(vec![100_000; 1000]),
            expected
        );
    }
}
