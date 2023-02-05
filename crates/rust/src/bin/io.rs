use std::io;
use std::io::Write;

fn main() {
    println!("Jesse says: Hello, world!");
    print!("Please input your guess: ");
    io::stdout().flush().expect("Failed to flush.");
    let mut guess = String::new();
    // variables and references immutable by default
    io::stdin().read_line(&mut guess).expect("Failed to read line.");
    println!("You guessed: {}", guess);
}
