// LeetCode 136

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// O(n) time and O(1) extra space. Equal values cancel under XOR.
    pub fn single_number_xor(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |single, num| single ^ num)
    }

    /// Expected O(n) time and O(n) extra space.
    pub fn single_number_hash_set(nums: Vec<i32>) -> i32 {
        let mut unpaired = HashSet::new();
        for num in nums {
            if !unpaired.insert(num) {
                unpaired.remove(&num);
            }
        }
        unpaired.into_iter().next().expect("one unpaired number")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    const CASES: &[(&[i32], i32)] = &[
        (&[2, 2, 1], 1),
        (&[4, 1, 2, 1, 2], 4),
        (&[1], 1),
        (&[-2, -3, -2], -3),
        (&[3, 0, 3], 0),
        (&[0, 5, -7, 5, 0], -7),
    ];

    #[test]
    fn xor_returns_the_single_number() {
        for &(nums, expected) in CASES {
            assert_eq!(
                Solution::single_number_xor(nums.to_vec()),
                expected,
                "{nums:?}"
            );
        }
    }

    #[test]
    fn hash_set_returns_the_single_number() {
        for &(nums, expected) in CASES {
            assert_eq!(
                Solution::single_number_hash_set(nums.to_vec()),
                expected,
                "{nums:?}"
            );
        }
    }
}
