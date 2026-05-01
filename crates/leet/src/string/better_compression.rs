/// leet 3167 HackerRank Better Compression

pub struct Solution;

impl Solution {
    /// O(n) time, O(1) space.
    pub fn better_compression(s: &str) -> String {
        let mut res = [0u32; 26];
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut i = 0;
        while i < n {
            let c = (bytes[i] - b'a') as usize;
            i += 1;
            let mut cnt: u32 = 0;
            while i < n && bytes[i].is_ascii_digit() {
                cnt = cnt * 10 + (bytes[i] - b'0') as u32;
                i += 1;
            }
            res[c] += cnt;
        }
        let mut result = String::new();
        for j in 0..26 {
            if res[j] > 0 {
                result.push((b'a' + j as u8) as char);
                result.push_str(&res[j].to_string());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_better_compression() {
        assert_eq!(Solution::better_compression("a3c9b2c1"), "a3b2c10");
        assert_eq!(Solution::better_compression("a12b56c1"), "a12b56c1");
        assert_eq!(Solution::better_compression("a12c56a1b5"), "a13b5c56");
        assert_eq!(Solution::better_compression("c2b3a1"), "a1b3c2");
        assert_eq!(Solution::better_compression("a2b4c1"), "a2b4c1");
    }
}
