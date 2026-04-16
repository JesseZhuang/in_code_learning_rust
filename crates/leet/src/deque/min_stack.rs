// lc 155

/// Two stacks approach. O(1) time for all operations, O(n) space.
pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>, // tracks current min at each level
}

impl MinStack {
    pub fn new() -> Self {
        MinStack { stack: Vec::new(), min_stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val); // O(1)
        let cur_min = match self.min_stack.last() {
            Some(&m) => val.min(m),
            None => val,
        };
        self.min_stack.push(cur_min); // O(1)
    }

    pub fn pop(&mut self) {
        self.stack.pop(); // O(1)
        self.min_stack.pop(); // O(1)
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap() // O(1)
    }

    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap() // O(1)
    }
}

/// Single stack with tuples. O(1) time for all operations, O(n) space.
pub struct MinStackV2 {
    stack: Vec<(i32, i32)>, // stores (val, current_min)
}

impl MinStackV2 {
    pub fn new() -> Self {
        MinStackV2 { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        let cur_min = match self.stack.last() {
            Some(&(_, m)) => val.min(m),
            None => val,
        };
        self.stack.push((val, cur_min)); // O(1)
    }

    pub fn pop(&mut self) {
        self.stack.pop(); // O(1)
    }

    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().0 // O(1)
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1 // O(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_stacks_example1() {
        let mut ms = MinStack::new();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        assert_eq!(-3, ms.get_min());
        ms.pop();
        assert_eq!(0, ms.top());
        assert_eq!(-2, ms.get_min());
    }

    #[test]
    fn test_two_stacks_decreasing() {
        let mut ms = MinStack::new();
        ms.push(3);
        ms.push(2);
        ms.push(1);
        assert_eq!(1, ms.get_min());
        ms.pop();
        assert_eq!(2, ms.get_min());
        ms.pop();
        assert_eq!(3, ms.get_min());
    }

    #[test]
    fn test_two_stacks_large_small() {
        let mut ms = MinStack::new();
        ms.push(1000);
        ms.push(-1000);
        assert_eq!(-1000, ms.get_min());
        ms.pop();
        assert_eq!(1000, ms.get_min());
    }

    #[test]
    fn test_single_stack_example1() {
        let mut ms = MinStackV2::new();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        assert_eq!(-3, ms.get_min());
        ms.pop();
        assert_eq!(0, ms.top());
        assert_eq!(-2, ms.get_min());
    }

    #[test]
    fn test_single_stack_decreasing() {
        let mut ms = MinStackV2::new();
        ms.push(3);
        ms.push(2);
        ms.push(1);
        assert_eq!(1, ms.get_min());
        ms.pop();
        assert_eq!(2, ms.get_min());
        ms.pop();
        assert_eq!(3, ms.get_min());
    }
}
