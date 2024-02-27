fn main() {
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let _none: Option<i32> = plus_one(None);

    match six {
        Some(6) => println!("six"),
        _ => (), // The _ pattern will match any value, so it’s often used in the last match arm to catch all the remaining cases that we don’t want to specify explicitly.
    }

    // or we can use if let
    // we are saying if six matches Some(6), execute the block of code.
    if let Some(6) = six {
        println!("six");
    }
}

// This functions uses a option type to return a value or None.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), // Notice that we wrap the expression i + 1 in Some, because that is the return type of the function.
    }
}
