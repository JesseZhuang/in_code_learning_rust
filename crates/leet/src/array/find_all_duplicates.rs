/// LeetCode 442 - Find All Duplicates in an Array
///
/// Given an integer array nums of length n where all integers are in the range
/// [1, n] and each integer appears at most twice, return all integers that
/// appear twice.
pub struct Solution;

impl Solution {
    /// Negation marking: for each value v, negate nums[v-1].
    /// If already negative, v is a duplicate.
    /// Time: O(n), Space: O(1) (output excluded)
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..nums.len() {
            let idx = nums[i].unsigned_abs() as usize - 1; // map value to index — O(1)
            if nums[idx] < 0 {
                // already visited — v is duplicate
                result.push(idx as i32 + 1);
            } else {
                nums[idx] = -nums[idx]; // mark as seen — O(1) per element
            }
        }
        result // total: O(n) time, O(1) extra space
    }

    /// Cyclic sort: place each value v at index v-1, then scan for mismatches.
    /// Time: O(n), Space: O(1) (output excluded)
    pub fn find_duplicates_cyclic_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        // Phase 1: cyclic sort — each element swapped at most once → O(n) total
        let mut i = 0;
        while i < n {
            let target = nums[i] as usize - 1; // correct index for value nums[i]
            if nums[i] != nums[target] {
                nums.swap(i, target); // place nums[i] at its home — O(1) per swap
            } else {
                i += 1;
            }
        }

        // Phase 2: scan for mismatches — O(n)
        let mut result = Vec::new();
        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                result.push(nums[i]); // value at wrong index is duplicate
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    #[test]
    fn test_find_duplicates_negation() {
        let cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3]),
            (vec![1, 1, 2], vec![1]),
            (vec![1], vec![]),
            (vec![1, 2, 1, 2], vec![1, 2]),
            (vec![2, 2], vec![2]),
            (vec![1, 2, 3, 4, 5], vec![]),
            (vec![1, 3, 4, 2, 1, 4], vec![1, 4]),
        ];
        for (nums, expected) in cases {
            assert_eq!(sorted(Solution::find_duplicates(nums)), sorted(expected));
        }
    }

    #[test]
    fn test_find_duplicates_cyclic_sort() {
        let cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3]),
            (vec![1, 1, 2], vec![1]),
            (vec![1], vec![]),
            (vec![1, 2, 1, 2], vec![1, 2]),
            (vec![2, 2], vec![2]),
            (vec![1, 2, 3, 4, 5], vec![]),
            (vec![1, 3, 4, 2, 1, 4], vec![1, 4]),
        ];
        for (nums, expected) in cases {
            assert_eq!(
                sorted(Solution::find_duplicates_cyclic_sort(nums)),
                sorted(expected),
            );
        }
    }
}
