use std::cmp::max;

/// leet 53

impl Solution {
    /// 0 ms, 3.34 mb.
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut max_here, mut res) = (0, i32::MIN);
        for n in nums {
            max_here = max(max_here, n);
            res = max(res, max_here);
        }
        res
    }
}

struct Solution;
