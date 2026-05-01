/// leet 423

pub struct Solution;

impl Solution {
    /// O(n) time, O(1) space.
    pub fn original_digits(s: String) -> String {
        let mut cnt = [0i32; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        let mut res = [0i32; 10];
        res[0] = cnt[(b'z' - b'a') as usize];
        res[2] = cnt[(b'w' - b'a') as usize];
        res[4] = cnt[(b'u' - b'a') as usize];
        res[6] = cnt[(b'x' - b'a') as usize];
        res[8] = cnt[(b'g' - b'a') as usize];
        res[1] = cnt[(b'o' - b'a') as usize] - (res[0] + res[2] + res[4]);
        res[3] = cnt[(b'r' - b'a') as usize] - (res[0] + res[4]);
        res[5] = cnt[(b'f' - b'a') as usize] - res[4];
        res[7] = cnt[(b's' - b'a') as usize] - res[6];
        res[9] = cnt[(b'i' - b'a') as usize] - (res[5] + res[6] + res[8]);
        let mut result = String::new();
        for i in 0..10 {
            for _ in 0..res[i] {
                result.push_str(&i.to_string());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_original_digits() {
        assert_eq!(
            Solution::original_digits("owoztneoer".to_string()),
            "012"
        );
        assert_eq!(Solution::original_digits("fviefuro".to_string()), "45");
        assert_eq!(
            Solution::original_digits(
                "zeroonetwothreefourfivesixseveneightnine".to_string()
            ),
            "0123456789"
        );
    }
}
