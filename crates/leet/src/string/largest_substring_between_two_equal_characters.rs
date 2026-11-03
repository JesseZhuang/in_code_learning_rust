/// LeetCode 1624 - Largest Substring Between Two Equal Characters
pub struct Solution;

impl Solution {
    /// Keeps each lowercase character's first index while scanning once.
    /// Time: O(n), Space: O(1), because the table always has 26 entries.
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut first_index = [-1_i32; 26];
        let mut longest = -1;

        for (index, byte) in s.bytes().enumerate() {
            let slot = (byte - b'a') as usize;
            if first_index[slot] == -1 {
                first_index[slot] = index as i32;
            } else {
                longest = longest.max(index as i32 - first_index[slot] - 1);
            }
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_longest_substring_between_matching_endpoints() {
        assert_eq!(
            Solution::max_length_between_equal_characters("abca".to_string()),
            2
        );
    }

    #[test]
    fn returns_zero_for_adjacent_matching_characters() {
        assert_eq!(
            Solution::max_length_between_equal_characters("aa".to_string()),
            0
        );
    }

    #[test]
    fn returns_negative_one_when_no_character_repeats() {
        assert_eq!(
            Solution::max_length_between_equal_characters("cbzxy".to_string()),
            -1
        );
    }

    #[test]
    fn retains_the_first_occurrence_for_later_matches() {
        assert_eq!(
            Solution::max_length_between_equal_characters("cabbac".to_string()),
            4
        );
    }
}
