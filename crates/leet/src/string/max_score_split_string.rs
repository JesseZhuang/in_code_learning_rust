/// LeetCode 1422

pub struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut ones = bytes.iter().filter(|&&b| b == b'1').count() as i32;
        let mut zero = 0i32;
        let mut res = 0i32;
        for i in 0..bytes.len() - 1 {
            if bytes[i] == b'0' { zero += 1; } else { ones -= 1; }
            res = res.max(zero + ones);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::string::max_score_split_string::Solution;

    #[test]
    fn test_max_score() {
        assert_eq!(5, Solution::max_score("011101".to_string()));
        assert_eq!(5, Solution::max_score("00111".to_string()));
        assert_eq!(3, Solution::max_score("1111".to_string()));
    }
}
