use rand::Rng;
use std::cmp::Ordering;
use std::io;

// In this program we learned about:
// - Using external crates
// - Using the rand crate to generate random numbers
// - Using the std::cmp module to compare two values
// - Using the std::io module to take user input
// - Using the match expression to handle different possibilities
// - Using the Ordering enum to compare values
// - Using the Result type to handle errors
// - Using the expect method to handle the Result type
// - Using the parse method to convert a string into a number
// - Using the shadowing feature to convert a value from one type to another type
// - Using the trim method to eliminate whitespace
// - Using the loop keyword to create an infinite loop
// - Using the break keyword to exit a loop
// - Using the continue keyword to skip the rest of the loop and start a new iteration

fn main() {
    println!("Guess the number game!");
    // Range expressions are inclusive on the lower bound and exclusive on the upper bound
    // so we are requesting a number between 1 and 100
    let secret_number: u32 = rand::thread_rng().gen_range(1..=20);

    loop {
        // Variables in Rust are immutable by default
        // The `mut` keyword allows us to make a variable mutable
        // The :: syntax in the ::new line indicates that new is an associated function of the String types
        let mut guess = String::new();

        println!("Please input your guess.");
        // io is a module that comes from the standard library
        // The :: syntax is used to call an associated function of a type
        // The read_line method takes the user input and appends it to the string we pass it
        // The & indicates that this argument is a reference
        // Result types should be handled, so we use the expect method to handle the Result type
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust allows us to shadow the previous value of guess with a new one
        // This feature is often used in situations in which you want to convert a value from one type to another type
        // The trim method eliminates any whitespace at the beginning and end of the string
        // The parse method is used to parse a string into some kind of number
        // The : u32 after guess tells Rust that we want to parse the string into a u32
        // Then we use a match expression to handle the Result type
        // If parse returns Ok with a value we get the number and continue
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // The match expression is made up of arms
        // .cmp is a method that can be called on anything that can be compared
        // Ordering is an enum with variants Less, Greater, and Equal
        // The match expression is made up of arms
        // An arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm’s pattern
        // If the arm’s pattern matches the value, the code associated with that pattern will be executed
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is: {secret_number}");
                break;
            }
        }
    }
}
