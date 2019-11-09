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
}
