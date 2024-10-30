// lc 146

use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

type Pointer<Node> = Option<Rc<RefCell<Node>>>;

struct LRUCache {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    lru: DLinkedList,
    cap: usize,
}

#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: Pointer<Node>,
    next: Pointer<Node>,
}

impl Node {
    pub fn new(key: i32, value: i32) -> Self {
        Self { key, value, prev: None, next: None }
    }
}

#[derive(Debug)]
struct DLinkedList {
    head: Pointer<Node>,
    tail: Pointer<Node>,
}


impl DLinkedList {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    fn get_head(&self) -> Pointer<Node> {
        if self.head.is_none() {
            None
        } else {
            Some(self.head.as_ref()?.clone()) // as_ref &Option<T> -> Option<&T>
        }
    }

    fn get_tail(&self) -> Pointer<Node> {
        if self.tail.is_none() {
            None
        } else {
            Some(self.tail.as_ref()?.clone())
        }
    }

    // pub fn add_front(&mut self, key: i32, value: i32) {
    //     let node = Rc::new(RefCell::new(Node {
    //         key,
    //         value,
    //         prev: None,
    //         next: self.get_head(),
    //     }));
    //     self.head.replace(node);
    // }
    //
    // pub fn add_back(&mut self, key: i32, value: i32) {
    //     let node = Rc::new(RefCell::new(Node {
    //         key,
    //         value,
    //         prev: self.get_tail(),
    //         next: None,
    //     }));
    //     self.tail.replace(node);
    // }

    pub fn add_front_node(&mut self, node: Rc<RefCell<Node>>) {
        let head = self.get_head();
        if head.is_some() {
            head.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
        }
        node.borrow_mut().prev = None;
        node.borrow_mut().next = head; // in front of head
        self.head = Some(node);
    }

    pub fn add_back_node(&mut self, node: Rc<RefCell<Node>>) {
        let tail = self.get_tail();
        if tail.is_some() {
            tail.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        }
        node.borrow_mut().prev = tail;
        node.borrow_mut().next = None;
        self.tail = Some(node);
    }

    pub fn remove(&mut self, target: Rc<RefCell<Node>>) {
        let prev = target.borrow().prev.clone();
        let next = target.borrow().next.clone();
        match (prev, next) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone()); // only clone for next
                next.borrow_mut().prev = Some(prev); // no clone
            }
            (Some(prev), None) => {
                // tail case
                prev.borrow_mut().next.take(); // take ownership, leave None on the place
                self.tail.replace(prev);
            }
            (None, Some(next)) => {
                // head case
                next.borrow_mut().prev.take();
                self.head.replace(next);
            }
            (None, None) => {
                // signal node case
                self.head.take();
                self.tail.take();
            }
        }
    }

    pub fn move_head(&mut self, target: Rc<RefCell<Node>>) {
        if !Rc::ptr_eq(self.get_head().as_ref().unwrap(), &target) {
            self.remove(target.clone());
            self.add_front_node(target);
        }
    }

    // pub fn move_tail(&mut self, target: Rc<RefCell<Node>>) {
    //     if !Rc::ptr_eq(self.get_tail().as_ref().unwrap(), &target) {
    //         self.remove(target.clone());
    //         self.add_back_node(target);
    //     }
    // }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            lru: DLinkedList::new(),
            cap: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            let node = self.map.get(&key).unwrap();
            self.lru.move_head(node.clone());
            node.as_ref().borrow().value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let node = if self.map.contains_key(&key) {
            let node = self.map.get(&key).unwrap();
            node.borrow_mut().value = value;
            self.lru.remove(node.clone());
            self.lru.add_front_node(node.clone());
            node.clone()
        } else {
            let node = Rc::new(RefCell::new(Node::new(key, value)));
            if self.map.len() == self.cap {
                let tail = self.lru.get_tail().as_ref().unwrap().clone();
                self.map.remove(&tail.as_ref().borrow().key);
                self.lru.remove(tail);

                self.map.insert(key, node.clone());
                self.lru.add_front_node(node.clone());
            } else {
                self.map.insert(key, node.clone());
                self.lru.add_front_node(node.clone());
            }
            node
        };
        if self.lru.tail.is_none() {
            self.lru.add_back_node(node);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list::lru_cache::Node;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_option_take() {
        let head = Some(Rc::new(RefCell::new(Node::new(1, 2))));
        let tail = Some(Rc::new(RefCell::new(Node::new(2, 1))));
        assert_eq!(1, Rc::strong_count(head.as_ref().unwrap())); // head var
        assert_eq!(1, Rc::strong_count(tail.as_ref().unwrap())); // tail var
        head.as_ref().unwrap().borrow_mut().next = tail.clone();
        assert_eq!(1, Rc::strong_count(head.as_ref().unwrap()));
        assert_eq!(2, Rc::strong_count(tail.as_ref().unwrap())); // tail, head.next
        tail.as_ref().unwrap().borrow_mut().prev = head.clone();
        assert_eq!(2, Rc::strong_count(head.as_ref().unwrap()));
        assert_eq!(2, Rc::strong_count(tail.as_ref().unwrap()));
        head.as_ref().unwrap().borrow_mut().next.take();
        assert_eq!(2, Rc::strong_count(head.as_ref().unwrap())); // head, tail.prev
        assert_eq!(1, Rc::strong_count(tail.as_ref().unwrap())); // tail
    }
}