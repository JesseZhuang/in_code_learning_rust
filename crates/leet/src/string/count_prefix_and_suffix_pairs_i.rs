pub struct Solution;

impl Solution {
    /// Time: O(n² · L); auxiliary space: O(1), where L is the maximum word length.
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        for (i, prefix) in words.iter().enumerate() {
            for word in &words[i + 1..] {
                if word.starts_with(prefix.as_str()) && word.ends_with(prefix.as_str()) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| (*word).to_owned()).collect()
    }

    #[test]
    fn counts_pairs_in_the_official_examples() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(strings(&["a", "aba", "ababa", "aa"])),
            4
        );
        assert_eq!(
            Solution::count_prefix_suffix_pairs(strings(&["pa", "papa", "ma", "mama"])),
            2
        );
        assert_eq!(
            Solution::count_prefix_suffix_pairs(strings(&["abab", "ab"])),
            0
        );
    }

    #[test]
    fn single_word_has_no_pairs() {
        assert_eq!(Solution::count_prefix_suffix_pairs(strings(&["a"])), 0);
    }

    #[test]
    fn counts_equal_words_at_distinct_indices() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(strings(&["ab", "ab", "ab"])),
            3
        );
    }

    #[test]
    fn rejects_words_that_are_only_prefixes_or_only_suffixes() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(strings(&["ab", "abc"])),
            0
        );
        assert_eq!(
            Solution::count_prefix_suffix_pairs(strings(&["bc", "abc"])),
            0
        );
    }

    #[test]
    fn rejects_longer_earlier_words() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(strings(&["aba", "a"])),
            0
        );
    }

    #[test]
    fn counts_all_pairs_at_maximum_input_size() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec!["abcdefghij".to_owned(); 50]),
            1225
        );
    }
}
