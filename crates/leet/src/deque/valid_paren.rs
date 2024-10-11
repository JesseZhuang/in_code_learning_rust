// lc 20

use std::collections::VecDeque;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st = VecDeque::new();
        for c in s.chars() {
            match c {
                '(' => st.push_back(')'),
                '[' => st.push_back(']'),
                '{' => st.push_back('}'),
                _ => if st.pop_back() != Some(c) { return false } // nice, no need check empty
            }
        }
        st.is_empty()
    }
}

struct Solution;