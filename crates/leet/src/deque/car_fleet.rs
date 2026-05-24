pub struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // sort by position descending, O(n log n)
        let mut res = 0;
        let mut cur: f64 = 0.0;
        for (p, s) in cars { // O(n)
            let time = (target - p) as f64 / s as f64;
            if time > cur {
                cur = time;
                res += 1;
            }
        }
        res // Time O(n log n), Space O(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(3, Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]));
    }

    #[test]
    fn test_example2() {
        assert_eq!(1, Solution::car_fleet(10, vec![3], vec![3]));
    }

    #[test]
    fn test_example3() {
        assert_eq!(1, Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]));
    }

    #[test]
    fn test_all_same_speed() {
        assert_eq!(3, Solution::car_fleet(10, vec![1, 4, 7], vec![2, 2, 2]));
    }

    #[test]
    fn test_single_car() {
        assert_eq!(1, Solution::car_fleet(100, vec![50], vec![10]));
    }
}
