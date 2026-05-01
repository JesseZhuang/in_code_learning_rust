/// LeetCode 2461

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    /// Sliding window with last-seen map. O(n) time, O(n) space.
    pub fn maximum_subarray_sum(nums: &[i32], k: i32) -> i64 {
        let k = k as usize;
        let mut res: i64 = 0;
        let mut cur: i64 = 0;
        let mut dup: i64 = -1;
        let mut last: HashMap<i32, i64> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            cur += v as i64;
            if i >= k {
                cur -= nums[i - k] as i64;
            }
            if let Some(&prev) = last.get(&v) {
                dup = dup.max(prev);
            }
            if i as i64 - dup >= k as i64 {
                res = res.max(cur);
            }
            last.insert(v, i as i64);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::maximum_subarray_sum(&[1, 5, 4, 2, 9, 9, 9], 3), 15);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::maximum_subarray_sum(&[4, 4, 4], 3), 0);
    }
}
