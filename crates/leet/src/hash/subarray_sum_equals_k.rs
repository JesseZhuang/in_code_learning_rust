use std::collections::HashMap;

/// leet 560

/// Solution 1: Prefix Sum + HashMap. O(n) time, O(n) space.
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);
        let (mut prefix, mut count) = (0, 0);
        for n in nums {
            prefix += n;
            if let Some(&c) = map.get(&(prefix - k)) {
                count += c;
            }
            *map.entry(prefix).or_insert(0) += 1;
        }
        count
    }
}

struct Solution;

/// Solution 2: Brute Force. O(n²) time, O(1) space.
impl Solution2 {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum == k {
                    count += 1;
                }
            }
        }
        count
    }
}

struct Solution2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarray_sum() {
        let cases = vec![
            (vec![1, 1, 1], 2, 2),
            (vec![1, 2, 3], 3, 2),
            (vec![1], 1, 1),
            (vec![1], 0, 0),
            (vec![-1, -1, 1], 0, 1),
            (vec![0, 0, 0], 0, 6),
            (vec![1, -1, 0], -1, 2),
            (vec![1, -1, 1, -1], 0, 4),
            (vec![1, 2, 3, 4, 5], 15, 1),
            (vec![1, 2, 3], 7, 0),
        ];
        for (nums, k, expected) in cases {
            assert_eq!(Solution::subarray_sum(nums.clone(), k), expected);
            assert_eq!(Solution2::subarray_sum(nums, k), expected);
        }
    }
}
