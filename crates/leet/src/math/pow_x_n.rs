pub struct Solution;

impl Solution {
    /// iterative binary exponentiation. O(lg n) time, O(1) space.
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut n = n as i64; // avoid overflow when negating i32::MIN
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }
        let mut pow = 1.0;
        while n > 0 {
            // O(lg n) iterations
            if n & 1 == 1 {
                pow *= x;
            }
            x *= x;
            n >>= 1;
        }
        pow
    }

    /// recursive binary exponentiation. O(lg n) time and space.
    pub fn my_pow_recursive(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n < 0 {
            return 1.0 / x * Self::my_pow_recursive(1.0 / x, -(n + 1));
            // O(lg n) recursion depth
        }
        if n % 2 == 0 {
            Self::my_pow_recursive(x * x, n / 2)
        } else {
            x * Self::my_pow_recursive(x * x, n / 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn example1() {
        assert!(approx_eq(Solution::my_pow(2.0, 10), 1024.0));
    }

    #[test]
    fn example2() {
        assert!(approx_eq(Solution::my_pow(2.1, 3), 9.261));
    }

    #[test]
    fn example3() {
        assert!(approx_eq(Solution::my_pow(2.0, -2), 0.25));
    }

    #[test]
    fn zero_exp() {
        assert!(approx_eq(Solution::my_pow(2.0, 0), 1.0));
    }

    #[test]
    fn negative_base_even() {
        assert!(approx_eq(Solution::my_pow(-2.0, 4), 16.0));
    }

    #[test]
    fn negative_base_odd() {
        assert!(approx_eq(Solution::my_pow(-2.0, 3), -8.0));
    }

    #[test]
    fn fractional() {
        assert!(approx_eq(Solution::my_pow(0.5, 3), 0.125));
    }

    #[test]
    fn int_min_exp() {
        assert!(approx_eq(Solution::my_pow(1.0, i32::MIN), 1.0));
    }

    #[test]
    fn int_max_exp() {
        assert!(approx_eq(Solution::my_pow(1.0, i32::MAX), 1.0));
    }

    #[test]
    fn recursive_example1() {
        assert!(approx_eq(Solution::my_pow_recursive(2.0, 10), 1024.0));
    }

    #[test]
    fn recursive_example3() {
        assert!(approx_eq(Solution::my_pow_recursive(2.0, -2), 0.25));
    }

    #[test]
    fn recursive_zero() {
        assert!(approx_eq(Solution::my_pow_recursive(2.0, 0), 1.0));
    }
}
