use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// Greedy scan with a set. O(n) time and O(1) extra space for 26 letters.
    pub fn partition_string(s: String) -> i32 {
        let mut partitions = 1;
        let mut seen = HashSet::new();
        for letter in s.bytes() {
            if !seen.insert(letter) {
                partitions += 1;
                seen.clear();
                seen.insert(letter);
            }
        }
        partitions
    }

    /// Greedy scan with a 26-bit mask. O(n) time and O(1) extra space.
    pub fn partition_string_bitmask(s: String) -> i32 {
        let mut partitions = 1;
        let mut seen = 0_u32;
        for letter in s.bytes() {
            let bit = 1_u32 << (letter - b'a');
            if seen & bit != 0 {
                partitions += 1;
                seen = 0;
            }
            seen |= bit;
        }
        partitions
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_partition_count(s: &str, expected: i32) {
        assert_eq!(Solution::partition_string(s.to_owned()), expected, "set");
        assert_eq!(
            Solution::partition_string_bitmask(s.to_owned()),
            expected,
            "bitmask"
        );
    }

    #[test]
    fn mixed_duplicates() {
        assert_partition_count("abacaba", 4);
    }

    #[test]
    fn all_characters_repeat() {
        assert_partition_count("ssssss", 6);
    }

    #[test]
    fn single_character() {
        assert_partition_count("s", 1);
    }

    #[test]
    fn full_alphabet_is_one_partition() {
        assert_partition_count("abcdefghijklmnopqrstuvwxyz", 1);
    }

    #[test]
    fn duplicate_after_full_alphabet_starts_new_partition() {
        assert_partition_count("abcdefghijklmnopqrstuvwxyzabc", 2);
    }

    #[test]
    fn maximum_length() {
        assert_partition_count(&"a".repeat(100_000), 100_000);
    }
}
