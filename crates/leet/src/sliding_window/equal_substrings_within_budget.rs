/// LeetCode 1208: Get Equal Substrings Within Budget.
pub struct Solution;

impl Solution {
    /// Sliding window over ASCII byte differences, O(n) time and O(1) extra space.
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (mut left, mut cost, mut longest) = (0, 0, 0);

        for right in 0..s.len() {
            cost += s[right].abs_diff(t[right]) as i32;

            while cost > max_cost {
                cost -= s[left].abs_diff(t[left]) as i32;
                left += 1;
            }

            longest = longest.max(right + 1 - left);
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn official_examples() {
        assert_eq!(
            Solution::equal_substring("abcd".into(), "bcdf".into(), 3),
            3
        );
        assert_eq!(
            Solution::equal_substring("abcd".into(), "cdef".into(), 3),
            1
        );
        assert_eq!(
            Solution::equal_substring("abcd".into(), "acde".into(), 0),
            1
        );
    }

    #[test]
    fn shrinks_multiple_times_and_finds_a_later_window() {
        assert_eq!(
            Solution::equal_substring("aaaaaa".into(), "bbdbbb".into(), 3),
            3
        );
    }

    #[test]
    fn zero_budget_with_no_matching_characters() {
        assert_eq!(Solution::equal_substring("xyz".into(), "abc".into(), 0), 0);
    }

    #[test]
    fn equal_strings_need_no_budget() {
        assert_eq!(
            Solution::equal_substring("rust".into(), "rust".into(), 0),
            4
        );
    }

    #[test]
    fn single_character_respects_the_budget_boundary() {
        assert_eq!(Solution::equal_substring("z".into(), "a".into(), 25), 1);
        assert_eq!(Solution::equal_substring("a".into(), "z".into(), 24), 0);
    }
}
