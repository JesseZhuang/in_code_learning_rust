use std::fmt;
use std::fmt::Formatter;

/// structure data type: c struct
#[allow(dead_code)] // field of Person never used
#[derive(Debug)] // creates implementation, printable with fmt::Debug
struct Person<'a> {
    name: &'a str,
    age: u8,
}


pub fn structures() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // Print debug struct
    println!("{:?}", peter);
    // pretty print
    println!("{:#?}", peter);
}

struct Structure(i32);

// Implementing the fmt::Display trait automatically implements the ToString trait.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "structure containing i32 number: {}", self.0) // no semicolon, return the result
    }
}

struct IntList(Vec<i32>);

impl fmt::Display for IntList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, num) in (&self.0).iter().enumerate() {
            if i != 0 { write!(f, ", ")?; }
            write!(f, "{i}: {num}")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::structure::{IntList, Structure};

    #[test]
    fn test_to_string() {
        let n = Structure(3);
        println!("user defined to string: {}", n);
    }

    #[test]
    fn test_int_list_to_string() {
        println!("{}", IntList(vec![1, 2, 3]));
    }
}