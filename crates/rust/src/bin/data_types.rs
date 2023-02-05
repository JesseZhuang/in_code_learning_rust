/// data types
extern crate in_code_learning_rust;

use in_code_learning_rust::data_type::*;
use in_code_learning_rust::SEPARATOR;

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
    #[allow(dead_code)]
    struct Unit; // unit struct, useful for generics

    enum WebEvent {
        // similar to unit struct
        PageLoad,
        PageUnload,
        KeyPress(char),
        // similar to tuple struct
        Paste(String),
        Click { x: i64, y: i64 }, // similar to c-like struct
    }

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
}
