/// leet 713, lint 1075

/// 0 ms, 2.6 mb
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 { return 0; }
        let (mut res, mut prod, mut l, mut r, n) = (0, 1, 0, 0, nums.len());
        while r < n {
            prod *= nums[r];
            r += 1;
            while prod >= k {
                prod /= nums[l];
                l += 1;
            }
            res += r - l;
        }
        res as i32
    }
}

struct Solution;
