use std::collections::HashMap;

/// LeetCode 532 - K-diff Pairs in an Array
pub struct Solution;

impl Solution {
    /// Count values and check for k-diff pairs.
    /// Time O(n), Space O(n).
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for &n in &nums {
            *cnt.entry(n).or_insert(0) += 1;
        }
        let mut res = 0;
        for (&key, &val) in &cnt {
            if k > 0 && cnt.contains_key(&(key + k)) {
                res += 1;
            } else if k == 0 && val > 1 {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(2, Solution::find_pairs(vec![3, 1, 4, 1, 5], 2));
    }

    #[test]
    fn test_example2() {
        assert_eq!(4, Solution::find_pairs(vec![1, 2, 3, 4, 5], 1));
    }

    #[test]
    fn test_example3() {
        assert_eq!(1, Solution::find_pairs(vec![1, 3, 1, 5, 4], 0));
    }
}
