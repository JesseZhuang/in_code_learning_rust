pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k as usize;
        let mut stack: Vec<u8> = Vec::new(); // O(n) space
        for &b in num.as_bytes() { // O(n)
            while k > 0 && !stack.is_empty() && *stack.last().unwrap() > b {
                stack.pop(); // amortized O(1)
                k -= 1;
            }
            stack.push(b);
        }
        stack.truncate(stack.len() - k);
        let result = String::from_utf8(stack).unwrap();
        let stripped = result.trim_start_matches('0');
        if stripped.is_empty() { "0".to_string() } else { stripped.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::remove_kdigits("1432219".into(), 3), "1219");
        assert_eq!(Solution::remove_kdigits("10200".into(), 1), "200");
        assert_eq!(Solution::remove_kdigits("10".into(), 2), "0");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::remove_kdigits("9".into(), 1), "0");
        assert_eq!(Solution::remove_kdigits("123".into(), 0), "123");
        assert_eq!(Solution::remove_kdigits("9876".into(), 2), "76");
        assert_eq!(Solution::remove_kdigits("10001".into(), 1), "1");
        assert_eq!(Solution::remove_kdigits("1234".into(), 2), "12");
    }
}
