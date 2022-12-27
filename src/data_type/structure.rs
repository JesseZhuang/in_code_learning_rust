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
    let peter = Person { name, age};
    // Print debug struct
    println!("{:?}", peter);
    // pretty print
    println!("{:#?}", peter);
}
