/// leet 811

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Hash map. O(nm) time, O(nm) space.
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut cnt: HashMap<&str, i32> = HashMap::new();
        for cd in &cpdomains {
            let sp = cd.find(' ').unwrap();
            let n: i32 = cd[..sp].parse().unwrap();
            let domain = &cd[sp + 1..];
            *cnt.entry(domain).or_insert(0) += n;
            for (i, c) in domain.char_indices() {
                if c == '.' {
                    *cnt.entry(&domain[i + 1..]).or_insert(0) += n;
                }
            }
        }
        cnt.into_iter()
            .map(|(d, c)| format!("{} {}", c, d))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    #[test]
    fn example1() {
        let input = vec!["9001 discuss.leetcode.com".to_string()];
        let res: HashSet<String> = Solution::subdomain_visits(input).into_iter().collect();
        let expected: HashSet<String> = vec![
            "9001 leetcode.com".to_string(),
            "9001 discuss.leetcode.com".to_string(),
            "9001 com".to_string(),
        ]
        .into_iter()
        .collect();
        assert_eq!(res, expected);
    }

    #[test]
    fn example2() {
        let input = vec![
            "900 google.mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "1 intel.mail.com".to_string(),
            "5 wiki.org".to_string(),
        ];
        let res: HashSet<String> = Solution::subdomain_visits(input).into_iter().collect();
        let expected: HashSet<String> = vec![
            "901 mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "900 google.mail.com".to_string(),
            "5 wiki.org".to_string(),
            "5 org".to_string(),
            "1 intel.mail.com".to_string(),
            "951 com".to_string(),
        ]
        .into_iter()
        .collect();
        assert_eq!(res, expected);
    }
}
