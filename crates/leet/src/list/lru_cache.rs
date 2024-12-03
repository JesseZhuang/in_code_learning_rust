// lc 146

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type NodePtr = Rc<RefCell<Node>>;
type WeakNodePtr = Weak<RefCell<Node>>;

struct Node {
    key: i32,
    value: i32,
    /// previous will be a weak pointer to avoid cyclic references.
    prev: Option<WeakNodePtr>,
    /// next is a strong pointer
    next: Option<NodePtr>,
}

impl Node {
    #[inline]
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, NodePtr>,
    head: Option<NodePtr>,
    tail: Option<NodePtr>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * todo @mimanshu-maheshwari
 */
impl LRUCache {
    /// initialize lru cache
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        // if node is present in map then return the value
        if let Some(node) = self.map.get(&key) {
            // we need to clone node as we will use it mutably in the methods.
            let node = Rc::clone(node);
            // bring node to front as it is accessed
            self.remove(&node);
            self.push_front(&node);
            // extract the value and return it.
            let value = node.borrow().value;
            value
        } else {
            // if node is not present then return -1
            // condition in question
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // check if map has key
        // if so then we can update it.
        match self.map.get(&key) {
            Some(node) => {
                // copy the node
                let node = Rc::clone(&node);
                // update value
                node.borrow_mut().value = value;
                // move node to front
                self.remove(&node);
                self.push_front(&node);
            }
            // if node is not present
            None => {
                // create a new node
                let node = Rc::new(RefCell::new(Node::new(key, value)));
                // add it to linked list
                self.push_front(&node);
                // add it to map
                self.map.insert(key, node);
                // evict if necessary
                if self.map.len() > self.capacity {
                    if let Some(evicted_node) = self.pop_back() {
                        // also remember to remove it from map.
                        self.map.remove(&evicted_node.borrow().key);
                    }
                }
            }
        }
    }

    /// this method is responsible for moving pushing node to front
    fn push_front(&mut self, node: &NodePtr) {
        // if head is present then
        match self.head.take() {
            Some(head) => {
                // set the prev of head to node
                head.borrow_mut().prev = Some(Rc::downgrade(&node));
                // set the next of node as head
                node.borrow_mut().next = Some(Rc::clone(&head));
                // update the prev of node to none
                node.borrow_mut().prev = None;
                // update the head in list to node
                self.head = Some(Rc::clone(&node));
            }
            // else if head is not present then
            None => {
                // add the node as head and tail
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
    }

    /// this method is responsible for removing the node from list.
    fn remove(&mut self, node: &NodePtr) {
        // we will match the node on prev and next
        match (node.clone().borrow().prev.as_ref(), node.clone().borrow().next.as_ref()) {
            // meaning only one node so we can remove head
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            // the node is a tail node
            (Some(prev), None) => {
                if let Some(prev) = prev.upgrade() {
                    self.tail = Some(Rc::clone(&prev));
                    prev.borrow_mut().next = None;
                }
            }
            // node is a head node
            (None, Some(next)) => {
                self.head = Some(Rc::clone(&next));
                next.borrow_mut().prev = None;
            }
            // node is in middle
            (Some(prev), Some(next)) => {
                if let Some(prev) = prev.upgrade() {
                    prev.borrow_mut().next = Some(Rc::clone(&next));
                }
                next.borrow_mut().prev = Some(prev.clone());
            }
        }
    }

    /// this will pop the tail of linked list
    fn pop_back(&mut self) -> Option<NodePtr> {
        match self.tail.take() {
            Some(tail) => {
                self.remove(&tail);
                Some(tail)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Default)]
    struct Node {
        key: i32,
        value: i32,
        prev: Option<Rc<RefCell<Node>>>,
        next: Option<Rc<RefCell<Node>>>,
    }

    #[test]
    fn test_option_take() {
        let head = Some(Rc::new(RefCell::new(Node::default())));
        let tail = Some(Rc::new(RefCell::new(Node::default())));
        assert_eq!(1, Rc::strong_count(head.as_ref().unwrap())); // head var
        assert_eq!(1, Rc::strong_count(tail.as_ref().unwrap())); // tail var
        head.as_ref().unwrap().borrow_mut().next = tail.clone();
        assert_eq!(1, Rc::strong_count(head.as_ref().unwrap()));
        assert_eq!(2, Rc::strong_count(tail.as_ref().unwrap())); // tail, head.next
        tail.as_ref().unwrap().borrow_mut().prev = head.clone();
        assert_eq!(2, Rc::strong_count(head.as_ref().unwrap()));
        assert_eq!(2, Rc::strong_count(tail.as_ref().unwrap()));
        // take ownership, leave None on the place
        head.as_ref().unwrap().borrow_mut().next.take();
        assert_eq!(2, Rc::strong_count(head.as_ref().unwrap())); // head, tail.prev
        assert_eq!(1, Rc::strong_count(tail.as_ref().unwrap())); // tail
    }
}
