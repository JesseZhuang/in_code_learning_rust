use std::collections::HashMap;

/// LeetCode 895 - Maximum Frequency Stack
///
/// Complexity: O(1) push/pop, O(n) space

pub struct FreqStack {
    freq: HashMap<i32, i32>,
    group: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}

impl FreqStack {
    pub fn new() -> Self {
        FreqStack {
            freq: HashMap::new(),
            group: HashMap::new(),
            max_freq: 0,
        }
    }

    pub fn push(&mut self, val: i32) {
        let f = self.freq.entry(val).or_insert(0);
        *f += 1;
        let f = *f;
        if f > self.max_freq {
            self.max_freq = f;
        }
        self.group.entry(f).or_insert_with(Vec::new).push(val);
    }

    pub fn pop(&mut self) -> i32 {
        let vals = self.group.get_mut(&self.max_freq).unwrap();
        let val = vals.pop().unwrap();
        if vals.is_empty() {
            self.group.remove(&self.max_freq);
            self.max_freq -= 1;
        }
        *self.freq.get_mut(&val).unwrap() -= 1;
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example() {
        let mut fs = FreqStack::new();
        for val in [5, 7, 5, 7, 4, 5] {
            fs.push(val);
        }
        assert_eq!(fs.pop(), 5);
        assert_eq!(fs.pop(), 7);
        assert_eq!(fs.pop(), 5);
        assert_eq!(fs.pop(), 4);
    }

    #[test]
    fn test_single_element() {
        let mut fs = FreqStack::new();
        fs.push(42);
        assert_eq!(fs.pop(), 42);
    }

    #[test]
    fn test_all_same() {
        let mut fs = FreqStack::new();
        fs.push(3);
        fs.push(3);
        fs.push(3);
        assert_eq!(fs.pop(), 3);
        assert_eq!(fs.pop(), 3);
        assert_eq!(fs.pop(), 3);
    }

    #[test]
    fn test_interleaved() {
        let mut fs = FreqStack::new();
        fs.push(1);
        fs.push(2);
        fs.push(1);
        assert_eq!(fs.pop(), 1); // freq 2
        fs.push(2);
        assert_eq!(fs.pop(), 2); // freq 2, most recent
        assert_eq!(fs.pop(), 2); // freq 1, most recent
        assert_eq!(fs.pop(), 1); // freq 1
    }
}
