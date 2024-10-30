use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (mut count, n) = (HashMap::new(), nums.len());
        for &v in nums.iter() { *count.entry(v).or_insert(0) += 1; }
        let mut buk = vec![Vec::new(); n + 1];
        for (&v, &cnt) in count.iter() { buk[cnt].push(v); }
        buk.into_iter().flatten().skip(count.len() - k as usize).collect()
    }
}

struct Solution;
