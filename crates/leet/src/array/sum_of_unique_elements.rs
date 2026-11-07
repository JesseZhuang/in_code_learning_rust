pub struct Solution;

impl Solution {
    /// Counts values in the bounded range 1..=100, then sums those seen once.
    /// Time: O(n + U), where U = 101; space: O(U) for the frequency array.
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut frequency = [0; 101];
        for value in nums {
            frequency[value as usize] += 1;
        }

        frequency
            .iter()
            .enumerate()
            .filter(|&(_, &count)| count == 1)
            .map(|(value, _)| value as i32)
            .sum()
    }

    /// Sorts and scans equal-value groups, operating on the owned Vec in place.
    /// Time: O(n log n); auxiliary space: O(log n) for the sorting stack.
    pub fn sum_of_unique_by_sorting(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut sum = 0;
        let mut index = 0;
        while index < nums.len() {
            let value = nums[index];
            let mut next = index + 1;
            while next < nums.len() && nums[next] == value {
                next += 1;
            }

            if next == index + 1 {
                sum += value;
            }
            index = next;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn singleton_is_unique() {
        assert_eq!(Solution::sum_of_unique(vec![7]), 7);
        assert_eq!(Solution::sum_of_unique_by_sorting(vec![7]), 7);
    }

    #[test]
    fn sums_all_distinct_values() {
        let nums = vec![1, 2, 3, 100];
        assert_eq!(Solution::sum_of_unique(nums.clone()), 106);
        assert_eq!(Solution::sum_of_unique_by_sorting(nums), 106);
    }

    #[test]
    fn repeated_values_are_excluded() {
        let nums = vec![42, 42, 42, 42];
        assert_eq!(Solution::sum_of_unique(nums.clone()), 0);
        assert_eq!(Solution::sum_of_unique_by_sorting(nums), 0);
    }

    #[test]
    fn includes_only_values_with_frequency_one() {
        let nums = vec![1, 1, 2, 2, 2, 3, 4, 4, 5, 6, 6, 6];
        assert_eq!(Solution::sum_of_unique(nums.clone()), 8);
        assert_eq!(Solution::sum_of_unique_by_sorting(nums), 8);
    }

    #[test]
    fn sorting_a_passed_copy_leaves_the_original_unchanged() {
        let original = vec![5, 1, 1, 2, 3];
        assert_eq!(Solution::sum_of_unique_by_sorting(original.clone()), 10);
        assert_eq!(original, vec![5, 1, 1, 2, 3]);
    }
}
