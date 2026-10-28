pub struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut unmatched_open = 0;

        for bracket in s.bytes() {
            if bracket == b'[' {
                unmatched_open += 1;
            } else if bracket == b']' && unmatched_open > 0 {
                unmatched_open -= 1;
            }
        }

        (unmatched_open + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_already_balanced() {
        assert_eq!(Solution::min_swaps("[]".to_string()), 0);
    }

    #[test]
    fn test_single_swap() {
        assert_eq!(Solution::min_swaps("][".to_string()), 1);
    }

    #[test]
    fn test_separated_unmatched_brackets() {
        assert_eq!(Solution::min_swaps("][][".to_string()), 1);
    }

    #[test]
    fn test_three_unmatched_brackets() {
        assert_eq!(Solution::min_swaps("]]][[[".to_string()), 2);
    }

    #[test]
    fn test_nested_brackets() {
        assert_eq!(Solution::min_swaps("[[[]]]".to_string()), 0);
        assert_eq!(Solution::min_swaps("[[][[]]]".to_string()), 0);
    }

    #[test]
    fn test_alternating_brackets() {
        assert_eq!(Solution::min_swaps("[][][][]".to_string()), 0);
    }

    #[test]
    fn test_odd_unmatched_count_rounds_up() {
        assert_eq!(Solution::min_swaps("]]]]][[[[[".to_string()), 3);
    }

    #[test]
    fn test_one_million_brackets() {
        let brackets = format!("{}{}", "]".repeat(500_000), "[".repeat(500_000));
        assert_eq!(Solution::min_swaps(brackets), 250_000);
    }
}
