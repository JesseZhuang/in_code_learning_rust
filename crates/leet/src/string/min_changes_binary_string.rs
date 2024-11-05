// lc 2914

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let (mut res, s) = (0, s.as_str());
        for i in (0..s.len()).step_by(2) {
            if s[i..i + 1] != s[i + 1..i + 2] { res += 1 }
        }
        res
    }
}

impl Solution2 {
    pub fn min_changes(s: String) -> i32 {
        (0..s.len() - 1).step_by(2)
            .filter(|&i| s.as_bytes()[i] != s.as_bytes()[i + 1]).count() as i32
    }
}

struct Solution;
struct Solution2;
