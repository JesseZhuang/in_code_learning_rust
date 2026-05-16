// leet code 17

pub struct Solution;

impl Solution {
    /// Iterative BFS approach. Time O(n * 4^n), Space O(4^n).
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mapping: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut result: Vec<String> = vec![String::new()];

        for ch in digits.chars() {
            let letters = mapping[(ch as usize) - ('2' as usize)];
            let mut next = Vec::with_capacity(result.len() * letters.len());
            for combo in &result {
                for letter in letters.chars() {
                    let mut s = combo.clone();
                    s.push(letter);
                    next.push(s);
                }
            }
            result = next;
        }
        result
    }

    /// Backtracking approach. Time O(n * 4^n), Space O(n) recursion + O(4^n) result.
    pub fn letter_combinations_v2(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mapping: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let digits: Vec<usize> = digits.chars().map(|c| c as usize - '2' as usize).collect();
        let mut result = Vec::new();
        let mut current = String::with_capacity(digits.len());
        Self::backtrack(&digits, &mapping, 0, &mut current, &mut result);
        result
    }

    fn backtrack(
        digits: &[usize],
        mapping: &[&str; 8],
        idx: usize,
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if idx == digits.len() {
            result.push(current.clone());
            return;
        }
        for letter in mapping[digits[idx]].chars() {
            current.push(letter);
            Self::backtrack(digits, mapping, idx + 1, current, result);
            current.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run(input: &str, expected: &[&str]) {
        let mut res1 = Solution::letter_combinations(input.to_string());
        let mut res2 = Solution::letter_combinations_v2(input.to_string());
        res1.sort();
        res2.sort();
        let mut exp: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
        exp.sort();
        assert_eq!(res1, exp);
        assert_eq!(res2, exp);
    }

    #[test]
    fn test_two_digits() {
        run(
            "23",
            &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
        );
    }

    #[test]
    fn test_empty() {
        run("", &[]);
    }

    #[test]
    fn test_single_digit_2() {
        run("2", &["a", "b", "c"]);
    }

    #[test]
    fn test_single_digit_7() {
        run("7", &["p", "q", "r", "s"]);
    }

    #[test]
    fn test_single_digit_9() {
        run("9", &["w", "x", "y", "z"]);
    }

    #[test]
    fn test_three_digits() {
        let res = Solution::letter_combinations("234".to_string());
        assert_eq!(res.len(), 27);
        let res2 = Solution::letter_combinations_v2("234".to_string());
        assert_eq!(res2.len(), 27);
    }

    #[test]
    fn test_repeated_digit() {
        let res = Solution::letter_combinations("22".to_string());
        assert_eq!(res.len(), 9);
        let res2 = Solution::letter_combinations_v2("22".to_string());
        assert_eq!(res2.len(), 9);
    }
}
