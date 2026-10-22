pub struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut first_counts = [0usize; 26];
        let mut second_counts = [0usize; 26];
        for (first, second) in word1.bytes().zip(word2.bytes()) {
            first_counts[(first - b'a') as usize] += 1;
            second_counts[(second - b'a') as usize] += 1;
        }

        if first_counts
            .iter()
            .zip(second_counts.iter())
            .any(|(&first, &second)| (first == 0) != (second == 0))
        {
            return false;
        }

        first_counts.sort_unstable();
        second_counts.sort_unstable();
        first_counts == second_counts
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_close(word1: &str, word2: &str, expected: bool) {
        assert_eq!(
            Solution::close_strings(word1.to_string(), word2.to_string()),
            expected
        );
    }

    #[test]
    fn official_examples() {
        assert_close("abc", "bca", true);
        assert_close("a", "aa", false);
        assert_close("cabbba", "abbccc", true);
    }

    #[test]
    fn different_support_cannot_be_transformed() {
        assert_close("ab", "cd", false);
        assert_close("aabb", "aabc", false);
    }

    #[test]
    fn same_support_but_different_frequencies() {
        assert_close("aaabbc", "aabbcc", false);
    }

    #[test]
    fn swapped_frequencies_are_close() {
        assert_close("aaabbc", "aabbbc", true);
    }

    #[test]
    fn single_character_and_empty_strings() {
        assert_close("a", "a", true);
        assert_close("", "", true);
        assert_close("", "a", false);
    }

    #[test]
    fn maximum_length_inputs() {
        let first = "a".repeat(99_999) + "b";
        let second = "a".to_string() + &"b".repeat(99_999);
        assert_close(&first, &second, true);
        assert_close(&"a".repeat(100_000), &"b".repeat(100_000), false);
    }
}
