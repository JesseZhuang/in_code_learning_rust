use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    /// Uses a min-heap. O(n log n) time and O(n) space.
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut values: BinaryHeap<Reverse<i64>> = nums
            .into_iter()
            .map(|value| Reverse(i64::from(value)))
            .collect();
        let threshold = i64::from(k);
        let mut operations = 0;

        while values.peek().unwrap().0 < threshold {
            let Reverse(x) = values.pop().unwrap();
            let Reverse(y) = values.pop().unwrap();
            values.push(Reverse(2 * x + y));
            operations += 1;
        }

        operations
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct Case {
        nums: Vec<i32>,
        k: i32,
        expected: i32,
    }

    #[test]
    fn combines_values_until_the_threshold_is_exceeded() {
        let cases = vec![
            Case {
                nums: vec![2, 11, 10, 1, 3],
                k: 10,
                expected: 2,
            },
            Case {
                nums: vec![1, 1, 2, 4, 9],
                k: 20,
                expected: 4,
            },
        ];

        for case in cases {
            assert_eq!(Solution::min_operations(case.nums, case.k), case.expected);
        }
    }

    #[test]
    fn returns_zero_when_the_minimum_already_meets_the_threshold() {
        assert_eq!(Solution::min_operations(vec![10, 12], 10), 0);
    }

    #[test]
    fn accepts_a_combined_value_equal_to_the_threshold() {
        assert_eq!(Solution::min_operations(vec![1, 5], 7), 1);
    }

    #[test]
    fn uses_64_bit_arithmetic_for_intermediate_values() {
        assert_eq!(
            Solution::min_operations(vec![999_999_999, 1_000_000_000], 1_000_000_000),
            1
        );
    }
}
