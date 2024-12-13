use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// leet 1650, lint 474
///
/// Rc (Reference Counted): Used for shared ownership of the TreeNode instances.
/// It allows multiple owners of the same data.
/// Weak reference to the parent avoids circular reference.
///
/// RefCell: Provides interior mutability, enabling you to modify the data even when you
/// only have a shared reference to it.
///
/// Option: Used to represent the possibility of a node being None

type NodePtr = Rc<RefCell<Node>>;
type WeakNodePtr = Weak<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    val: i32,
    parent: Option<WeakNodePtr>,
    left: Option<NodePtr>,
    right: Option<NodePtr>,
}

impl Node {
    /// constructs a Rc<RefCell>> pointer to a new point with val
    #[inline]
    pub fn new_ptr(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            parent: None,
            left: None,
            right: None,
        }))
    }

    pub fn connect(root: NodePtr, left: NodePtr, right: NodePtr) {
        let mut r_m = root.borrow_mut();
        (r_m.left, r_m.right) = (Some(left.clone()), Some(right.clone()));
        left.borrow_mut().parent = Some(Rc::downgrade(&root));
        right.borrow_mut().parent = Some(Rc::downgrade(&root));
    }
}

struct Solution;

impl Solution {
    pub fn lca(p: Option<NodePtr>, q: Option<NodePtr>) -> Option<NodePtr> {
        let (p, q) = (p?, q?);
        let (mut a, mut b) = (p.clone(), q.clone());
        while a.borrow().val != b.borrow().val {
            let (ap, bp) = (a.borrow().parent.clone(), b.borrow().parent.clone());
            a = if ap.is_none() { q.clone() } else { ap?.upgrade()? };
            b = if bp.is_none() { p.clone() } else { bp?.upgrade()? };
        }
        Some(a)
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::lowest_common_ancestor_bt_iii::{Node, Solution};
    use std::rc::Rc;

    #[test]
    fn test_lca() {
        // {4,3,7,#,#,5,6},3,5
        let root = Node::new_ptr(4); // root 4
        // first layer: 3,7
        let (left, right) = (Node::new_ptr(3), Node::new_ptr(7));
        Node::connect(root.clone(), left.clone(), right.clone());
        // second layer under 7: 5,6
        let p = left.clone();
        let r_2 = right.clone();
        let (left, right) = (Node::new_ptr(5), Node::new_ptr(6));
        let q = left.clone();
        Node::connect(r_2.clone(), left.clone(), right.clone());
        let res = Solution::lca(Some(p), Some(q));
        println!("{:#?}", res);
        assert_eq!(root.borrow().val, res.unwrap().borrow().val);
    }

    #[test]
    fn test_option_eq() {
        let (a, b) = (2, 2);
        assert_eq!(Some(a), Some(b));
    }

    #[test]
    fn test_rc_eq() {
        let five = Rc::new(5);
        let same_five = five.clone(); // Rc::clone(&five);
        let other_five = Rc::new(5);
        // five and same_five reference the same value in memory
        assert!(Rc::ptr_eq(&five, &same_five));
        // five and other_five does not reference the same value in memory
        assert!(!Rc::ptr_eq(&five, &other_five));
    }

    #[test]
    fn test_tree_node() {
        let n_o = Some(Node::new_ptr(0)); // wrapped in option
        println!("{n_o:#?}");
        // Some(
        //     RefCell {
        //         value: Node {
        //             val: 0,
        //             parent: None,
        //             left: None,
        //             right: None,
        //         },
        //     },
        // )
        let (left, right) = (Node::new_ptr(-1), Node::new_ptr(1)); // not wrapped
        let n = n_o.unwrap(); // node moved to var n, n_o can no longer be used
        // let mut n_mut = n.borrow_mut();
        n.borrow_mut().left = Some(left.clone());
        n.borrow_mut().right = Some(right.clone());
        left.borrow_mut().parent = Some(Rc::downgrade(&n));
        right.borrow_mut().parent = Some(Rc::downgrade(&n));
        println!("{n:#?}");
        // RefCell {
        //     value: Node {
        //         val: 0,
        //         parent: None,
        //         left: Some(
        //             RefCell {
        //                 value: Node {
        //                     val: -1,
        //                     parent: Some(
        //                         (Weak),
        //                     ),
        //                     left: None,
        //                     right: None,
        //                 },
        //             },
        //         ),
        //         right: Some(
        //             RefCell {
        //                 value: Node {
        //                     val: 1,
        //                     parent: Some(
        //                         (Weak),
        //                     ),
        //                     left: None,
        //                     right: None,
        //                 },
        //             },
        //         ),
        //     },
        // }
        println!("{:#?}", left.borrow().parent.as_ref()
            .expect("parent is not None").upgrade());
        println!("{:#?}", left.borrow().parent);
        // Some(
        //     (Weak),
        // )
        // borrow left interior RefCell second time, multiple immutable borrows
        println!("{:#?}", left.borrow().parent.as_ref()
            .expect("parent is not None").upgrade());
        // Some(
        //     RefCell {
        //         value: Node {
        //             val: 0,
        //             parent: None,
        //             left: Some(
        //                 RefCell {
        //                     value: Node {
        //                         val: -1,
        //                         parent: Some(
        //                             (Weak),
        //                         ),
        //                         left: None,
        //                         right: None,
        //                     },
        //                 },
        //             ),
        //             right: Some(
        //                 RefCell {
        //                     value: Node {
        //                         val: 1,
        //                         parent: Some(
        //                             (Weak),
        //                         ),
        //                         left: None,
        //                         right: None,
        //                     },
        //                 },
        //             ),
        //         },
        //     },
        // )
    }

    #[test]
    fn test_tree_node_eq() {
        // Weak does not support derive PartialEq, compare node val instead
        let (a, b) = (1, 1);
        assert_eq!(a, b);
        println!("address {:p} != {:p}", &a, &b);
        // address 0x16d312684 != 0x16d312688
        let n = Node::new_ptr(0);
        let (mut n1, mut n2) = (Some(n.clone()), Some(n.clone()));
        assert_eq!(3, Rc::strong_count(&n)); // 3 refs: n,n1,n2
        let n2n = n2.unwrap();
        assert_eq!(n1.unwrap().borrow().val, n2n.borrow().val);
        n1 = Some(Node::new_ptr(1));
        assert_ne!(n1.as_ref().expect("").borrow().val, n2n.borrow().val);
        n2 = Some(Node::new_ptr(1));
        // unwraps moves content in option, as_ref then expect does not
        assert_eq!(n1.as_ref().expect("").borrow().val, n2.unwrap().borrow().val);
    }

    #[test]
    fn test_borrow_node() {
        let n = Node::new_ptr(0);
        assert_eq!(n.borrow().val, 0); // immutably borrows the node in Rc<RefCell<Node>>
    }
}
