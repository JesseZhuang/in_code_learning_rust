extern crate in_code_learning_rust;

use in_code_learning_rust::SEPARATOR;

//fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
//fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.
// Implementing the fmt::Display trait automatically implements the ToString trait.
fn main() {
    // Arguments will be stringified.
    // Without a suffix, 31 becomes an i32. Change by providing a suffix.
    println!("{} days", 31);
    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");
    // named arguments
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");
    println!("Base 10:               {}", 69420); //69420
    println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
    println!("Base 8 (octal):      0o{:o}", 69420); //207454
    println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C
    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);
    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);
    // Rust checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}.", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"
    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);
    // Custom types require more complicated handling.
    // the trait `std::fmt::Display` is not implemented for `main::Structure`
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
    let pi = 3.1415926;
    println!("{pi} with 3 digits is {pi:.3}", pi = pi);
    println!("{pi} with 10 min width right aligned is {pi:0>10}", pi = pi); // 10 min width
    println!("{}", SEPARATOR);
    debug_print();
}

fn debug_print() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("print string {:?}.", "string");
    println!("print string {}.", "string");
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor = "actor's");
    println!("Now {:?} will print!", DebugPrintable(3));
    // The problem with `derive` is there is no control over how the results look.
    // What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(DebugPrintable(7)));
}

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);
