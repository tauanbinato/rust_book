fn main() {
    // A function in rust is defined using the `fn` keyword
    // The function is called with 2 arguments
    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");

    // We can also pass a block of code as an argument to a function
    // The block of code is evaluated first and then the result is passed to the function
    // The block of code is an expression
    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );

    let x;
    if 1 == 1 {
        x = 1
    } else {
        x = 2
    }
    print!("The value of x is: {x}");
}

// All function arguments must have a type annotation
// The return type of the function is also specified
// The return type is specified after the `->` symbol
fn print_labeled_measurement(value: i32, unit_label: char) -> () {
    println!("The measurement is: {value}{unit_label}");
}

// We can return 5 without using the `return` keyword
// The last expression in the function is the return value
// No need to use let to bind the value to a variable because its an expression
fn five() -> i32 {
    5
}

fn f(x: i32) -> i32 {
    x + 1
}
