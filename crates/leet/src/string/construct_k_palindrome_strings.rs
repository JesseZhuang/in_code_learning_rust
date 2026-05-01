/// LeetCode 1400

pub struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let n = s.len() as i32;
        if n < k { return false; }
        if n == k { return true; }
        let mut cnt = [0i32; 26];
        for c in s.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        let odd = cnt.iter().filter(|&&c| c & 1 == 1).count() as i32;
        odd <= k
    }
}

#[cfg(test)]
mod tests {
    use crate::string::construct_k_palindrome_strings::Solution;

    #[test]
    fn test_can_construct() {
        assert!(Solution::can_construct("annabelle".to_string(), 2));
        assert!(!Solution::can_construct("leetcode".to_string(), 3));
        assert!(Solution::can_construct("true".to_string(), 4));
    }
}
