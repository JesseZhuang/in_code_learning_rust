/// leet 1055, lint 3652

pub struct Solution;

impl Solution {
    pub fn shortest_way(source: &str, target: &str) -> i32 {
        let (src, tgt) = (source.as_bytes(), target.as_bytes());
        let (m, n) = (src.len(), tgt.len());
        let (mut res, mut j) = (0, 0);
        while j < n {
            let mut found = false;
            let mut i = 0;
            while i < m && j < n {
                if src[i] == tgt[j] {
                    found = true;
                    j += 1;
                }
                i += 1;
            }
            if !found {
                return -1;
            }
            res += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_shortest_way() {
        assert_eq!(2, Solution::shortest_way("abc", "abcbc"));
        assert_eq!(-1, Solution::shortest_way("abc", "acdbc"));
        assert_eq!(3, Solution::shortest_way("xyz", "xzyxz"));
    }
}
