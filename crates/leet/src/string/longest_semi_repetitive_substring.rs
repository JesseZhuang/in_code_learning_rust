pub struct Solution;

impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let digits = s.as_bytes();
        let mut left = 0;
        let mut equal_pairs = 0;
        let mut longest = 0;

        // The right edge advances once, so this loop is O(n) time overall.
        for right in 0..digits.len() {
            if right > 0 && digits[right] == digits[right - 1] {
                equal_pairs += 1;
            }

            // Each left-edge advance removes one pair at most, keeping this O(n).
            while equal_pairs > 1 {
                if digits[left] == digits[left + 1] {
                    equal_pairs -= 1;
                }
                left += 1;
            }

            longest = longest.max(right - left + 1);
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_semi_repetitive_substring_examples() {
        let cases = [
            ("52233", 4),
            ("5494", 4),
            ("123456", 6),
            ("122345", 6),
            ("112233", 4),
            ("1233112", 5),
            ("111111", 2),
            ("1123455", 6),
            ("12", 2),
            ("11", 2),
            ("5", 1),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::longest_semi_repetitive_substring(input.to_string()),
                expected,
                "input: {input}"
            );
        }
    }
}
