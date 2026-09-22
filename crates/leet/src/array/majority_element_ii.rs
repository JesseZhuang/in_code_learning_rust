/// LeetCode 229 - Majority Element II

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Boyer-Moore Voting extended to n/3 — O(n) time, O(1) space
    ///
    /// At most 2 elements can appear more than ⌊n/3⌋ times.
    /// Track two candidates with counts; verify in a second pass.
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let (mut c1, mut c2) = (0, 1); // distinct initial values
        let (mut cnt1, mut cnt2) = (0i32, 0i32);

        for &n in &nums {
            if n == c1 {
                cnt1 += 1;
            } else if n == c2 {
                cnt2 += 1;
            } else if cnt1 == 0 {
                c1 = n;
                cnt1 = 1;
            } else if cnt2 == 0 {
                c2 = n;
                cnt2 = 1;
            } else {
                cnt1 -= 1;
                cnt2 -= 1;
            }
        }

        // Verify candidates
        let threshold = (nums.len() / 3) as i32;
        let mut result: Vec<i32> = [c1, c2]
            .iter()
            .copied()
            .filter(|&c| nums.iter().filter(|&&x| x == c).count() as i32 > threshold)
            .collect();
        result.sort();
        result.dedup();
        result
    }

    /// HashMap counting — O(n) time, O(n) space
    pub fn majority_element_hashmap(nums: Vec<i32>) -> Vec<i32> {
        let threshold = nums.len() / 3;
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for &n in &nums {
            *counts.entry(n).or_insert(0) += 1;
        }
        let mut result: Vec<i32> = counts
            .into_iter()
            .filter(|&(_, cnt)| cnt > threshold)
            .map(|(val, _)| val)
            .collect();
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_3_2_3() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
        assert_eq!(Solution::majority_element_hashmap(vec![3, 2, 3]), vec![3]);
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::majority_element(vec![1]), vec![1]);
        assert_eq!(Solution::majority_element_hashmap(vec![1]), vec![1]);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(Solution::majority_element(vec![1, 2]), vec![1, 2]);
        assert_eq!(Solution::majority_element_hashmap(vec![1, 2]), vec![1, 2]);
    }

    #[test]
    fn two_majority() {
        // n=7, threshold=2; 1 appears 3 times, 2 appears 3 times
        assert_eq!(
            Solution::majority_element(vec![1, 2, 1, 2, 1, 2, 3]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::majority_element_hashmap(vec![1, 2, 1, 2, 1, 2, 3]),
            vec![1, 2]
        );
    }

    #[test]
    fn no_majority() {
        // n=6, threshold=2; each appears exactly 2 times
        assert_eq!(
            Solution::majority_element(vec![1, 2, 3, 1, 2, 3]),
            vec![] as Vec<i32>
        );
        assert_eq!(
            Solution::majority_element_hashmap(vec![1, 2, 3, 1, 2, 3]),
            vec![] as Vec<i32>
        );
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::majority_element(vec![5, 5, 5, 5]), vec![5]);
        assert_eq!(Solution::majority_element_hashmap(vec![5, 5, 5, 5]), vec![5]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(
            Solution::majority_element(vec![-1, -1, -1, 2, 3]),
            vec![-1]
        );
        assert_eq!(
            Solution::majority_element_hashmap(vec![-1, -1, -1, 2, 3]),
            vec![-1]
        );
    }

    #[test]
    fn large_values() {
        assert_eq!(
            Solution::majority_element(vec![1_000_000_000, 1_000_000_000, -1_000_000_000]),
            vec![1_000_000_000]
        );
        assert_eq!(
            Solution::majority_element_hashmap(vec![1_000_000_000, 1_000_000_000, -1_000_000_000]),
            vec![1_000_000_000]
        );
    }

    #[test]
    fn three_elements_all_different() {
        // n=3, threshold=1; each appears once = exactly threshold, not more
        assert_eq!(
            Solution::majority_element(vec![1, 2, 3]),
            vec![] as Vec<i32>
        );
        assert_eq!(
            Solution::majority_element_hashmap(vec![1, 2, 3]),
            vec![] as Vec<i32>
        );
    }
}
