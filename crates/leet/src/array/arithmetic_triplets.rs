use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // Expected O(n) time and O(n) space for constant-time membership checks.
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let values: HashSet<i32> = nums.iter().copied().collect();
        nums.into_iter()
            .filter(|&x| values.contains(&(x + diff)) && values.contains(&(x + 2 * diff)))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn official_example_one() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
    }

    #[test]
    fn official_example_two() {
        assert_eq!(Solution::arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }

    #[test]
    fn no_complete_triplets() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 4, 6], 3), 0);
    }

    #[test]
    fn overlapping_triplets() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 2, 3, 4], 1), 3);
    }

    #[test]
    fn minimum_size_with_triplet() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 100, 200], 100), 1);
    }

    #[test]
    fn minimum_size_without_triplet() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 3], 1), 0);
    }
}
