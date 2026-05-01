/// leet 214

pub struct Solution;

impl Solution {
    /// Rolling hash approach. O(n) time, O(n) space.
    pub fn shortest_palindrome(s: String) -> String {
        let base: u64 = 31;
        let modulus: u64 = 1_000_000_007;
        let mut pw: u64 = 1;
        let mut forward: u64 = 0;
        let mut reverse: u64 = 0;
        let mut end: i32 = -1;
        for (i, c) in s.bytes().enumerate() {
            let id = (c - b'a' + 1) as u64;
            forward = (forward * base + id) % modulus;
            reverse = (reverse + id * pw) % modulus;
            pw = (pw * base) % modulus;
            if forward == reverse {
                end = i as i32;
            }
        }
        let suffix = &s[(end + 1) as usize..];
        let rev_suffix: String = suffix.chars().rev().collect();
        rev_suffix + &s
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_shortest_palindrome() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_string()),
            "aaacecaaa"
        );
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_string()),
            "dcbabcd"
        );
        assert_eq!(Solution::shortest_palindrome("".to_string()), "");
        assert_eq!(Solution::shortest_palindrome("a".to_string()), "a");
        assert_eq!(Solution::shortest_palindrome("aba".to_string()), "aba");
        assert_eq!(
            Solution::shortest_palindrome("aaabc".to_string()),
            "cbaaabc"
        );
    }
}
