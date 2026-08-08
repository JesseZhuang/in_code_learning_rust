use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// LeetCode 1048 - Longest String Chain
    /// Time: O(n * L^2) where n = number of words, L = max word length
    /// Space: O(n * L) for the HashMap storing all words
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_by_key(|w| w.len()); // O(n log n) sort by length

        // dp[word] = length of longest chain ending at word
        let mut dp: HashMap<String, i32> = HashMap::new(); // O(n) space
        let mut ans = 1;

        for word in &words {
            let mut best = 1;
            // Try removing each character to form a predecessor — O(L) iterations
            for i in 0..word.len() {
                // Building predecessor string — O(L) per iteration
                let pred = format!("{}{}", &word[..i], &word[i + 1..]);
                if let Some(&prev_len) = dp.get(&pred) {
                    best = best.max(prev_len + 1);
                }
            }
            dp.insert(word.clone(), best);
            ans = ans.max(best);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let words: Vec<String> = vec!["a", "b", "ba", "bca", "bda", "bdca"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::longest_str_chain(words), 4);
    }

    #[test]
    fn test_example2() {
        let words: Vec<String> = vec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::longest_str_chain(words), 5);
    }

    #[test]
    fn test_single_word() {
        let words: Vec<String> = vec!["abcd"].into_iter().map(String::from).collect();
        assert_eq!(Solution::longest_str_chain(words), 1);
    }

    #[test]
    fn test_no_chain() {
        let words: Vec<String> = vec!["ab", "cd", "ef"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::longest_str_chain(words), 1);
    }
}
