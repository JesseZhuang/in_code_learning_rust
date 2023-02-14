#![allow(dead_code)]

/// data types
extern crate in_code_learning_rust;

use in_code_learning_rust::data_type::*;
use in_code_learning_rust::SEPARATOR;

use crate::List::*;

enum WebEvent {
    // similar to unit struct
    PageLoad,
    PageUnload,
    KeyPress(char),
    // similar to tuple struct
    Paste(String),
    Click { x: i64, y: i64 }, // similar to c-like struct
}

#[derive(Debug)]
enum VeryVerboseEnumThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

enum Status {
    Rich,
    Poor,
}

// enum with implicit discriminator (start at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff_0000,
    Green = 0x00_ff00,
    Blue = 0x00_00ff,
}

enum List {
    Cons(u32, Box<List>),
    // all values in Rust are stack allocated by default
    Nil,
}

impl List {
    fn new() -> List { Nil }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    primitive::primitives();
    // Access constant in the main thread
    println!("{}", SEPARATOR);
    structure::structures();
    println!("{}", SEPARATOR);
    constant::constants();
}

#[cfg(test)]
mod tests {
    use crate::{List, VeryVerboseEnumThingsToDoWithNumbers, WebEvent};

    #[allow(dead_code)]
    struct Unit; // unit struct, useful for generics

    fn event_description(event: WebEvent) -> String {
        match event {
            WebEvent::PageLoad => "a page load event".to_owned(),
            WebEvent::PageUnload => "a page unload event".to_owned(),
            WebEvent::KeyPress(c) => format!("{c} key was pressed"),
            WebEvent::Paste(s) => format!("pasted \"{s}\""),
            WebEvent::Click { x, y } => format!("clicked at {x},{y}"),
        }
    }

    #[test]
    fn test_enum() {
        let events = [
            WebEvent::Paste("test paste".to_owned()),
            WebEvent::Click { x: 23, y: 42 },
            WebEvent::PageLoad,
            WebEvent::PageUnload,
            WebEvent::KeyPress('p')
        ];
        for e in events {
            let es = event_description(e);
            println!("{es}");
        }
    }

    #[test]
    fn test_type_alias() {
        type Ops = VeryVerboseEnumThingsToDoWithNumbers;
        let x = Ops::Add;
        println!("{:?}", x); // prints Add
    }

    #[test]
    fn test_enum_algebra_ops() {
        type Ops = VeryVerboseEnumThingsToDoWithNumbers;
        let (add, subtract) = (Ops::Add, Ops::Subtract);
        assert_eq!(3, add.run(1, 2));
        assert_eq!(0, subtract.run(23, 23));
    }

    #[test]
    fn test_enum_use_avoid_manual_scoping() {
        use crate::Status::{Poor, Rich}; // available without manual scoping
        let status = Poor;
        match status {
            Rich => println!("The rich have lots of money"),
            Poor => println!("The poor have no money"),
        }
    }

    #[test]
    fn test_c_like_enum() {
        use crate::{Number, Color};
        println!("zero is {}", Number::Zero as i32);
        println!("one is {}", Number::One as i32);
        println!("roses are {:06x}", Color::Red as i32);
        let blue = Color::Blue as i32;
        println!("violets are {:06x}, {}", blue, blue);
    }

    #[test]
    fn test_enum_linked_list() {
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("linked list has length: {}", list.len());
        println!("list: {}", list.stringify());
    }
}
