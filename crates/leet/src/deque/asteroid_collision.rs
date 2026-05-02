// lc 735

pub struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        for &a in &asteroids { // O(n)
            let mut alive = true;
            while alive && a < 0 && !stack.is_empty() && *stack.last().unwrap() > 0 { // O(n) total
                let top = *stack.last().unwrap();
                if top < -a {
                    stack.pop();
                } else if top == -a {
                    stack.pop();
                    alive = false;
                } else {
                    alive = false;
                }
            }
            if alive {
                stack.push(a);
            }
        }
        stack // Time O(n), Space O(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(vec![5, 10], Solution::asteroid_collision(vec![5, 10, -5]));
    }

    #[test]
    fn test_example2() {
        assert_eq!(Vec::<i32>::new(), Solution::asteroid_collision(vec![8, -8]));
    }

    #[test]
    fn test_example3() {
        assert_eq!(vec![10], Solution::asteroid_collision(vec![10, 2, -5]));
    }

    #[test]
    fn test_all_positive() {
        assert_eq!(vec![1, 2, 3], Solution::asteroid_collision(vec![1, 2, 3]));
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(
            vec![-3, -2, -1],
            Solution::asteroid_collision(vec![-3, -2, -1])
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(vec![1], Solution::asteroid_collision(vec![1]));
    }

    #[test]
    fn test_large_destroys_many() {
        assert_eq!(vec![-10], Solution::asteroid_collision(vec![1, 2, 3, -10]));
    }

    #[test]
    fn test_no_collision() {
        assert_eq!(
            vec![-2, -1, 1, 2],
            Solution::asteroid_collision(vec![-2, -1, 1, 2])
        );
    }

    #[test]
    fn test_chain() {
        assert_eq!(Vec::<i32>::new(), Solution::asteroid_collision(vec![10, -5, -10]));
    }

    #[test]
    fn test_alternating() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::asteroid_collision(vec![1, -1, 2, -2])
        );
    }

    #[test]
    fn test_surviving_mix() {
        assert_eq!(
            vec![-2, 2],
            Solution::asteroid_collision(vec![-2, 1, -1, 2])
        );
    }
}
