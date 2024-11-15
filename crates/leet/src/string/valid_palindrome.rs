// leet code 125

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s.chars().filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_uppercase());
        while let (Some(l), Some(r)) = (chars.next(), chars.next_back()) {
            if l != r { return false; }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::string::valid_palindrome::Solution;

    #[test]
    fn test() {
        assert!(Solution::is_palindrome(String::from("a, bb -a")));
        assert!(Solution::is_palindrome(String::from("a, b,,,,cb -a")));
    }
}