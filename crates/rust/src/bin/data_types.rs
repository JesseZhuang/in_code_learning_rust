/// data types
extern crate in_code_learning_rust;

use in_code_learning_rust::data_type::*;
use in_code_learning_rust::SEPARATOR;

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
    use crate::{VeryVerboseEnumThingsToDoWithNumbers, WebEvent};

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
    fn test_algebra_ops() {
        type Ops = VeryVerboseEnumThingsToDoWithNumbers;
        let (add, subtract) = (Ops::Add, Ops::Subtract);
        assert_eq!(3, add.run(1, 2));
        assert_eq!(0, subtract.run(23, 23));
    }
}
