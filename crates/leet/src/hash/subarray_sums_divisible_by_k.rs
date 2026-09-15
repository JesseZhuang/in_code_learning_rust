use std::collections::HashMap;

/// leet 974

/// Solution 1: Prefix Sum + HashMap. O(n) time, O(k) space.
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);
        let (mut prefix, mut count) = (0, 0);
        for n in nums {
            prefix += n;
            let rem = ((prefix % k) + k) % k;
            if let Some(&c) = map.get(&rem) {
                count += c;
            }
            *map.entry(rem).or_insert(0) += 1;
        }
        count
    }
}

struct Solution;

/// Solution 2: Brute Force. O(n²) time, O(1) space.
impl Solution2 {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum % k == 0 {
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
    fn test_subarrays_div_by_k() {
        let cases = vec![
            (vec![4, 5, 0, -2, -3, 1], 5, 7),
            (vec![5], 9, 0),
            (vec![5, 10, 15], 5, 6),
            (vec![0], 1, 1),
            (vec![-1, 2, 9], 2, 2),
            (vec![0, 0, 0], 3, 6),
            (vec![1, 2, 3], 100, 0),
            (vec![1, 2, 3], 1, 6),
            (vec![-5, 1, 2, -3, 4], 5, 3),
            (vec![6], 3, 1),
        ];
        for (nums, k, expected) in cases {
            assert_eq!(Solution::subarrays_div_by_k(nums.clone(), k), expected);
            assert_eq!(Solution2::subarrays_div_by_k(nums, k), expected);
        }
    }
}
