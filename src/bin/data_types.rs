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
