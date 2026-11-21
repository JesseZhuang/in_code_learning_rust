pub struct Solution;

impl Solution {
    /// One-pass DP. O(n) time, O(1) extra space.
    pub fn minimum_deletions(s: String) -> i32 {
        let mut b_count = 0;
        let mut deletions = 0;

        for &byte in s.as_bytes() {
            if byte == b'b' {
                b_count += 1;
            } else {
                // Delete this 'a', or delete every earlier 'b'.
                deletions = (deletions + 1).min(b_count);
            }
        }

        deletions
    }

    /// Check every split using suffix 'a' counts. O(n) time, O(n) extra space.
    pub fn minimum_deletions_by_split(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut suffix_a = vec![0; n + 1];

        for i in (0..n).rev() {
            suffix_a[i] = suffix_a[i + 1] + i32::from(bytes[i] == b'a');
        }

        let mut prefix_b = 0;
        let mut minimum = n as i32;
        for split in 0..=n {
            minimum = minimum.min(prefix_b + suffix_a[split]);
            if split < n && bytes[split] == b'b' {
                prefix_b += 1;
            }
        }

        minimum
    }
}

#[cfg(test)]
mod tests {
    use crate::string::minimum_deletions_to_make_string_balanced::Solution;

    #[test]
    fn minimum_deletions_examples_and_edges() {
        let cases = [
            ("aababbab", 2),
            ("bbaaaaabb", 2),
            ("a", 0),
            ("b", 0),
            ("", 0),
            ("aaaa", 0),
            ("bbbb", 0),
            ("aaabbb", 0),
            ("ba", 1),
            ("bbbaaa", 3),
            ("aabbab", 1),
        ];

        for (input, expected) in cases {
            assert_eq!(
                expected,
                Solution::minimum_deletions(input.to_string()),
                "{input}"
            );
        }
    }

    #[test]
    fn minimum_deletions_by_split_examples_and_edges() {
        let cases = [
            ("aababbab", 2),
            ("bbaaaaabb", 2),
            ("a", 0),
            ("b", 0),
            ("", 0),
            ("aaaa", 0),
            ("bbbb", 0),
            ("aaabbb", 0),
            ("ba", 1),
            ("bbbaaa", 3),
            ("aabbab", 1),
        ];

        for (input, expected) in cases {
            assert_eq!(
                expected,
                Solution::minimum_deletions_by_split(input.to_string()),
                "{input}"
            );
        }
    }

    #[test]
    fn both_methods_handle_maximum_input_length() {
        let input = format!("{}{}", "b".repeat(50_000), "a".repeat(50_000));

        assert_eq!(50_000, Solution::minimum_deletions(input.clone()));
        assert_eq!(50_000, Solution::minimum_deletions_by_split(input));
    }
}
