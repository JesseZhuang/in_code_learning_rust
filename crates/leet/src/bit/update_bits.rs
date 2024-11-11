// lint code 179

pub struct Solution;

impl Solution {
    /// - n: integer n
    /// - m: integer m
    pub fn update_bits(n: i32, m: i32, i: i32, j: i32) -> i32 {
        // or just cast all to u32 and cast back at end
        let mask = if j < 31 {
            !(1i32 << (j + 1)).overflowing_sub(1 << i).0
        } else {
            ((1 << i) as i32).overflowing_sub(1).0
        };
        n & mask | m << i
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_precedence() {
        assert_eq!(1 << 31 - 1, 1 << 30); // bit shift after -1
    }

    #[test]
    fn test_overflow() {
        // 1<<31 i32:MIN, -1 will overflow
        assert_eq!((1 << 31) as u32 - 1, i32::MAX as u32);
    }
}
