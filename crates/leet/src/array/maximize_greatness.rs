pub struct Solution;

impl Solution {
    /// Sorts the values and greedily pairs each smallest unmatched value with
    /// the first larger value available.
    /// Time: O(n log n), Space: O(log n) stack for the in-place unstable sort.
    pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut smallest_unmatched = 0;
        let mut candidate = 0;
        while candidate < nums.len() {
            if nums[candidate] > nums[smallest_unmatched] {
                smallest_unmatched += 1;
            }
            candidate += 1;
        }

        smallest_unmatched as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn matches_leetcode_examples() {
        assert_eq!(
            Solution::maximize_greatness(vec![1, 3, 5, 2, 1, 3, 1]),
            4
        );
        assert_eq!(Solution::maximize_greatness(vec![1, 2, 3, 4]), 3);
    }

    #[test]
    fn handles_equal_and_singleton_inputs() {
        assert_eq!(Solution::maximize_greatness(vec![7, 7, 7, 7]), 0);
        assert_eq!(Solution::maximize_greatness(vec![42]), 0);
    }

    #[test]
    fn pairs_duplicate_values_with_strictly_greater_values() {
        assert_eq!(Solution::maximize_greatness(vec![1, 1, 1, 2, 2, 3]), 3);
        // The shorter duplicate-heavy example has a maximum greatness of 3.
        assert_eq!(Solution::maximize_greatness(vec![1, 3, 5, 2, 1]), 3);
    }

    #[test]
    fn handles_high_values() {
        assert_eq!(Solution::maximize_greatness(vec![99_998, 99_999, 100_000]), 2);
    }
}
