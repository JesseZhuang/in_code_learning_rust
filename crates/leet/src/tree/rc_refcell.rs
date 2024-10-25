// https://stackoverflow.com/questions/61997859/understanding-usage-of-rcrefcellsomestruct-in-rust

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct LinkedList<T> {
    pub head: Pointer<T>,
    pub tail: Pointer<T>,
}

#[derive(Default, Debug)]
pub struct Node<T> {
    pub element: T,
    pub next: Pointer<T>,
    pub prev: Pointer<T>,
}

// we need multiple owners who can mutate the data
// it is Option because "end.next" would be None
type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

#[cfg(test)]
mod tests {
    use crate::tree::rc_refcell::{LinkedList, Node};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_linked_list() {
        let mut dll = LinkedList { head: None, tail: None };
        dll.head = Some(Rc::new(RefCell::new(Node { element: 1, next: None, prev: None })));
        let new_node = Some(Rc::new(RefCell::new(Node { element: 2, next: None, prev: None })));
        dll.head.unwrap().borrow_mut().next = new_node;
        // new_node.unwrap().borrow_mut().prev = dll.head;
    }
}