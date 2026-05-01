pub struct Solution;

impl Solution {
    pub fn count_distinct_strings(s: String, k: i32) -> i32 {
        let n = s.len() as u64;
        let k = k as u64;
        let m = 1_000_000_007u64;
        let mut ans = 1u64;
        for _ in 0..(n - k + 1) {
            ans = ans * 2 % m;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_distinct_strings() {
        assert_eq!(Solution::count_distinct_strings("1001".to_string(), 3), 4);
        assert_eq!(Solution::count_distinct_strings("10110".to_string(), 5), 2);
        // k == n: only 2 options (flip or not)
        assert_eq!(Solution::count_distinct_strings("01".to_string(), 2), 2);
        // k == 1: 2^n
        assert_eq!(Solution::count_distinct_strings("000".to_string(), 1), 8);
        // single char
        assert_eq!(Solution::count_distinct_strings("0".to_string(), 1), 2);
    }
}
