/// leet 408, lint  637

pub struct Solution;

/// 42 ms, 3.16 mb
impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let (wc, ac): (Vec<char>, Vec<char>) = (word.chars().collect(), abbr.chars().collect());
        let (mut i, mut j, mut x) = (0, 0, 0);
        let (n, m) = (wc.len(), ac.len());
        while i < n && j < m {
            if ac[j].is_numeric() {
                if ac[j] == '0' && x == 0 { return false; }
                x = x * 10 + ac[j].to_digit(10).unwrap() as usize;
            } else {
                i += x;
                x = 0;
                if i >= n || wc[i] != ac[j] { return false; }
                i += 1;
            }
            j += 1;
        }
        i + x == n && j == m
    }
}
