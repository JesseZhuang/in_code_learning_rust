pub struct Solution;

impl Solution {
    /// Time: O(m + n); extra space: O(1), excluding the O(m + n) output.
    /// The problem guarantees lowercase English letters, so byte indexing is safe.
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut merged = String::with_capacity(word1.len() + word2.len());
        let (mut i, mut j) = (0, 0);
        while i < word1.len() && j < word2.len() {
            merged.push(word1.as_bytes()[i] as char);
            merged.push(word2.as_bytes()[j] as char);
            i += 1;
            j += 1;
        }
        merged.push_str(&word1[i..]);
        merged.push_str(&word2[j..]);
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn alternates_equal_length_words_starting_with_first_word() {
        assert_eq!(
            Solution::merge_alternately("abc".into(), "pqr".into()),
            "apbqcr"
        );
    }

    #[test]
    fn appends_remainder_from_longer_second_word() {
        assert_eq!(
            Solution::merge_alternately("ab".into(), "pqrs".into()),
            "apbqrs"
        );
    }

    #[test]
    fn appends_remainder_from_longer_first_word() {
        assert_eq!(
            Solution::merge_alternately("abcd".into(), "pq".into()),
            "apbqcd"
        );
    }

    #[test]
    fn handles_empty_words() {
        assert_eq!(Solution::merge_alternately("".into(), "abc".into()), "abc");
        assert_eq!(Solution::merge_alternately("abc".into(), "".into()), "abc");
        assert_eq!(Solution::merge_alternately("".into(), "".into()), "");
    }
}
