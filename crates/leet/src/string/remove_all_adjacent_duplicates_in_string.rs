pub struct Solution;

impl Solution {
    /// Time: O(n), where n is the number of characters. Space: O(n) for the stack.
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();

        for ch in s.chars() {
            if stack.last() == Some(&ch) {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }

        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn examples() {
        assert_eq!(Solution::remove_duplicates("abbaca".to_string()), "ca");
        assert_eq!(Solution::remove_duplicates("azxxzy".to_string()), "ay");
    }

    #[test]
    fn cascading_pairs() {
        assert_eq!(Solution::remove_duplicates("abccba".to_string()), "");
        assert_eq!(Solution::remove_duplicates("aabccba".to_string()), "a");
    }

    #[test]
    fn singleton() {
        assert_eq!(Solution::remove_duplicates("a".to_string()), "a");
    }

    #[test]
    fn no_duplicate() {
        assert_eq!(Solution::remove_duplicates("abcdef".to_string()), "abcdef");
    }

    #[test]
    fn long_string_constraint() {
        assert_eq!(Solution::remove_duplicates("a".repeat(100_000)), "");
    }
}
