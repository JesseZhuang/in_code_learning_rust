/// 31. Next Permutation https://leetcode.com/problems/next-permutation/
pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }
        // Step 1: find pivot — rightmost element smaller than successor, O(n)
        let pivot = (0..n - 1).rev().find(|&i| nums[i] < nums[i + 1]);
        if let Some(i) = pivot {
            // Step 2: find rightmost element larger than pivot, O(n)
            let j = (i + 1..n).rev().find(|&j| nums[j] > nums[i]).unwrap();
            nums.swap(i, j);
            // Step 3: reverse suffix, O(n). Total: O(n) time, O(1) space
            nums[i + 1..].reverse();
        } else {
            nums.reverse();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascending() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn descending() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn duplicates() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }

    #[test]
    fn single_element() {
        let mut nums = vec![1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn two_ascending() {
        let mut nums = vec![1, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1]);
    }

    #[test]
    fn duplicates_complex() {
        let mut nums = vec![1, 2, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 2]);
    }

    #[test]
    fn middle_pivot() {
        let mut nums = vec![2, 3, 1, 3, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 3, 3, 1, 3]);
    }
}
