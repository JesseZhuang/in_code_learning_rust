/// leet 1930

pub struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: &str) -> i32 {
        let bytes = s.as_bytes();
        let mut first = [i32::MAX; 26];
        let mut last = [0i32; 26];
        for (i, &b) in bytes.iter().enumerate() {
            let id = (b - b'a') as usize;
            first[id] = first[id].min(i as i32);
            last[id] = i as i32;
        }
        let mut res = 0;
        for i in 0..26 {
            if first[i] < last[i] {
                let mut seen = [false; 26];
                for j in (first[i] + 1)..last[i] {
                    seen[(bytes[j as usize] - b'a') as usize] = true;
                }
                res += seen.iter().filter(|&&x| x).count() as i32;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_palindromic_subsequence() {
        assert_eq!(3, Solution::count_palindromic_subsequence("aabca"));
        assert_eq!(0, Solution::count_palindromic_subsequence("adc"));
        assert_eq!(4, Solution::count_palindromic_subsequence("bbcbaba"));
    }
}
