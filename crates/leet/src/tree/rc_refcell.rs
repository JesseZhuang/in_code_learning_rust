// https://stackoverflow.com/questions/61997859/understanding-usage-of-rcrefcellsomestruct-in-rust
// https://gist.github.com/matey-jack/3e19b6370c6f7036a9119b79a82098ca

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

// node pointer
type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Default, Debug)]
pub struct Node<T: Default> {
    pub val: T,
    pub prev: Option<Weak<RefCell<Node<T>>>>, // does not own pointer to prev
    pub next: Pointer<T>, // owns pointer to next
}

impl<T> Node<T>
where
    T: Default,
{
    pub fn new(val: T) -> Self {
        Self { val, prev: None, next: None }
    }

    pub fn add_last(node: &mut Rc<RefCell<Node<T>>>, val: T) -> Option<Rc<RefCell<Node<T>>>> {
        if node.borrow().next.is_none() {
            // If the current node is the tail, create a new node,
            // set its prev pointer to the current node, and store it as
            // the node after the current one.
            let mut new_node = Node::new(val);
            new_node.prev = Some(Rc::downgrade(&node));
            let rc = Rc::new(RefCell::new(new_node));
            node.borrow_mut().next = Some(rc.clone());
            Some(rc)
        } else {
            // Not the tail node, just continue traversing the list:
            if let Some(ref mut next) = node.borrow_mut().next {
                Self::add_last(next, val)
            } else { None }
        }
    }
}

#[derive(Default, Debug)]
struct DLinkedList<T: Default> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

impl<T> DLinkedList<T>
where
    T: Default,
{
    // Appends a new node to the list, handling the case where the list is empty.
    pub fn append(&mut self, val: T) {
        if let Some(ref mut next) = self.tail {
            self.tail = Node::add_last(next, val);
        } else {
            let n = Rc::new(RefCell::new(Node::new(val)));
            self.head = Some(n.clone());
            self.tail = Some(n); // moves f
        }
    }
}

// Pretty-printing
impl<T: Display> Display for DLinkedList<T>
where
    T: Default,
{
    fn fmt(&self, w: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().val)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use crate::tree::rc_refcell::DLinkedList;
    use crate::tree::rc_refcell::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_linked_list1() {
        let value = Rc::new(RefCell::new(5));
        assert_eq!(5, *value.borrow());
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("b before = {b:?}");
        *value.borrow_mut() += 10;
        println!("a after = {a:?}");
        println!("b after = {b:?}");
        println!("c after = {c:?}");
        assert_eq!(15, *value.borrow());
        assert_eq!(3, Rc::strong_count(&a)); // a, b, c
    }

    #[test]
    fn test_linked_list2() {
        let mut list = DLinkedList::default();
        println!("{}", list);
        for i in 0..10 {
            list.append(i);
        }
        println!("{}", list);
    }
}
