// leet 179

struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut strs: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
        strs.sort_by(|a, b| {
            let ab = format!("{}{}", a, b);
            let ba = format!("{}{}", b, a);
            ba.cmp(&ab)
        });
        if strs[0] == "0" {
            return "0".to_string();
        }
        strs.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (vec![10, 2], "210"),
            (vec![3, 30, 34, 5, 9], "9534330"),
            (vec![0, 0], "0"),
            (vec![1], "1"),
            (vec![12, 121], "12121"),
            (vec![34323, 3432], "343234323"),
        ];
        for (nums, exp) in cases {
            assert_eq!(Solution::largest_number(nums), exp);
        }
    }
}
