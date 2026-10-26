pub struct Solution;

impl Solution {
    /// Time: O(n); extra space: O(1), reusing the word's buffer.
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut bytes = word.into_bytes();
        if let Some(mut right) = bytes.iter().position(|&byte| byte == ch as u8) {
            let mut left = 0;
            while left < right {
                bytes.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        String::from_utf8(bytes).expect("lowercase letters are valid UTF-8")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn reverses_through_first_middle_occurrence() {
        assert_eq!(Solution::reverse_prefix("abcdefd".into(), 'd'), "dcbaefd");
        assert_eq!(Solution::reverse_prefix("abccba".into(), 'c'), "cbacba");
    }

    #[test]
    fn keeps_word_unchanged_when_character_is_absent() {
        assert_eq!(Solution::reverse_prefix("abcdef".into(), 'z'), "abcdef");
    }

    #[test]
    fn handles_first_and_last_positions() {
        assert_eq!(Solution::reverse_prefix("abcdef".into(), 'a'), "abcdef");
        assert_eq!(Solution::reverse_prefix("abcdef".into(), 'f'), "fedcba");
        assert_eq!(Solution::reverse_prefix("a".into(), 'a'), "a");
        assert_eq!(Solution::reverse_prefix("a".into(), 'b'), "a");
    }

    #[test]
    fn handles_maximum_length_word() {
        let word = format!("{}a", "b".repeat(249));
        let expected = format!("a{}", "b".repeat(249));
        assert_eq!(Solution::reverse_prefix(word, 'a'), expected);
    }
}
