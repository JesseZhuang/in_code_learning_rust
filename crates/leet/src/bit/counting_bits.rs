// leet code 338

pub struct Solution;

impl Solution {
    /// DP with bit shift: ans[i] = ans[i >> 1] + (i & 1)
    /// Time O(n), Space O(1) extra (output array only)
    pub fn count_bits_shift(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0i32; n + 1];
        for i in 1..=n {
            res[i] = res[i >> 1] + (i as i32 & 1); // right-shift drops LSB, add it back
        }
        res
    }

    /// DP with Brian Kernighan: ans[i] = ans[i & (i-1)] + 1
    /// Time O(n), Space O(1) extra (output array only)
    pub fn count_bits_kernighan(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0i32; n + 1];
        for i in 1..=n {
            res[i] = res[i & (i - 1)] + 1; // i & (i-1) clears lowest set bit
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn expected(n: usize) -> Vec<i32> {
        (0..=n).map(|i| i.count_ones() as i32).collect()
    }

    #[test]
    fn test_shift() {
        for &n in &[0, 1, 2, 5, 8, 15, 100_000] {
            assert_eq!(Solution::count_bits_shift(n as i32), expected(n));
        }
    }

    #[test]
    fn test_kernighan() {
        for &n in &[0, 1, 2, 5, 8, 15, 100_000] {
            assert_eq!(Solution::count_bits_kernighan(n as i32), expected(n));
        }
    }
}
