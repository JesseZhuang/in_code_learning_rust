//! LeetCode 38, medium, tags: math, string, simulation.

pub struct Solution;

impl Solution {
    /// Iteratively run-length-encode the previous term.
    /// Time O(1.3^n) (Conway's constant), Space O(1.3^n).
    pub fn count_and_say(n: i32) -> String {
        let mut res = String::from("1");
        for _ in 1..n {
            let bytes = res.as_bytes();
            let mut next = String::new();
            let m = bytes.len();
            let mut i = 0;
            while i < m {
                let mut count = 1;
                while i + 1 < m && bytes[i] == bytes[i + 1] {
                    count += 1;
                    i += 1;
                }
                next.push_str(&count.to_string());
                next.push(bytes[i] as char);
                i += 1;
            }
            res = next;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn base() {
        assert_eq!(Solution::count_and_say(1), "1");
    }

    #[test]
    fn first_ten_terms() {
        let expected = [
            "1",
            "11",
            "21",
            "1211",
            "111221",
            "312211",
            "13112221",
            "1113213211",
            "31131211131221",
            "13211311123113112211",
        ];
        for (idx, want) in expected.iter().enumerate() {
            assert_eq!(&Solution::count_and_say((idx + 1) as i32), want);
        }
    }

    #[test]
    fn only_digits_1_to_3() {
        for i in 1..=15 {
            for c in Solution::count_and_say(i).chars() {
                assert!(c == '1' || c == '2' || c == '3');
            }
        }
    }

    #[test]
    fn monotonic_length() {
        let mut prev = 0usize;
        for i in 1..=12 {
            let len = Solution::count_and_say(i).len();
            assert!(len >= prev);
            prev = len;
        }
    }
}
