use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let first: HashSet<i32> = nums1.into_iter().collect();
        let second: HashSet<i32> = nums2.into_iter().collect();

        vec![
            first.difference(&second).copied().collect(),
            second.difference(&first).copied().collect(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_difference(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        expected_first: Vec<i32>,
        expected_second: Vec<i32>,
    ) {
        let mut actual = Solution::find_difference(nums1, nums2);
        assert_eq!(actual.len(), 2);
        for values in &mut actual {
            values.sort_unstable();
        }
        assert_eq!(actual, vec![expected_first, expected_second]);
    }

    #[test]
    fn duplicates_appear_only_once() {
        assert_difference(vec![1, 2, 3, 3, 3], vec![1, 1, 2, 2], vec![3], vec![]);
    }

    #[test]
    fn identical_inputs_have_no_unique_values() {
        assert_difference(vec![1, 1, 2], vec![2, 1, 2], vec![], vec![]);
    }

    #[test]
    fn disjoint_inputs_return_both_sets() {
        assert_difference(vec![1, 2], vec![3, 4], vec![1, 2], vec![3, 4]);
    }

    #[test]
    fn negative_values_and_inclusive_bounds() {
        assert_difference(
            vec![-1000, 0, 1000],
            vec![-1000, -1, 1000],
            vec![0],
            vec![-1],
        );
    }

    #[test]
    fn asymmetric_unique_values() {
        assert_difference(vec![1, 2, 3, 3], vec![1, 4], vec![2, 3], vec![4]);
    }

    #[test]
    fn singleton_inputs() {
        assert_difference(vec![7], vec![7], vec![], vec![]);
        assert_difference(vec![-1000], vec![1000], vec![-1000], vec![1000]);
    }
}
