/// LeetCode 1358: Number of Substrings Containing All Three Characters.
pub struct Solution;

impl Solution {
    /// Last-seen indices: O(n) time and O(1) auxiliary space.
    pub fn number_of_substrings(s: String) -> i32 {
        let mut last_seen = [-1; 3];
        let mut total = 0;

        for (right, byte) in s.bytes().enumerate() {
            last_seen[(byte - b'a') as usize] = right as i32;
            // Starts through the earliest last-seen index include all three.
            // Until all characters appear, the minimum stays -1 and adds zero.
            total += last_seen.iter().min().unwrap() + 1;
        }

        total
    }

    /// Shrinking window: O(n) time and O(1) auxiliary space.
    /// Each byte enters and leaves the window at most once.
    pub fn number_of_substrings_sliding_window(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut counts = [0; 3];
        let mut left = 0;
        let mut total = 0;

        for &byte in bytes {
            counts[(byte - b'a') as usize] += 1;
            while counts.iter().all(|&count| count > 0) {
                counts[(bytes[left] - b'a') as usize] -= 1;
                left += 1;
            }
            // The first invalid start is left; every earlier start is valid.
            total += left as i32;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_both(s: &str, expected: i32) {
        assert_eq!(Solution::number_of_substrings(s.to_owned()), expected);
        assert_eq!(
            Solution::number_of_substrings_sliding_window(s.to_owned()),
            expected
        );
    }

    #[test]
    fn test_examples() {
        assert_both("abcabc", 10);
        assert_both("aaacb", 3);
    }

    #[test]
    fn test_missing_character() {
        assert_both("aaaaa", 0);
        assert_both("ababab", 0);
        assert_both("bcbcbc", 0);
        assert_both("acacac", 0);
    }

    #[test]
    fn test_minimum_valid_string() {
        assert_both("abc", 1);
        assert_both("cba", 1);
    }

    #[test]
    fn test_repeated_and_overlapping_valid_substrings() {
        assert_both("aabbcc", 4);
        assert_both("abca", 3);
    }

    #[test]
    fn test_empty_string() {
        assert_both("", 0);
    }

    #[test]
    fn test_maximum_length() {
        let s = "abc".repeat(16_666) + "ab";
        assert_both(&s, 1_249_925_001);
    }
}
